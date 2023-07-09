#[cfg(test)]
mod tests {

    use rust_llm_utils::programming::rust::fix_code::FixRustCode;
    use rust_llm_utils::TopicPrompt;
    use rust_llm_utils::{OpenAiClient, OpenAiSimplifiedResponse};

    #[tokio::test]
    async fn should_be_able_to_make_prompt_to_api() {
        // Rust code to prompt about
        let rust_code_to_fix = r#"fn some_func() -> String {\"abc\"}"#.to_string();

        // we have these prompt templates, which are like templates
        // this one is correcting Rust code
        let prompt = FixRustCode::new(rust_code_to_fix);

        // we create the client with defaults
        let open_ai_client = OpenAiClient::new(None, None);

        // then we make the call with the prompt
        let simplified_response_result = open_ai_client.perform_request(&prompt).await;

        assert!(simplified_response_result.is_ok());

        // then we get the response to our question from the response
        if let Ok(simplified_response) = simplified_response_result {
            print_banner(prompt, simplified_response);
            return;
        } else {
            println!("Error: {:?}", simplified_response_result.err());
        }
    }

    fn print_banner(topic_prompt: impl TopicPrompt, simplified_response: OpenAiSimplifiedResponse) {
        let answer = if let Some(answer) = simplified_response.answer {
            answer
        } else {
            "NA".to_string()
        };

        let followup_query = if let Some(followup_query) = simplified_response.follow_up_query {
            followup_query
        } else {
            "NA".to_string()
        };

        println!("##### ###### #####");
        println!("##### prompt #####");
        println!("##### ###### #####");
        let prompt = topic_prompt.query();
        println!("{prompt}");

        println!("##### ###### #####");
        println!("##### answer #####");
        println!("##### ###### #####");
        println!("{answer}");

        println!("##### ############### #####");
        println!("##### follow up query #####");
        println!("##### ############### #####");
        println!("{followup_query}");
    }
}
