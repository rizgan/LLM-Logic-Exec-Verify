use serde::{Deserialize, Serialize};

fn main() {
    let generate_code_prompt_template = r#"
{{{0}}}

Write on Rust language code of this function (only function body without example of usage):
```rust
fn solution(
"#;
    let generate_test_prompt_template = r#"
{{{0}}}
Rust language code of this function:
```rust
{{{1}}}
```

Write on Rust language code of test for this function (only test code without function implementation):
```rust
#[cfg(test)]
mod tests {
use super::*;

#[test]
"#;


    println!("Explain what function should to do:");
    let mut explanation = String::new();
    std::io::stdin().read_line(&mut explanation).unwrap();
    let generate_code_prompt = construct_prompt(generate_code_prompt_template, vec![&explanation]);
    let generation_code_result = generate(&generate_code_prompt);
    println!("{}", generation_code_result);
    println!("===============");
    let code =  extract_code(&generation_code_result);
    println!("{}",code);
    println!("===============");
    create_rust_project(&code, "");
    let (exit_code, output) = execute("build");
    if exit_code == 0 {
        let generate_test_prompt = construct_prompt(generate_test_prompt_template, vec![&explanation, &code]);
        let generation_test_result = generate(&generate_test_prompt);
        let code_test = extract_code(&generation_test_result);
        println!("{}", code_test);
        println!("===============");
        create_rust_project(&code, &code_test);
        let (exit_code, output) = execute("test");
        if exit_code == 0 {
          println!("{}\n{}", code, code_test);
        } else {

        }
    }

}


fn execute(command: &str) -> (i32, String) {
    println!("cargo {}", command);
    let output = std::process::Command::new("cargo")
        .arg(command)
        .current_dir("sandbox")
        .output()
        .unwrap();
    let exit_code = output.status.code().unwrap();
    let std_out = String::from_utf8(output.stdout).unwrap();
    let std_err = String::from_utf8(output.stderr).unwrap();
    println!("Exit code: {}", exit_code);
    let output = std_out + &std_err;
    println!("Output: {}", output);
    println!("===============");

    (exit_code,output)
}
fn create_rust_project(code: &str, test: &str) {
    let sandbox_path = "sandbox";
    let src_path = format!("{}/src", sandbox_path);
    let main_path = format!("{}/src/main.rs", sandbox_path);
    let cargo_path = format!("{}/Cargo.toml", sandbox_path);
    if !std::path::Path::new(sandbox_path).exists() {
        std::fs::create_dir(sandbox_path).unwrap();
    } else {
        std::fs::remove_dir_all(sandbox_path).unwrap();
        std::fs::create_dir(sandbox_path).unwrap();
    }
    if !std::path::Path::new(&src_path).exists() {
        std::fs::create_dir(&src_path).unwrap();
    }
    let main_rs = r#"fn main() {}"#;
    std::fs::write(&main_path, format!("{}\n{}\n{}", main_rs, code, test)).unwrap();

    let cargo_toml = r#"
[package]
name = "sandbox"
version = "0.1.0"
edition = "2018"

[dependencies]
"#;
    std::fs::write(&cargo_path, cargo_toml).unwrap();
}
fn construct_prompt(template: &str, replace: Vec<&str>) -> String {
    let mut prompt = template.to_string();
    for (i, r) in replace.iter().enumerate() {
        let placeholder = format!("{{{{{{{}}}}}}}", i); // "{{{0}}}"
        prompt = prompt.replace(&placeholder, r);
    }
    prompt
}
fn extract_code(input: &str) -> String {
    let mut code = String::new();
    let mut in_code_block = false;
    for line in input.lines() {
        if line.starts_with("```") {
            if in_code_block {
                return code;
            }
            in_code_block = !in_code_block;
        } else if in_code_block {
            code.push_str(line);
            code.push_str("\n");
        }
    }
    code
}


fn generate(prompt: &str) -> String {
    let request = OllamaRequest {
        model: "gemma2:2b".to_string(),
        prompt: prompt.to_string(),
        stream: false,
        options: OllamaOptions {
            num_predict: 10000,
        },
    };
    println!("Request: {}", request.prompt);
    println!("===============");

    let response = reqwest::blocking::Client::new()
        .post("http://127.0.0.1:11434/api/generate")
        .json(&request)
        .send()
        .unwrap()
        .json::<OllamaResponse>()
        .unwrap();

    // print request and response
    println!("Response: {}", response.response);
    println!("===============");

    response.response
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OllamaOptions {
    num_predict: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct OllamaResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
    done_reason: String,
    context: Vec<i64>,
    total_duration: i64,
    load_duration: i64,
    prompt_eval_count: i32,
    prompt_eval_duration: i64,
    eval_count: i32,
    eval_duration: i64,
}

// tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_code() {
        let input = "This is code\n```rust\nprintln!(\"{}\", generate(\"What is the capital of France?\"));\n```\nExplanation of code  This is code\n```rust\nprintln!(\"{}\", generate(\"What is the capital of France?\"));\n```\nExplanation of code";
        let expected = "println!(\"{}\", generate(\"What is the capital of France?\"));\n";
        assert_eq!(extract_code(input), expected);
    }

    #[test]
    fn test_construct_prompt() {
        let template = "This is a template with {{{0}}} and {{{1}}}";
        let replace = vec!["first", "second"];
        let expected = "This is a template with first and second";
        assert_eq!(construct_prompt(template, replace), expected);
    }
}