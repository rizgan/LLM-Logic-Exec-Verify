use crate::DEBUG;

pub fn extract_code(input: &str) -> String {
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




pub fn extract_number(input: &str) -> i32 {
    for word in input.split_whitespace() {
        if let Ok(num) = word.parse::<i32>() {
            return num;
        }
    }
    1 // default value if no number found
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