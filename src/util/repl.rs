use std::env;

pub fn replace_envs(input: &str) -> String {
    let mut output = String::new();
    let mut i = 0;
    while i < input.len() {
        if input.chars().nth(i).unwrap() == '$' {
            // Check for the ${VARIABLE} syntax
            if i + 1 < input.len() && input.chars().nth(i + 1).unwrap() == '{' {
                let mut var_name = String::new();
                // Extract variable name
                let mut j = i + 2;
                while j < input.len() && input.chars().nth(j).unwrap() != '}' {
                    var_name.push(input.chars().nth(j).unwrap());
                    j += 1;
                }
                if j >= input.len() || input.chars().nth(j).unwrap() != '}' {
                    // If no closing bracket is found, treat as literal
                    output.push('$');
                    output.push('{');
                    i += 2;
                    continue;
                }
                // Replace with env variable value or an empty string if not set

                output.push_str(env::var(var_name).unwrap_or_default().as_str());
                i = j + 1; // Move past the '}'
            } else {
                // Extract variable name without '{' and '}'
                let mut var_name = String::new();
                let mut j = i + 1;
                while j < input.len() && !input.chars().nth(j).unwrap().is_whitespace() {
                    if input.chars().nth(j).unwrap() == '/' || input.chars().nth(j).unwrap() == '.'
                    {
                        break; // Stop at path separators
                    }
                    var_name.push(input.chars().nth(j).unwrap());
                    j += 1;
                }
                if !var_name.is_empty() {
                    output.push_str(env::var(var_name).unwrap_or_default().as_str());
                } else {
                    // If no variable name is found, just append the '$'
                    output.push('$');
                }
                i = j; // Move past the variable name
            }
        } else if input.chars().nth(i).unwrap() == '%' && i + 1 < input.len() {
            let mut var_name = String::new();
            let mut j = i + 1;
            while j < input.len() && input.chars().nth(j).unwrap() != '%' {
                var_name.push(input.chars().nth(j).unwrap());
                j += 1;
            }
            if j >= input.len() || input.chars().nth(j).unwrap() != '%' {
                // If no closing '%' is found, treat as literal
                output.push('%');
                i += 1;
                continue;
            }
            let var_name = var_name.to_uppercase();
            // Convert to uppercase for Windows compatibility
            if let Ok(value) = env::var(var_name.clone()) {
                output.push_str(&value);
            } else {
                // Variable not set, append nothing or handle as needed
            }
            i = j + 1; // Move past the '%'
        } else {
            output.push(input.chars().nth(i).unwrap());
            i += 1;
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_env() {
        env::set_var("TEST_ENV", "test");
        assert_eq!(replace_envs("This is a $TEST_ENV"), "This is a test");
    }

    #[test]
    fn test_multiple_envs() {
        env::set_var("TEST_ENV", "test");
        env::set_var("ANOTHER_ENV", "another");
        env::set_var("YET_ANOTHER_ENV", "yet another");
        assert_eq!(
            replace_envs("This is a ${TEST_ENV} and $ANOTHER_ENV and %YET_ANOTHER_ENV%"),
            "This is a test and another and yet another"
        );
    }
}
