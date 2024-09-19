use std::collections::HashMap;

fn main() {
    let states = r#"
stateDiagram
[*] --> llm_request("generate_code_prompt_template",[question]) : question
llm_request("generate_code_prompt_template",[question]) --> extract_code(code_response) : code_response
extract_code(code_response) --> create_project(code,dependencies,tests) : code
create_project(code,dependencies,tests) --> build_tool("build")
build_tool("build") --> finish : (true,output)
build_tool("build") --> llm_request("build_dependencies_req_prompt_template",[question,code,output]) : (false,output)
llm_request("build_dependencies_req_prompt_template",[question,code,output])  --> extract_number(response) : response
extract_number(response) --> finish : 2
extract_number(response) -->  llm_request("build_dependencies_prompt_template",[question,code]) : 1
llm_request("build_dependencies_prompt_template",[question,code]) --> extract_code(dependencies_response) : dependencies_response
extract_code(dependencies_response) --> create_project(code,dependencies,tests) : dependencies
finish --> [*]
"#;
    let question = "take 2 params and multiply and return result";
    let mut code = "".to_string();
    let mut dependencies = "".to_string();
    let mut tests = "".to_string();

    run_state_machine(states, question, &mut code, &mut dependencies, &mut tests);
    println!("{}/n{}/n{}", code, dependencies, tests);
}

pub struct State {
    name: String,
    transitions: HashMap<String, String>, // state_name, condition
}

fn run_state_machine(
    states_str: &str,
    question: &str,
    code: &mut String,
    dependencies: &mut String,
    tests: &mut String,
) {
    let mut states: HashMap<String, State> = extract_states(states_str);
    let mut current_state_name: String = extract_first_state(states_str);
    loop {
        match current_state_name.as_str() {
            state_name => {
                current_state_name = "".to_string();
                continue;
            }

            "finish" => {
                // finish --> [*]
                break;
            }
        }

    }
}

fn extract_states(p0: &str) -> HashMap<String, State> {
    todo!()
}

fn extract_first_state(p0: &str) -> String {
    todo!()
}

// IN: llm_request("generate_code_prompt_template",[question])
// OUT: llm_request
fn extract_state_type(state_str: &str) -> String {
    let state_type = state_str.split("(").collect::<Vec<&str>>()[0];
    state_type.to_string()
}
// IN: llm_request("generate_code_prompt_template",[question])
// OUT: vec!["\"generate_code_prompt_template\"","[question]"]
fn extract_state_params(state_str: &str) -> Vec<&str> {
    let state_params = state_str.split("(").collect::<Vec<&str>>()[1];
    let state_params = state_params.split(")").collect::<Vec<&str>>()[0];
    state_params.split(",").collect::<Vec<&str>>()
}

// IN: [question,code,output]
// OUT: vec!["question","code","output"]
fn extract_param_array(param_str: &str) -> Vec<&str> {
    let state_params = param_str.split("[").collect::<Vec<&str>>()[1];
    let state_params = state_params.split("]").collect::<Vec<&str>>()[0];
    state_params.split(",").collect::<Vec<&str>>()
}
fn create_project(code: &str, dependencies: &str, tests: &str) {

}

fn llm_request(prompt: &str, params: Vec<&str>) -> String {
    todo!()
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
}