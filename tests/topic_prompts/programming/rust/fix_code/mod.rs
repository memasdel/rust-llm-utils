use crate::topic_prompts::TopicPrompt;

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
///     String::from("abc")
/// }
/// ```
///
/// In this updated code, the `String::from` function is used to create a new
/// String` object with the value "abc", which is then returned by the function.
/// ````
/// But you should define these, so that they are specific to your use case.
pub struct FixRustCode {
    query: String,
}

impl TopicPrompt for FixRustCode {
    fn query(&self) -> String {
        self.query.clone()
    }

    fn new_from_prompt_template(code_to_fix: String) -> Self {
        let augmented_query = format!(
            r#"Could you help me to fix this Rust code:
```rust
{code_to_fix}
```
"#
        );

        Self {
            query: augmented_query,
        }
    }
}
