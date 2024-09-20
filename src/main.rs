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
    let mut lines = vec![];
    let mut start_sec = 0 as u64;
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        lines.push(line);
        let now_sec = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        if start_sec == 0 {
            start_sec = now_sec;
        } else {
            if now_sec - start_sec < 1 {
                continue;
            } else {
                break
            }
        }
        if question.trim() != "" {
            break;
        }
    }
    question = lines.join("");

    let mut code = "".to_string();
    let mut dependencies = "".to_string();
    let mut tests = "".to_string();
    let mut output = "".to_string();

    let states_str = std::fs::read_to_string("logic.md").unwrap();
    println!("====================");
    state_machine::run_state_machine(&states_str, &question, &mut code, &mut dependencies, &mut tests, &mut output, &prompt, &mut cache, lang);
    println!("++++++++ Finished ++++++++++++");
    println!("\n{}\n{}\n{}", code, dependencies, tests);
    println!("++++++++ Finished ++++++++++++");


}







