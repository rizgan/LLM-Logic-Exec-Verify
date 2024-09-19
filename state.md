```mermaid
stateDiagram
[*] --> llm_request("generate_code_prompt_template",[question]) : question
llm_request("generate_code_prompt_template",[question]) --> extract_code(code_response) : code_response
extract_code(code_response) --> create_project(code,"","") : code
create_project(code,"","") --> build_tool("build")
create_project(code,dependencies,"") --> build_tool("build")
create_project(code,dependencies,test) --> build_tool("test")
build_tool("test") --> finish : (true,output)
build_tool("build") --> build_tool("test") : (true,output) 
build_tool("build") --> llm_request("build_dependencies_req_prompt_template",[question,code,output]) : (false,output) 
llm_request("build_dependencies_req_prompt_template",[question,code,output])  --> extract_number(dependency_response) : dependency_response
extract_number(dependency_response) --> llm_request("generate_test_prompt_template",[question,code,dependencies]) : 2
extract_number(dependency_response) --> llm_request("build_dependencies_prompt_template",[question,code]) : 1
llm_request("build_dependencies_prompt_template",[question,code]) --> extract_code(dependencies_response) : dependencies_response
extract_code(dependencies_response) --> create_project(code,dependencies,"") : dependencies 
llm_request("generate_test_prompt_template",[question,code,dependencies]) --> extract_code(test_response) : test_response
extract_code(test_response) --> create_project(code,dependencies,test) : test
finish --> [*]
```