use std::collections::HashMap;

const states_str: &str = r#"
```mermaid
stateDiagram
[*] --> llm_request("generate_code_prompt_template",[question]) : question
llm_request("generate_code_prompt_template",[question]) --> extract_code(code_response) : code_response
extract_code(code_response) --> create_project(code,dependencies,tests) : code
create_project(code,dependencies,tests) --> build_tool("build")
build_tool("build") --> finish : (true,output)
build_tool("build") --> llm_request("build_dependencies_req_prompt_template",[question,code,output]) : (false,output)
llm_request("build_dependencies_req_prompt_template",[question,code,output])  --> extract_number(response) : response
extract_number(response) --> finish : 2
extract_number(response) --> llm_request("build_dependencies_prompt_template",[question,code]) : 1
llm_request("build_dependencies_prompt_template",[question,code]) --> extract_code(dependencies_response) : dependencies_response
extract_code(dependencies_response) --> create_project(code,dependencies,tests) : dependencies
finish --> [*]
```
"#;
fn main() {
    let question = "take 2 params and multiply and return result";
    let mut code = "".to_string();
    let mut dependencies = "".to_string();
    let mut tests = "".to_string();

    run_state_machine(states_str, question, &mut code, &mut dependencies, &mut tests);
    println!("{}/n{}/n{}", code, dependencies, tests);
}



fn run_state_machine(
    states_str_var: &str,
    question: &str,
    code: &mut String,
    dependencies: &mut String,
    tests: &mut String,
) {
    let states: HashMap<String, State> = extract_states(states_str_var);
    let mut current_state_name: String = extract_first_state(states_str_var);
    let mut current_state_params: HashMap<String, String> = HashMap::new();
    loop {
        match current_state_name.as_str() {
            state_name => {
                println!("{}", current_state_name);
                println!("{:#?}", current_state_params);
                let state_type = extract_state_type(state_name);
                let state_params = extract_state_params(state_name);
                let current_state = states.get(state_name).unwrap();
                match state_type.as_str() {
                    "llm_request" => {
                        let array_src = extract_param_array(state_params[1]);
                        let array:Vec<String> = replace_in_array(array_src,  question, code, dependencies, tests);
                        let result = llm_request(state_params[0].replace("\"","").as_str(), &array);

                        let next_state_name = current_state.transitions.keys().next().unwrap().to_string();
                        let param = current_state.transitions.get(&next_state_name).unwrap().to_string();
                        let mut next_state_params = HashMap::new();
                        next_state_params.insert(param, result);

                        current_state_name = next_state_name;
                        current_state_params = next_state_params;
                        println!("===============");

                        continue;
                    }
                    "finish" => {
                        return;
                    }
                    &_ => {

                        current_state_params = HashMap::new();
                        current_state_name = "finish".to_string();
                        println!("===============");
                        continue;
                    }
                }

            }
        }
    }
}

fn replace_in_array(array: Vec<&str>, question: &str, code: &str, dependencies: &str, tests: &str) -> Vec<String> {
    let mut new_array = Vec::new();
    for item in array {
        match item {
            "question" => new_array.push(question.to_string()),
            "code" => new_array.push(code.to_string()),
            "dependencies" => new_array.push(dependencies.to_string()),
            "tests" => new_array.push(tests.to_string()),
            &_ => new_array.push(item.to_string())
        }
    }
    new_array
}


fn extract_first_state(states_str_var: &str) -> String {
    let mut states = extract_states_impl(states_str_var);
    let first_state = states.remove("[*]").unwrap();
    first_state.transitions.keys().next().unwrap().to_string()
}
#[derive(Debug)]
pub struct State {
    name: String,
    transitions: HashMap<String, String>, // state_name, condition
}
fn extract_states(states_str_var: &str) -> HashMap<String, State> {
    let mut states = extract_states_impl(states_str_var);
    states.retain(|k, _| k != "[*]");
    states
}

fn extract_states_impl(states_str_var: &str) -> HashMap<String, State> {
    let mut states_map = HashMap::new();

    for line in states_str.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        let line = line.trim_start_matches("//").trim();

        let parts: Vec<&str> = line.split("-->").collect();
        if parts.len() != 2 {
            continue;
        }
        let source = parts[0].trim();
        let rest = parts[1].trim();

        let target_and_condition: Vec<&str> = rest.split(':').collect();
        let target = target_and_condition[0].trim();
        let condition = if target_and_condition.len() > 1 {
            target_and_condition[1].trim()
        } else {
            ""
        };

        let source_state = states_map.entry(source.to_string()).or_insert(State {
            name: source.to_string(),
            transitions: HashMap::new(),
        });
        source_state
            .transitions
            .insert(target.to_string(), condition.to_string());

        // Ensure the target state exists in the map
        states_map.entry(target.to_string()).or_insert(State {
            name: target.to_string(),
            transitions: HashMap::new(),
        });
    }

    states_map
}

fn extract_state_type(state_str: &str) -> String {
    let state_type = state_str.split("(").collect::<Vec<&str>>()[0];
    state_type.to_string()
}
fn extract_state_params(state_str: &str) -> Vec<&str> {
    if !state_str.contains("(") {
        return vec![];
    }
    let state_params = state_str.split("(").collect::<Vec<&str>>()[1];
    let state_params = state_params.split(")").collect::<Vec<&str>>()[0];
    state_params.split(",").collect::<Vec<&str>>()
}
fn extract_param_array(param_str: &str) -> Vec<&str> {
    let state_params = param_str.split("[").collect::<Vec<&str>>()[1];
    let state_params = state_params.split("]").collect::<Vec<&str>>()[0];
    state_params.split(",").collect::<Vec<&str>>()
}
fn create_project(code: &str, dependencies: &str, tests: &str) {
    todo!()
}

fn llm_request(prompt: &str, params: &Vec<String>) -> String {
    println!("LLM Request: {}", prompt);
    println!("LLM Params: {:#?}", params);
    "AI response".to_string()
}

fn extract_code(response: &str) -> String {
    todo!()
}

fn extract_number(response: &str) -> i32 {
    todo!()
}

fn build_tool(command: &str) -> (bool, String) {
    todo!()
}


mod tests {
    use crate::state_machine::{extract_states,  states_str};

    #[test]
    fn test_extract_state_type() {
        let state_str = r#"llm_request("generate_code_prompt_template",[question])"#;
        assert_eq!(super::extract_state_type(state_str), "llm_request");
    }

    #[test]
    fn test_extract_state_params() {
        let state_str = r#"llm_request("generate_code_prompt_template",[question])"#;
        assert_eq!(super::extract_state_params(state_str), vec!["\"generate_code_prompt_template\"","[question]"]);
    }

    #[test]
    fn test_extract_param_array() {
        let param_str = "[question,code,output]";
        assert_eq!(super::extract_param_array(param_str), vec!["question","code","output"]);
    }

    #[test]
    fn test_extract_states() {
        println!("{:#?}", extract_states(states_str));
    }

    #[test]
    fn test_extract_first_state() {
        let first_state = super::extract_first_state(states_str);
        assert_eq!(first_state, "llm_request(\"generate_code_prompt_template\",[question])");
    }
    #[test]
    fn test_main() {
        super::main();
    }

}