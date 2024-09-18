// take 2 params and multiply and return result
// take 1 parameter multiply by random number and return tuple with  result and random number
// parse json string and return struct User (age, name) and use PartialEq for User
use std::time::Duration;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

const DEBUG: bool = false;

fn main() {

    let number_of_attempts = 4;
    let number_of_attempts_test= 1;
    let generate_code_prompt_template = r#"
{{{0}}}

Write on Rust language code of this function (without example of usage like main function):
```rust
fn solution(
"#;

let rewrite_code_prompt_template = r#"
{{{0}}}
Rust language code of this function:
```rust
{{{1}}}
```
Try to compile this code:
'''bash
cargo build
'''
Result of compilation:
'''console
{{{2}}}
'''

Rewrite code for fixing errors of this function (without example of usage like main function):
```rust
"#;


 let build_dependencies_req_prompt_template = r#"
{{{0}}}
Rust language code of this function:
```rust
{{{1}}}
```

For this function is not required some dependencies in Cargo.toml file?
1. Yes
2. No
Anser(just number):"#;


    let build_dependencies_prompt_template = r#"
{{{0}}}
Rust language code of this function:
```rust
{{{1}}}
```

Write dependencies to Cargo.toml file (only dependencies section without Rust language code):
```toml
[package]
name = "sandbox"
version = "0.1.0"
edition = "2018"

[dependencies]
"#;


    let rewrite_dependencies_prompt_template = r#"
{{{0}}}
Rust language code of this function:
```rust
{{{1}}}
```

Cargo.toml file
```toml
[package]
name = "sandbox"
version = "0.1.0"
edition = "2018"

{{{2}}}
```

cargo build

Result of compilation:

```console
{{{3}}}
```

Rewrite dependencies for fixing error to Cargo.toml file (only dependencies section without Rust language code without comments):
```toml
[dependencies]
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
fn test_solution(
"#;
    let rewrite_test_prompt_template = r#"
{{{0}}}
Rust language code of this function:
```rust
{{{1}}}
```

Test code for this function:
```rust
{{{2}}}
```

'''bash
cargo test
'''

Result of testing:
'''console
{{{3}}}
'''

Rewrite test code for fixing error (only test code without function implementation):
```rust
#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test_solution(
"#;


    println!("Explain what function should to do:");
    let mut explanation = String::new();
    std::io::stdin().read_line(&mut explanation).unwrap();
    let generate_code_prompt = construct_prompt(generate_code_prompt_template, vec![&explanation]);
    println!("===============");
    let generation_code_result = generate(&generate_code_prompt);
    let mut code =  extract_code(&generation_code_result);

    let mut dependencies: String = "".to_string();
    let build_dependencies_req_prompt = construct_prompt(build_dependencies_req_prompt_template, vec![&explanation, &code]);
    let build_dependencies_req_result = generate(&build_dependencies_req_prompt);
    let build_dependencies_req = build_dependencies_req_result.trim();
    if extract_number(build_dependencies_req) == 1 {
        let build_dependencies_prompt = construct_prompt(build_dependencies_prompt_template, vec![&explanation, &code]);
        let build_dependencies_result = generate(&build_dependencies_prompt);
        dependencies = extract_code(&build_dependencies_result);
        create_rust_project("", "", &dependencies);
        let (mut exit_code, mut output) = execute("build");
        let mut dependencies_rewrite_count = 0;
        while exit_code != 0 || dependencies_rewrite_count == 0 {
            if dependencies_rewrite_count > number_of_attempts {
                println!("Too many attempts to rewrite dependencies. Exit.");
                println!("===============");
                return;
            }
            if exit_code == 0 {
                break;
            } else {
                dependencies_rewrite_count += 1;
                let rewrite_dependencies_prompt = construct_prompt(rewrite_dependencies_prompt_template, vec![&explanation, &code, &dependencies, &output]);
                let rewrite_dependencies_result = generate(&rewrite_dependencies_prompt);
                dependencies = extract_code(&rewrite_dependencies_result);
                create_rust_project("", "", &dependencies);
                (exit_code, output) = execute("build");
            }
        }
    }

    create_rust_project(&code, "", &dependencies);
    let (mut exit_code, mut output) = execute("build");
    let mut code_rewrite_count = 0;
    while exit_code != 0 || code_rewrite_count == 0 {
        if code_rewrite_count > number_of_attempts {
            println!("Too many attempts to rewrite code. Exit.");
            println!("===============");
            return;
        }
        if exit_code == 0 {
            let mut test_rewrite_count = 0;
            let generate_test_prompt = construct_prompt(generate_test_prompt_template, vec![&explanation, &code]);
            let generation_test_result = generate(&generate_test_prompt);
            let mut code_test = extract_code(&generation_test_result);

            create_rust_project(&code, &code_test, &dependencies);
            loop {
                let (exit_code, output) = execute("test");
                if exit_code == 0 {
                    println!("{}\n{}\n{}", dependencies,  code, code_test);
                    println!("Finished");
                    return;
                } else {
                    test_rewrite_count += 1;
                    if test_rewrite_count > number_of_attempts_test {
                        let rewrite_code_prompt = construct_prompt(rewrite_code_prompt_template, vec![&explanation, &code, &output]);
                        let rewrite_code_result = generate(&rewrite_code_prompt);
                        code = extract_code(&rewrite_code_result);
                        create_rust_project(&code, &code_test, &dependencies);
                    } else {
                        let rewrite_test_prompt = construct_prompt(rewrite_test_prompt_template, vec![&explanation, &code, &code_test, &output]);
                        let rewrite_test_result = generate(&rewrite_test_prompt);
                        code_test = extract_code(&rewrite_test_result);
                        create_rust_project(&code, &code_test, &dependencies);
                    }
                }

                if test_rewrite_count > number_of_attempts {
                    println!("Too many attempts to rewrite test. Exit.");
                    println!("===============");
                    return;
                }
            }
        } else {
            code_rewrite_count += 1;

            let rewrite_dependencies_prompt = construct_prompt(rewrite_dependencies_prompt_template, vec![&explanation, &code, &dependencies, &output]);
            let rewrite_dependencies_result = generate(&rewrite_dependencies_prompt);
            let dependencies_new = extract_code(&rewrite_dependencies_result);
            create_rust_project(&code, "", &dependencies_new);
            (exit_code, output) = execute("build");
            if exit_code == 0 {
                dependencies = dependencies_new;
            }

            let rewrite_code_prompt = construct_prompt(rewrite_code_prompt_template, vec![&explanation, &code, &output]);
            let rewrite_code_result = generate(&rewrite_code_prompt);
            code = extract_code(&rewrite_code_result);
            create_rust_project(&code, "", &dependencies);
            (exit_code, output) = execute("build");
        }
    }

}


fn execute(command: &str) -> (i32, String) {
    println!("Run: cargo {}", command);
    let output = std::process::Command::new("cargo")
        .arg(command)
        .current_dir("sandbox")
        .output()
        .unwrap();
    let exit_code = output.status.code().unwrap();
    let std_out = String::from_utf8(output.stdout).unwrap();
    let std_err = String::from_utf8(output.stderr).unwrap();
    println!("Exit code: {}", exit_code);
    let output = std_err + &std_out ;
    if DEBUG {
        println!("Output: {}", output);
    }
    println!("===============");

    (exit_code,extract_error_message(&output, exit_code))
}
fn create_rust_project(code: &str, test: &str, dependencies: &str) {
    let code_str = if code == "" {
        ""
    } else {
        "'code'"
    };
    let test_str = if test == "" {
        ""
    } else {
        "'test'"
    };

    let dependencies_str = if dependencies == "" {
        ""
    } else {
        "'dependencies'"
    };

    println!("Create sandbox project with: {} {} {}", code_str,  dependencies_str, test_str);
    println!("====================");

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

    std::fs::write(&cargo_path, format!(r#"
[package]
name = "sandbox"
version = "0.1.0"
edition = "2018"

{}
"#, dependencies )).unwrap();
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
    let mut code = "".to_string();
    let mut in_code_block = false;
    for line in input.lines() {
        if line.trim().starts_with("```") {
            if in_code_block {
                let res = if code == "" {
                    "Error: extract_code()".to_string()
                } else {
                    code
                };
                if DEBUG {
                    println!("{}",res);
                    println!("============");
                }
                return res;
            }
            in_code_block = !in_code_block;
        } else if in_code_block {
            code.push_str(line);
            code.push_str("\n");
        }
    }
    let res = if code == "" {
        "Error: extract_code()".to_string()
    } else {
        code
    };

    if DEBUG {
        println!("{}",res);
        println!("============");
    }

    res
}


