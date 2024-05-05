#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenState {
    None,
    Number,
    Operator,
    Parenthesis,
}

fn is_operator(op: &str) -> bool {
    match op {
        "+" | "-" | "*" | "/" | "**" | "-u" => true,
        _ => false,
    }
}

fn get_operator_precedence(op: &str) -> i32 {
    match op {
        "+" | "-" => 1,
        "*" | "/" => 2,
        "**" => 3,
        "-u" => 4, //unary minus
        _ => 0,
    }
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn tokenizer(expr: String) -> Vec<String> {
    /*
    tokens are: floating point numbers, operators (+,-,*,/,**) and parentheses
    */
    let mut tokens: Vec<String> = Vec::new();
    let mut num_buffer: String = String::new();
    let mut current_state: TokenState = TokenState::None;
    let mut past_state: TokenState;
    let mut skip = 0;
    // make it state machine style
    for i in 0..expr.len() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        let c = expr.chars().nth(i).unwrap();
        past_state = current_state;
        match c {
            //match an operator
            '+' | '-' | '*' | '/' => {
                current_state = TokenState::Operator;
                //it may be power operator **, check next character
                if i < expr.len() - 1 && expr.chars().nth(i + 1).unwrap() == '*' {
                    tokens.push(c.to_string() + "*");
                    skip = 1;
                } else {
                    tokens.push(c.to_string());
                }
            }
            //match a parenthesis
            '(' | ')' => {
                current_state = TokenState::Parenthesis;

                if c == '(' && past_state == TokenState::Operator{
                    if tokens.len() > 0 && tokens.last().unwrap() == "-"{
                        if tokens.len() <= 1 {
                            //is unary minus
                            tokens.pop();
                            tokens.push("-u".to_string());
                        }
                        else if tokens.len() > 1 && is_operator(&tokens[tokens.len() - 2]){
                            //is unary minus
                            tokens.pop();
                            tokens.push("-u".to_string());
                        }
                    }
                }

                tokens.push(c.to_string());
            }
            //match a number
            '.' | '0'..='9' => {
                current_state = TokenState::Number;
                //if past state was operator, check if the token was + or -, then check if the token before it was also an operator,
                //if so, merge the + or - with the number buffer and remove the last token
                if past_state == TokenState::Operator
                    || past_state == TokenState::Parenthesis && tokens.last().unwrap() == "("
                {
                    if tokens.last().unwrap() == "+" || tokens.last().unwrap() == "-" {
                        //check if it is the first token ever, if it is, merge it with the number buffer then remove it
                        if tokens.len() == 1 {
                            num_buffer = tokens.pop().unwrap() + &num_buffer;
                        }
                        //otherwise check if there were more than 1 tokens, and if the one before was an operator
                        else if tokens.len() > 1 {
                            if is_operator(&tokens[tokens.len() - 2])
                                || tokens[tokens.len() - 2] == "("
                            {
                                num_buffer = tokens.pop().unwrap() + &num_buffer;
                            }
                        }
                    }
                }
                //iterate over the string and add each number character to the number buffer
                let mut explored_digits_count = 0;
                for j in i..expr.len() {
                    let c2 = expr.chars().nth(j).unwrap();
                    if c2 == '.' || c2.is_digit(10) {
                        num_buffer.push(c2);
                        explored_digits_count += 1;
                    } else {
                        break;
                    }
                }
                tokens.push(num_buffer.clone());
                num_buffer.clear();
                skip = if explored_digits_count > 0 {
                    explored_digits_count - 1
                } else {
                    explored_digits_count
                };
            }
            //default case, do nothing
            _ => {}
        }
    }
    //merge + and - with numbers whenever an operator is followed by a number
    //return the tokens
    return tokens;
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn infix_to_postfix_tokens(tokens: Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut operators: Vec<String> = Vec::new();

    for token in tokens {
        match token.as_str() {
            "(" => {
                operators.push(token);
            }
            ")" => {
                while operators.len() > 0 && operators[operators.len() - 1] != "(" {
                    output.push(operators.pop().unwrap());
                }
                operators.pop();
            }
            "+" | "-" | "*" | "/" | "**" | "-u" => {
                //if the current token to operators only if it has a greater precedence compared to the last operator in the stack
                while operators.len() > 0
                    && get_operator_precedence(&token)
                        <= get_operator_precedence(&operators[operators.len() - 1])
                {
                    output.push(operators.pop().unwrap());
                }
                operators.push(token);
            }
            _ => {
                output.push(token);
            }
        }
    }
    while operators.len() > 0 {
        output.push(operators.pop().unwrap());
    }

    output
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn eval_float64(expr: String) -> f64 {
    let tokens = tokenizer(expr);
    let postfix_tokens = infix_to_postfix_tokens(tokens);
    let mut stack: Vec<f64> = Vec::new();
    for token in postfix_tokens {
        match token.as_str() {
            "+" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            }
            "-" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b - a);
            }
            "*" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a * b);
            }
            "/" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b / a);
            }
            "**" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b.powf(a));
            }
            "-u" => {
                let a = stack.pop().unwrap();
                stack.push(-a);
            }
            _ => {
                stack.push(token.parse::<f64>().unwrap());
            }
        }
    }
    stack.pop().unwrap()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn eval_float64_str(expr: String) -> String {
    eval_float64(expr).to_string()
}

#[cfg(test)]
mod tests;
