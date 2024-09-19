mod cache;
mod prompt;

use std::time::Duration;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use crate::cache::Cache;

const DEBUG: bool = false;

fn main() {
    let mut cache = cache::Cache::new();
    let lang = "rust";
    let prompt = prompt::Prompt::new(&format!("{}.prompt", lang));

    let number_of_attempts = 3;

    println!("Explain what the function should do:");
    let mut explanation = String::new();
    std::io::stdin().read_line(&mut explanation).unwrap();

    let mut code_attempts = 0;
    let mut dependencies_attempts;
    let mut test_attempts;
    let mut code;
    let mut dependencies = "".to_string();
    let mut code_test = "".to_string();


    let generate_code_prompt = prompt.create(
        "generate_code_prompt_template",
        vec![&explanation],
    );
    println!("===============");
    let generation_code_result = llm_request(&generate_code_prompt, &mut cache);
    code = extract_code(&generation_code_result);


    create_project(lang, &code, "", "");
    let (mut exit_code, mut output) = cargo("build", &mut cache);

    'code_generation: loop {
        if code_attempts >= number_of_attempts {
            println!("Too many attempts to generate code. Exiting.");
            return;
        }
        code_attempts += 1;



        if exit_code == 0 {

            dependencies_attempts = 0;

            'dependencies_generation: loop {
                if dependencies_attempts >= number_of_attempts {

                    code_attempts = 0;
                    exit_code = 1;
                    continue 'code_generation;
                }
                dependencies_attempts += 1;
                if exit_code == 0 &&  (dependencies_attempts == 1 || dependencies == "")  {


                    let build_dependencies_req_prompt = prompt.create(
                        "build_dependencies_req_prompt_template",
                        vec![&explanation, &code, &output],
                    );
                    let build_dependencies_req_result = llm_request(&build_dependencies_req_prompt, &mut cache);
                    let build_dependencies_req = build_dependencies_req_result.trim();
                    if extract_number(build_dependencies_req) == 1 {

                        let build_dependencies_prompt = prompt.create(
                            "build_dependencies_prompt_template",
                            vec![&explanation, &code],
                        );
                        let build_dependencies_result = llm_request(&build_dependencies_prompt, &mut cache);
                        dependencies = extract_code(&build_dependencies_result);
                    }


                    create_project(lang, &code, "", &dependencies);
                }
                let (exit_code_immut, output_immut) = cargo("build", &mut cache);
                exit_code = exit_code_immut;
                output = output_immut;
                if exit_code == 0 {
                    test_attempts = 0;

                    loop {
                        if test_attempts >= number_of_attempts {
                            dependencies_attempts = 0;
                            exit_code = 0;
                            continue 'dependencies_generation;
                        }
                        test_attempts += 1;

                        if exit_code == 0 {

                            let generate_test_prompt = prompt.create(
                                "generate_test_prompt_template",
                                vec![&explanation, &code],
                            );
                            let generation_test_result = llm_request(&generate_test_prompt, &mut cache);
                            code_test = extract_code(&generation_test_result);

                            create_project(lang, &code, &code_test, &dependencies);
                        }


                        let (exit_code_immut, output_immut) = cargo("test", &mut cache);
                        exit_code = exit_code_immut;
                        output = output_immut;
                        if exit_code == 0 {
                            println!("{}\n{}\n{}", dependencies, code, code_test);
                            println!("Finished");
                            return;
                        } else {

                            let rewrite_code_req_prompt_template_prompt = prompt.create(
                                "rewrite_code_req_prompt_template",
                                vec![&explanation, &code, &code_test, &output],
                            );
                            let rewrite_code_req_result = llm_request(&rewrite_code_req_prompt_template_prompt, &mut cache);
                            if extract_number(&rewrite_code_req_result) == 1 {
                                let rewrite_code_prompt = prompt.create(
                                    "rewrite_code_prompt_template",
                                    vec![&explanation, &code, &output],
                                );
                                let rewrite_code_result = llm_request(&rewrite_code_prompt, &mut cache);
                                code = extract_code(&rewrite_code_result);
                            } else {
                                let rewrite_test_prompt = prompt.create(
                                    "rewrite_test_prompt_template",
                                    vec![&explanation, &code, &code_test, &output],
                                );
                                let rewrite_test_result = llm_request(&rewrite_test_prompt, &mut cache);
                                code_test = extract_code(&rewrite_test_result);
                            }
                        }
                    }
                } else {
                    let rewrite_dependencies_prompt = prompt.create(
                        "rewrite_dependencies_prompt_template",
                        vec![&explanation, &code, &dependencies, &output],
                    );
                    let rewrite_dependencies_result = llm_request(&rewrite_dependencies_prompt, &mut cache);
                    dependencies = extract_code(&rewrite_dependencies_result);
                }
            }
        } else {
            let build_dependencies_req_prompt = prompt.create(
                "build_dependencies_req_prompt_template",
                vec![&explanation, &code, &output],
            );
            let build_dependencies_req_result = llm_request(&build_dependencies_req_prompt, &mut cache);
            let build_dependencies_req = build_dependencies_req_result.trim();
            if extract_number(build_dependencies_req) == 1 {
                let build_dependencies_prompt = prompt.create(
                    "build_dependencies_prompt_template",
                    vec![&explanation, &code],
                );
                let build_dependencies_result = llm_request(&build_dependencies_prompt, &mut cache);
                dependencies = extract_code(&build_dependencies_result);
            }
            create_project(lang, &code, "", &dependencies);
            let (exit_code_immut, output_immut) = cargo("build", &mut cache);
            if exit_code_immut != 0 {
                output = output_immut;

                let rewrite_code_prompt = prompt.create(
                    "rewrite_code_prompt_template",
                    vec![&explanation, &code, &output],
                );
                let rewrite_code_result = llm_request(&rewrite_code_prompt, &mut cache);
                code = extract_code(&rewrite_code_result);
                create_project(lang, &code, "", &dependencies);
                let (exit_code_immut, output_immut) = cargo("build", &mut cache);
                exit_code = exit_code_immut;
                output = output_immut;
            } else {
                exit_code = 0;
            }
        }
    }

}


