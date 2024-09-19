mod cache;
mod llm_prompt;
mod build_tool;

mod llm_api;
mod llm_parser;

use crate::build_tool::{build_tool, create_project};
use crate::llm_api::llm_request;
use crate::llm_parser::{extract_code, extract_number};

const DEBUG: bool = false;

fn main() {
    let mut cache = cache::Cache::new();
    let lang = "rust";
    let prompt = llm_prompt::Prompt::new(&format!("{}.prompt", lang));

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
    let (mut exit_code, mut output) = build_tool(lang, "build", &mut cache);

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
                let (exit_code_immut, output_immut) = build_tool(lang, "build", &mut cache);
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


                        let (exit_code_immut, output_immut) = build_tool(lang, "test", &mut cache);
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
            let (exit_code_immut, output_immut) = build_tool(lang, "build", &mut cache);
            if exit_code_immut != 0 {
                output = output_immut;

                let rewrite_code_prompt = prompt.create(
                    "rewrite_code_prompt_template",
                    vec![&explanation, &code, &output],
                );
                let rewrite_code_result = llm_request(&rewrite_code_prompt, &mut cache);
                code = extract_code(&rewrite_code_result);
                create_project(lang, &code, "", &dependencies);
                let (exit_code_immut, output_immut) = build_tool(lang, "build", &mut cache);
                exit_code = exit_code_immut;
                output = output_immut;
            } else {
                exit_code = 0;
            }
        }
    }

}







