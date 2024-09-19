use std::time::Duration;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use crate::cache::Cache;
use crate::llm_prompt::Prompt;

pub fn llm_request(prompt_template: &str, params: &Vec<String>, cache: &mut Cache, prompt: &Prompt) -> String {
    let prompt = prompt.create(prompt_template, params);
    let stop = vec!["**Explanation".to_string()];
    let request = OllamaRequest {
        model: "gemma2:27b".to_string(),
        // model: "gemma2:2b".to_string(),
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