fn cargo(command: &str, cache: &mut Cache) -> (i32, String) {
    println!("Run: cargo {}", command);
    let code = if std::path::Path::new("sandbox/src/main.rs").exists() {
        std::fs::read_to_string("sandbox/src/main.rs").unwrap()
    } else {
        "".to_string()
    };
    let dependencies = if std::path::Path::new("sandbox/Cargo.toml").exists() {
        std::fs::read_to_string("sandbox/Cargo.toml").unwrap()
    } else {
        "".to_string()
    };
    let src= format!("{}\n{}", dependencies, code);

    let key = format!("{}{}", command, src);
    let result_str_opt = cache.get(&key);
    let result_str = match result_str_opt {
        None => {

            let output = std::process::Command::new("cargo")
                .arg(command)
                .current_dir("sandbox")
                .output()
                .unwrap();
            let exit_code = output.status.code().unwrap();
            let std_out = String::from_utf8(output.stdout).unwrap();
            let std_err = String::from_utf8(output.stderr).unwrap();
            let output = std_err + &std_out;
            let tuple: (i32, String) = (exit_code, output);
            let json_str = serde_json::to_string(&tuple).unwrap();
            cache.set(key, json_str.clone());
            json_str
        }
        Some(result) => {
            result.to_string()
        }
    };
    let parsed: (i32, String) = serde_json::from_str(&result_str).unwrap();

    let exit_code = parsed.0;
    let output = parsed.1;


    println!("Exit code: {}", exit_code);
    if DEBUG {
        println!("Output: {}", output);
    }
    println!("===============");

    (exit_code,extract_error_message(&output, exit_code))
}
fn create_project(lang: &str, code: &str, test: &str, dependencies: &str) {
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
    println!("{}\n{}\n{}", dependencies, code, test);
    println!("====================");

    if lang == "rust" {
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
    } else {
        panic!("Unsupported language: {}", lang);
    }
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


fn llm_request(prompt: &str, cache: &mut Cache) -> String {
    let stop = vec!["**Explanation".to_string()];
    let request = OllamaRequest {
        model: "gemma2:27b".to_string(),
        prompt: prompt.to_string(),
        stream: false,
        options: OllamaOptions {
            num_predict: 500,
            stop: stop
        },
    };

    let request_str = serde_json::to_string(&request).unwrap();
    println!("Request: {}", request.prompt);
    println!("===============");

    let response_opt = cache.get(&request_str);
    let response = match response_opt {
        None => {
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
            cache.set(request_str.clone(), response.response.clone());
            response.response
        }
        Some(result) => {
            result.to_string()
        }
    };

    println!("Response: {}", response);
    println!("===============");
    response
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
    fn test_extract_number() {
        let input = "Bla bla bla\nTututu 123\nmore bla bla\nTutu 456\nbla bla";
        let expected = 123;
        assert_eq!(extract_number(input), expected);
    }
}