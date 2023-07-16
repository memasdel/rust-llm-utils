/// with this we can convey those example QA
pub trait MultiShotQuestionsAndAnswers {
    /// this simply returns a fixed number of example questions and answers.
    /// This is being utilised by [`MultiShot`] to get the example QA.
    fn multi_shot_questions_and_answers() -> [String; 5];
}

pub enum MultiShotExampleCount {
    One = 1,
    Two = 2,
    Tree = 3,
    Four = 4,
    Five = 5,
}

/// this has to be implemented for a trait and then that trait has to be defined
/// as a type for the type [`MultiShotQuestionsAndAnswers`] for an implementation
/// for a prompt. This allows the developer of a prompt to define a set of
/// questions and answers for a prompt to be used as a multi shot prompt.
///
/// # Example
///
/// We can generate a very use case specific example for mapping from prompt to
/// answer that the model will then follow with our prompt. This can be used to
/// very specifically to influence how the model will response to our prompt:
///
/// ```bash
/// prompt: it is 25c and sunny in Berlin, answer: it seems like summer weather. Es sieht aus wie Sommerwetter ☀️
/// prompt: it is 8c and rainy in Berlin, answer: it seems like autumn weather. Es sieht aus wie Herbstwetter 🍂
/// prompt: it is 11c and sunny in Berlin, answer: it seems like autumn weather. Es sieht aus wie Herbstwetter 🍂
/// question: Could you generate a statement about the weather based on the weather condition statement in similar style as my examples above.
/// it is -8c and snowing in Berlin
/// ````
///
/// model (GPT-4) then reply with:
/// ```bash
/// answer: It seems like winter weather. Es sieht aus wie Winterwetter ❄️
/// ````
///
/// **Q:** Why do I need this? What is multi shot?
///
/// **A:** by providing the model example prompts and answers you can influence more
///   precisely how it will be response to your query. This is not necessary to
///   be defined for all types of prompts and even for those that it is, it
///   might not have to be unique. For examples if we were to define it to
///   support in programming, we would not define different ones for Java and C++.
///
/// **Q:** You lost me already: why do I need this and why would I care if it is 1 or 5?
///
/// **A:** in some cases 1 or 2 examples might be enough, if it is, it is preferable
///   to save tokens. However if it is not you might have to define more examples.
///
/// # usage
/// ```no_run
///
/// // we have some structure
/// struct FixJavaCodeShotQnA;
///
/// // we implement a function that returns those example QA
/// impl MultiShotQuestionsAndAnswers for FixJavaCodeShotQnA {
///
/// fn multi_shot_questions_and_answers() -> [String; 5] {
///     let qa_1 = r#"question: some example question 1, answer: some example answer 1"#
///     let qa_2 = r#"question: some example question 2, answer: some example answer 2"#
///     let qa_3 = r#"question: some example question 3, answer: some example answer 3"#
///     let qa_4 = r#"question: some example question 4, answer: some example answer 4"#
///     let qa_5 = r#"question: some example question 5, answer: some example answer 5"#
/// 	[qa_1, qa_2, qa_3, qa_4, qa_5]
/// }
///
/// // now we can implement `MultiShot` for our prompt, it only needs that type
/// //  for returning those example QA
/// impl MultiShot for FixJavaCode {
/// 	type MultiShotQuestionsAndAnswers = MockPromptMultiShortQnA;
/// }
/// ```
pub trait MultiShot {
    /// we have to provide a type that has a function to give those example QA
    type MultiShotQuestionsAndAnswers: MultiShotQuestionsAndAnswers;

    /// gets the prompt that has been already generated from a template and a
    /// number of examples to include in the prompt.
    ///
    /// For now the type for `prompt` is `String` but this can be changed to
    /// more strict to be a `TopicPrompt`.
    fn wrap_to_prompt_type(prompt: String, example_count: MultiShotExampleCount) -> String {
        let questions_and_answers: String =
            Self::MultiShotQuestionsAndAnswers::multi_shot_questions_and_answers()
                .into_iter()
                .take(example_count as usize)
                .collect();

        let questions_and_answers_with_prompt = format!(
            r#"
{questions_and_answers}
question: {prompt}"#
        );

        questions_and_answers_with_prompt
    }
}

#[cfg(test)]
mod tests {
    use super::MultiShotQuestionsAndAnswers;
    use super::{MultiShot, MultiShotExampleCount};
    use crate::TopicPrompt;

    struct MockPrompt;
    impl TopicPrompt for MockPrompt {
        fn query(&self) -> String {
            "&self.query".to_string()
        }

        fn new(code_to_fix: String) -> Self {
            Self {}
        }
    }

    struct MockPromptMultiShortQnA;

    impl MultiShotQuestionsAndAnswers for MockPromptMultiShortQnA {
        fn multi_shot_questions_and_answers() -> [String; 5] {
            let qa_1 = r#"question: aaa
	answer: aaaa
	###"#
                .to_string();

            let qa_2 = r#"
	question: aaa
	answer: aaaa
	###"#
                .to_string();

            let qa_3 = r#"
	question: aaa
	answer: aaaa
	###"#
                .to_string();

            let qa_4 = r#"
	question: aaa
	answer: aaaa
	###"#
                .to_string();

            let qa_5 = r#"
	question: aaa
	answer: aaaa
	###"#
                .to_string();

            [qa_1, qa_2, qa_3, qa_4, qa_5]
        }
    }

    impl MultiShot for MockPrompt {
        type MultiShotQuestionsAndAnswers = MockPromptMultiShortQnA;
    }

    #[test]
    fn should_contain_one_question() {
        let question_answer_pairs =
            MockPrompt::wrap_to_prompt_type("aaa".to_string(), MultiShotExampleCount::One);
        assert_eq!(
            question_answer_pairs
                .matches("question:")
                .collect::<Vec<_>>()
                .len(),
            1 + 1
        );
    }

    #[test]
    fn should_contain_five_questions() {
        let question_answer_pairs =
            MockPrompt::wrap_to_prompt_type("aaa".to_string(), MultiShotExampleCount::Five);
        assert_eq!(
            question_answer_pairs
                .matches("question:")
                .collect::<Vec<_>>()
                .len(),
            5 + 1
        );
    }

    #[test]
    fn should_contain_questions_and_prompt() {
        let prompt = "prompt for to use in this test".to_string();
        let question_answer_pairs =
            MockPrompt::wrap_to_prompt_type(prompt.clone(), MultiShotExampleCount::Five);

        println!("{question_answer_pairs}");
        assert_eq!(
            question_answer_pairs
                .matches(prompt.as_str())
                .collect::<Vec<_>>()
                .len(),
            1
        );
    }
}
