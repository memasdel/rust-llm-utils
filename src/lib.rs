mod open_ai_api;
mod prompt_types;
mod topic_prompts;

pub use open_ai_api::{OpenAiClient, OpenAiCompletionsResponseBody, OpenAiSimplifiedResponse};
pub use topic_prompts::{programming, test_prompts, TopicPrompt};

pub use prompt_types::multi_shot_prompt::{
    MultiShot, MultiShotExampleCount, MultiShotQuestionsAndAnswers,
};
pub use prompt_types::zero_shot_prompt::ZeroShot;
