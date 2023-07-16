/// `programming::rust::fix_code:FixRustCode` is just an example of how to use TopicPrompt.
pub mod programming;

pub mod test_prompts;

/// `TopicPrompt` is a prompt for specific kind of a question. It is like a
/// template that allow the developer to centralize the formulating of the question
/// or request to an LLM. The actual prompts sent to the LLMs can then be
/// constructed from these.
///
/// # Examples
/// ```
/// user: "could you fix this rust code: {rust_code_to_fix}"
/// user: "could you fix this rust code and explain the mistakes: {rust_code_to_fix}"
/// user: "could you give issues in the following code in bullet points: {rust_code_to_fix}"
/// ```
///
/// For now it is very simple, but I will extend it to contain functionality to
/// follow up prompts to that it will be easier to build workflows composes of
/// prompts. At that point having it as a trait and being able to define common
/// behavior will become more useful.
pub trait TopicPrompt {
    fn new(input: String) -> Self;
    fn query(&self) -> String;
}
