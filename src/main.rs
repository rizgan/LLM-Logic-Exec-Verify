mod cache;
mod llm_prompt;
mod build_tool;

mod llm_api;
mod llm_parser;
mod state_machine;


const DEBUG: bool = false;

fn main() {
    let mut cache = cache::Cache::new();
    let lang = "rust";
    let prompt = llm_prompt::Prompt::new(&format!("{}.prompt", lang));

    println!("Explain what the function should do:");
    let mut question = String::new();
    std::io::stdin().read_line(&mut question).unwrap();

    let mut code = "".to_string();
    let mut dependencies = "".to_string();
    let mut tests = "".to_string();
    let mut output = "".to_string();

    let states_str = std::fs::read_to_string("logic.md").unwrap();
    println!("====================");
    state_machine::run_state_machine(&states_str, &question, &mut code, &mut dependencies, &mut tests, &mut output, &prompt, &mut cache, lang);
    println!("++++++++++++++++++++++++");
    println!("Finished\n{}\n{}\n{}", code, dependencies, tests);


}







