use crate::topic_prompts::TopicPrompt;
use dotenv::dotenv;
use hyper::body::to_bytes;
use hyper::Client;
use hyper::{Body, Method, Request};
use hyper_tls::HttpsConnector;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env::var;

// OpenAI token from .env, panics if not found
lazy_static! {
    static ref OPEN_AI_TOKEN: String =
        var("OPEN_AI_TOKEN").expect("could not find OPEN_AI_TOKEN environment variable");
}

#[derive(Deserialize, Debug)]
pub struct OpenAiCompletionsResponseBody {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
pub struct OpenAiSimplifiedResponse {
    pub answer: Option<String>,
    pub follow_up_query: Option<String>,
}

impl TryFrom<OpenAiCompletionsResponseBody> for OpenAiSimplifiedResponse {
    type Error = String;
    fn try_from(value: OpenAiCompletionsResponseBody) -> Result<Self, Self::Error> {
        let message_content = &value
            .choices
            .first()
            .ok_or_else(|| "failed to get the choice from response".to_string())?
            .message
            .content;

        Ok(Self {
            answer: Some(message_content.clone()),
            follow_up_query: None,
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub index: u64,
    pub message: Message,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Copy)]
pub enum OpenAiModel {
    /// https://platform.openai.com/docs/models/gpt-3-5
    Gpt35_16k,

    /// https://platform.openai.com/docs/models/gpt-4
    Gpt40_32k,
}

impl Serialize for OpenAiModel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.value().as_str())
    }
}

impl OpenAiModel {
    fn value(&self) -> String {
        match self {
            Self::Gpt35_16k => "gpt-3.5-turbo-16k".to_string(),
            Self::Gpt40_32k => {
                panic!("gpt 4.0 is not available yet through the API even thought it was announced in the first week of July")
            }
        }
    }
}

#[derive(Serialize, Debug)]
struct Prompt {
    /// open ai model, use gpt3.5 or gpt4.0
    model: OpenAiModel,

    /// structure holding our query
    messages: Vec<Message>,

    /// verbosity of the answer, 0.1 is dry, 1 is very chatty
    temperature: f32,
}

/// panics if the OPEN_AI_TOKEN is not in the .env file
fn read_open_ai_token_from_dot_env_file() -> &'static str {
    // read .env file to env vars
    dotenv().ok();
    &OPEN_AI_TOKEN
}

pub struct OpenAiClient<'a> {
    model: OpenAiModel,
    token: &'a str,
}

impl<'a> OpenAiClient<'a> {
    pub fn new(
        model_override_maybe: Option<OpenAiModel>,
        token_override_maybe: Option<&'a str>,
    ) -> Self {
        // default or override model
        let model = if let Some(model_override) = model_override_maybe {
            model_override
        } else {
            OpenAiModel::Gpt35_16k
        };

        // default or override token
        let token = if let Some(token_override) = token_override_maybe {
            token_override
        } else {
            // read .env file to env vars
            read_open_ai_token_from_dot_env_file()
        };

        Self { model, token }
    }

    // TODO: rename to reflect the fact that this creates a Model specific prompt
    pub async fn perform_request(&self, prompt: &str) -> Result<OpenAiSimplifiedResponse, String> {
        // pub async fn perform_request(
        //     &self,
        //     topic_prompt: &impl TopicPrompt,
        // ) -> Result<OpenAiSimplifiedResponse, String> {
        // generate a prompt that can be sent to OpenAI
        let prompt = self.generate_prompt(prompt);

        // call OpenAI
        let open_ai_completions_response_body = self
            .call_open_ai(prompt)
            .await
            .map_err(|error| format!("error while calling OpenAI: {error}"))?;

        let simplified_response: OpenAiSimplifiedResponse =
            open_ai_completions_response_body.try_into()?;

        // return body, which contains the response to our prompt with a few
        // other things
        Ok(simplified_response)
    }

    /// part of the high level flow, performing the actual call
    pub async fn call_open_ai(
        &self,
        prompt: String,
    ) -> Result<OpenAiCompletionsResponseBody, String> {
        let https_builder = HttpsConnector::new();
        let client = Client::builder().build(https_builder);

        let token = &self.token;

        let request = Request::builder()
            .uri("https://api.openai.com/v1/chat/completions")
            .method(Method::POST)
            .header("content-type", "application/json")
            .header("Authorization", format!("Bearer {token}"))
            .body(Body::from(prompt))
            .map_err(|error| error.to_string())?;

        let mut resp = client.request(request).await.unwrap();
        let body = to_bytes(resp.body_mut()).await.unwrap();

        let parsed_body: OpenAiCompletionsResponseBody =
            serde_json::from_str(std::str::from_utf8(body.as_ref()).unwrap()).unwrap();

        Ok(parsed_body)
    }

    // TODO: rename to reflect the fact that this creates a Model specific prompt
    /// returns a ready prompt request that can be posted to OpenAI's API
    pub fn generate_prompt(&self, prompt: &str) -> String {
        // pub fn generate_prompt(&self, topic_prompt: &impl TopicPrompt) -> String {
        // let query = topic_prompt.query();
        let escaped_query = prompt.replace("\n", "\\n");

        let prompt = Prompt {
            messages: vec![Message {
                content: escaped_query,
                role: "user".to_string(),
            }],
            model: self.model,
            temperature: 0.01,
        };

        serde_json::to_string(&prompt).unwrap()
    }
}
