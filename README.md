# Rust LLM Utils 🦀🤖🛠

This project is a framework for building LLM enabled Rust applications. It tries to identify the largest possible set of functionality which is common between projects, and write it once here.

If all goes well, this framework will contain just the right level of abstraction, so that the consumers can truely avoid writing non interesting part of their applications, while still be open enough to cover all necessary use cases.


# Road map 🛣🏔
* [x] Make it easy to generate prompt templates and execute them on OpenAI API.
* [x] Add different prompt types:
	* [x] Single shot
	* [x] Few shot
* [ ] Add calls to Llama 2.
* [ ] create macro for defining LLM templates using Rust function signatures.
* [ ] Make it easy to generate prompts that generate prompts and then execute them.
* [ ] Allow defining JSON schemas for responses.
* [ ] Allow defining Rust types for responses.
* [ ] Introduce a concept of workflow, which is a chain of prompts, maybe call
  it `PromptFlow`.
* [ ] "Memory" and summarizing functionality.
* [ ] Persistance for the "memory".
* [ ] Allow executing same prompts parallel in several LLMs.
* [ ] Make this a node dep and allow calling from TypeScript
* [ ] Use some more polished SDKs for the LLM calls.

## Scripts
* `get_available_open_ai_models.sh` - pulls the models from OpenAI API

## Glossary
* `TopicPrompt` - this is a trait which represents a prompt that is specific to
  a certain query. An example could be "fix syntax error in Rust code", "fix
  syntax error in Java code", "Explain a concept in software engineering". The
  purpose of having this is so that the prompt to explain something can be
  quickly improved for a large set of queries. It contains a template for a
  prompt:
  r#"Could you fix this rust code:
  \```rust
  {rust_code_to_fix}
  \```
  "#
* `Prompt` - This is the prompt that is sent to an AI service, such as OpenAI
  API or llama. This do not care about the actual content of the prompt, it just
  generates the body of the request.
* `query` - is the actual string that is being placed to a `Prompt` to be sent
  to an AI service. An example could be:
  "Could you fix this rust code:
  ```rust
  fn do_something() -> String {
	"abc"
  }
  ```
  "
* `PromptFlow` - A chain of prompts that together form a workflow. Results from prompts may alter the flow of the `PromptFlow`.
