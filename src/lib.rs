mod open_ai_api;
mod topic_prompts;

pub use open_ai_api::{OpenAiClient, OpenAiCompletionsResponseBody, OpenAiSimplifiedResponse};

pub use topic_prompts::{programming, TopicPrompt};
