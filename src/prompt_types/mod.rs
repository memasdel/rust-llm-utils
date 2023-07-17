pub mod multi_shot_prompt;
pub mod zero_shot_prompt;

pub use multi_shot_prompt::{MultiShotExampleCount, MultiShotPrompt, MultiShotQuestionsAndAnswers};
pub use zero_shot_prompt::ZeroShotPrompt;

pub enum PromptType {
    ZeroShotPrompt(ZeroShotPrompt),
    MultiShotPrompt(MultiShotPrompt),
}

impl PromptType {
    /// constructs a new multi shot prompt from the inputs
    /// * `inner_prompt` should have been generaged from a template
    /// * `multi_shot_questions_and_answers` should have been implemented for a
    ///   topic prompt
    /// * `multi_shot_example_count`indicates how many of the example QA will be used
    pub fn new_multi_shot_prompt(
        inner_prompt: String,
        multi_shot_questions_and_answers: impl MultiShotQuestionsAndAnswers,
        multi_shot_example_count: MultiShotExampleCount,
    ) -> PromptType {
        let questions_and_answers: String = multi_shot_questions_and_answers
            .multi_shot_questions_and_answers()
            .into_iter()
            .take(multi_shot_example_count as usize)
            .collect();

        let questions_and_answers_with_prompt = format!(
            r#"
{questions_and_answers}
question: {inner_prompt}"#
        );
        PromptType::MultiShotPrompt(MultiShotPrompt::new(questions_and_answers_with_prompt))
    }

    /// do not really provide any utility but is used to being able to pass this
    /// type around
    pub fn new_zero_shot_prompt(inner_prompt: String) -> PromptType {
        PromptType::ZeroShotPrompt(ZeroShotPrompt::new(inner_prompt))
    }

    /// returns the constructed prompt
    pub fn prompt(&self) -> String {
        match self {
            PromptType::MultiShotPrompt(multi_shot_prompt) => multi_shot_prompt.prompt(),
            PromptType::ZeroShotPrompt(zero_shot_prompt) => zero_shot_prompt.prompt(),
        }
    }
}
