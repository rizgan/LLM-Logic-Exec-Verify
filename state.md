```rust
fn main() {
    let question = "take 2 params and multiply and return result";
    //...
}

fn create_project(code: &str, dependencies: &str, tests: &str) { }

fn llm_request(prompt: &str, params: Vec<&str>) -> String { }

fn extract_code(response: &str) -> String { }

fn extract_number(response: &str) -> i32 { }


```


```mermaid
stateDiagram
[*] --> llm_request("generate_code_prompt_template",question) : question
llm_request("generate_code_prompt_template",question) --> extract_code(response_code) : response_code
extract_code(response_code) --> create_project(code,dependencies,tests) : code
create_project(code,dependencies,tests) --> [*]
```

