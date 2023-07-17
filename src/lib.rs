mod open_ai_api;
mod prompt_types;
mod topic_prompts;

pub use open_ai_api::{OpenAiClient, OpenAiCompletionsResponseBody, OpenAiSimplifiedResponse};
pub use prompt_types::{MultiShotExampleCount, MultiShotPrompt, MultiShotQuestionsAndAnswers};
pub use prompt_types::{PromptType, ZeroShotPrompt};
pub use topic_prompts::{programming, test_prompts, TopicPrompt};
