use crate::cache::Cache;
use crate::{DEBUG, extract_error_message};

pub fn build_tool(lang: &str, command: &str, cache: &mut Cache) -> (i32, String) {
    if lang == "rust" {
        println!("Exec: cargo {}", command);
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
    } else {
        panic!("Unsupported language: {}", lang);
    }
}

pub fn create_project(lang: &str, code: &str, test: &str, dependencies: &str) {
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