fn generate(prompt: &str) -> String {
     let stop = vec!["**Explanation".to_string()];
    let request = OllamaRequest {
        model: "gemma2".to_string(),
        prompt: prompt.to_string(),
        stream: false,
        options: OllamaOptions {
            num_predict: 500,
            stop: stop
        },
    };
    println!("Request: {}", request.prompt);
    println!("===============");
    let client = Client::builder()
        .timeout(Duration::from_secs(60*5))
        .build()
        .unwrap();

    let response = client
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
    stop: Vec<String>,
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


fn extract_number(input: &str) -> i32 {
    for word in input.split_whitespace() {
        if let Ok(num) = word.parse::<i32>() {
            return num;
        }
    }
    1 // default value if no number found
}

fn extract_error_message(output: &str, exit_code: i32) -> String {
    let mut error_lines = Vec::new();
    let mut in_error_section = false;

    for line in output.lines() {
        if line.starts_with("error[") {
            in_error_section = true;
        }

        if in_error_section {
            error_lines.push(line);

            if line.starts_with("For more information about this error") {
                in_error_section = false;
            }
        }
    }

    let r = error_lines.join("\n");
    let ret = if r == "" && exit_code != 0 {
        output.to_string()
    } else  {
        r
    };
    if DEBUG {
        println!("=========Errors=========:");
        println!("{}", ret);
        println!("===================");
    }
    ret
}


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

    #[test]
    fn test_extract_number() {
        let input = "Bla bla bla\nTututu 123\nmore bla bla\nTutu 456\nbla bla";
        let expected = 123;
        assert_eq!(extract_number(input), expected);
    }
}