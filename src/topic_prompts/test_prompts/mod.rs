use crate::topic_prompts::TopicPrompt;
use crate::MultiShotQuestionsAndAnswers;

/// This is an example of implementing `TopicPrompt`. This is for prompting to
/// fix non running Rust code.
///
/// # Usage
/// ```no_run
/// use rust_llm_utils::OpenAiClient;
///
/// // with this `TopicPrompt` we are supposed to only pass in code
/// let rust_code_to_fix = r#"fn some_func() -> String {\"abc\"}"#.to_string();
///
/// // we construct the
/// let topic_prompt = FixRustCode::new(rust_code_to_fix);
///
/// // we create the client with defaults
/// let open_ai_client = OpenAiClient::new(None, None);
///
/// // then we make the call with the prompt
/// let simplified_response_result = open_ai_client.perform_request(&topic_prompt).await;
/// ```
///
/// the above `simplified_response_result.answer` would contain a sting:
///
/// ````
/// Certainly! The issue with the code you provided is that the function `some_func`
/// is declared to return a `String`, but it is not actually returning a `String`
/// value. To fix this, you can modify the code as follows:
///
/// ```
/// fn some_func() -> String {
///    String::from("abc")
/// }
/// ```
///
/// In this updated code, the `String::from` function is used to create a new
/// String` object with the value "abc", which is then returned by the function.
/// ````
/// But you should define these, so that they are specific to your use case.
pub struct WeatherInTwoLanguages {
    query: String,
}

impl TopicPrompt for WeatherInTwoLanguages {
    fn query(&self) -> String {
        self.query.clone()
    }

    fn new_from_prompt_template(weather_statement: String) -> Self {
        let augmented_query = format!(
            r#"Could you generate a statement about the weather based on the weather condition statement in similar style as my examples above.
{weather_statement}
"#
        );

        Self {
            query: augmented_query,
        }
    }
}

pub struct MultiShotQuestionsAndAnswersWeatherInTwoLanguages;

impl MultiShotQuestionsAndAnswers for MultiShotQuestionsAndAnswersWeatherInTwoLanguages {
    fn multi_shot_questions_and_answers(&self) -> [String; 5] {
        let qa_1 =
            r#"prompt: it is 25c and sunny in Berlin, answer: it seems like summer weather. Es sieht aus wie Sommerwetter ☀️
"#.to_string();
        let qa_2 =
			r#"prompt: it is 8c and rainy in Berlin, answer: it seems like autumn weather. Es sieht aus wie Herbstwetter 🍂
"#.to_string();
        let qa_3 =
			r#"prompt: it is 11c and sunny in Berlin, answer: it seems like autumn weather. Es sieht aus wie Herbstwetter 🍂
"#.to_string();
        let qa_4 =
			r#"prompt: it is -2c and sunny in Berlin, answer: it seems like winter weather. Es sieht aus wie Winterwetter ❄️
"#.to_string();
        let qa_5 =
			r#"prompt: it is -4c and sunny in Berlin, answer: it seems like winter weather. Es sieht aus wie Winterwetter ❄️
"#.to_string();
        [qa_1, qa_2, qa_3, qa_4, qa_5]
    }
}
