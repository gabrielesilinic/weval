use super::*;

#[test]
fn tokenizer_defaultok() {
    let expr = "1+2+-5";
    let tokens = tokenizer(expr.to_string());
    assert_eq!(tokens, vec!["1", "+", "2", "+", "-5"]);
}
#[test]
fn tokenizer_defaultok2() {
    let expr = "1+2+-5*3";
    let tokens = tokenizer(expr.to_string());
    assert_eq!(tokens, vec!["1", "+", "2", "+", "-5", "*", "3"]);
}
#[test]
fn tokenizer_defaultok3() {
    let expr = "1+2+-5*3/4";
    let tokens = tokenizer(expr.to_string());
    assert_eq!(tokens, vec!["1", "+", "2", "+", "-5", "*", "3", "/", "4"]);
}
#[test]
fn tokenizer_parerentheses() {
    let expr = "+43**(1+2)**-23.45";
    let tokens = tokenizer(expr.to_string());
    assert_eq!(
        tokens,
        vec!["+43", "**", "(", "1", "+", "2", ")", "**", "-23.45"]
    );
}
#[test]
fn tokenizer_parerentheses2() {
    let expr = "34+(1+2**3)*3";
    let tokens = tokenizer(expr.to_string());
    assert_eq!(
        tokens,
        vec!["34", "+", "(", "1", "+", "2", "**", "3", ")", "*", "3"]
    );
}
#[test]
fn tokenizer_parerentheses3() {
    let expr = "34**(1+2**3)*-3/4";
    let tokens = tokenizer(expr.to_string());
    assert_eq!(
        tokens,
        vec!["34", "**", "(", "1", "+", "2", "**", "3", ")", "*", "-3", "/", "4"]
    );
}
#[test]
fn infix_to_postfix_tokens_defaultok() {
    //1+2*(3-4)**2+5**(3+4+5)
    //1 2 3 4 - 2 ** * + 5 3 4 + 5 + **
    let tokens: Vec<String> = vec![
        "1", "+", "2", "*", "(", "3", "-", "4", ")", "**", "2", "+", "5", "**", "(", "3", "+", "4",
        "+", "5", ")",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();
    let postfix_tokens = infix_to_postfix_tokens(tokens.clone());
    assert_eq!(
        postfix_tokens,
        vec!["1", "2", "3", "4", "-", "2", "**", "*", "+", "5", "3", "4", "+", "5", "+", "**", "+"]
    );
}
#[test]
fn infix_to_postfix_tokens_sum() {
    let tokens: Vec<String> = vec!["1", "+", "2"].iter().map(|&s| s.to_string()).collect();
    let postfix_tokens = infix_to_postfix_tokens(tokens.clone());
    assert_eq!(postfix_tokens, vec!["1", "2", "+"]);
}
#[test]
fn infix_to_postfix_tokens_sub() {
    let tokens: Vec<String> = vec!["1", "-", "2"].iter().map(|&s| s.to_string()).collect();
    let postfix_tokens = infix_to_postfix_tokens(tokens.clone());
    assert_eq!(postfix_tokens, vec!["1", "2", "-"]);
}
#[test]
fn infix_to_postfix_tokens_mul() {
    let tokens: Vec<String> = vec!["1", "*", "2"].iter().map(|&s| s.to_string()).collect();
    let postfix_tokens = infix_to_postfix_tokens(tokens.clone());
    assert_eq!(postfix_tokens, vec!["1", "2", "*"]);
}
#[test]
fn infix_to_postfix_tokens_div() {
    let tokens: Vec<String> = vec!["1", "/", "2"].iter().map(|&s| s.to_string()).collect();
    let postfix_tokens = infix_to_postfix_tokens(tokens.clone());
    assert_eq!(postfix_tokens, vec!["1", "2", "/"]);
}
#[test]
fn infix_to_postfix_tokens_pow() {
    let tokens: Vec<String> = vec!["1", "**", "2"]
        .iter()
        .map(|&s| s.to_string())
        .collect();
    let postfix_tokens = infix_to_postfix_tokens(tokens.clone());
    assert_eq!(postfix_tokens, vec!["1", "2", "**"]);
}
#[test]
fn infix_to_postfix_tokens_sum_mul() {
    let tokens: Vec<String> = vec!["1", "+", "2", "*", "3"]
        .iter()
        .map(|&s| s.to_string())
        .collect();
    let postfix_tokens = infix_to_postfix_tokens(tokens.clone());
    assert_eq!(postfix_tokens, vec!["1", "2", "3", "*", "+"]);
}
#[test]
fn infix_to_postfix_tokens_mul_sum() {
    let tokens: Vec<String> = vec!["1", "*", "2", "+", "3"]
        .iter()
        .map(|&s| s.to_string())
        .collect();
    let postfix_tokens = infix_to_postfix_tokens(tokens.clone());
    assert_eq!(postfix_tokens, vec!["1", "2", "*", "3", "+"]);
}
#[test]
fn infix_to_postfix_tokens_sum_sub() {
    let tokens: Vec<String> = vec!["1", "+", "2", "-", "3"]
        .iter()
        .map(|&s| s.to_string())
        .collect();
    let postfix_tokens = infix_to_postfix_tokens(tokens.clone());
    assert_eq!(postfix_tokens, vec!["1", "2", "+", "3", "-"]);
}
#[test]
fn infix_to_postfix_tokens_sub_sum() {
    let tokens: Vec<String> = vec!["1", "-", "2", "+", "3"]
        .iter()
        .map(|&s| s.to_string())
        .collect();
    let postfix_tokens = infix_to_postfix_tokens(tokens.clone());
    assert_eq!(postfix_tokens, vec!["1", "2", "-", "3", "+"]);
}
#[test]
fn infix_to_postfix_tokens_sum_div() {
    let tokens: Vec<String> = vec!["1", "+", "2", "/", "3"]
        .iter()
        .map(|&s| s.to_string())
        .collect();
    let postfix_tokens = infix_to_postfix_tokens(tokens.clone());
    assert_eq!(postfix_tokens, vec!["1", "2", "3", "/", "+"]);
}
#[test]
fn infix_to_postfix_tokens_div_sum() {
    let tokens: Vec<String> = vec!["1", "/", "2", "+", "3"]
        .iter()
        .map(|&s| s.to_string())
        .collect();
    let postfix_tokens = infix_to_postfix_tokens(tokens.clone());
    assert_eq!(postfix_tokens, vec!["1", "2", "/", "3", "+"]);
}
#[test]
fn infix_to_postfix_tokens_sum_pow() {
    let tokens: Vec<String> = vec!["1", "+", "2", "**", "3"]
        .iter()
        .map(|&s| s.to_string())
        .collect();
    let postfix_tokens = infix_to_postfix_tokens(tokens.clone());
    assert_eq!(postfix_tokens, vec!["1", "2", "3", "**", "+"]);
}
#[test]
fn infix_to_postfix_tokens_pow_sum() {
    let tokens: Vec<String> = vec!["1", "**", "2", "+", "3"]
        .iter()
        .map(|&s| s.to_string())
        .collect();
    let postfix_tokens = infix_to_postfix_tokens(tokens.clone());
    assert_eq!(postfix_tokens, vec!["1", "2", "**", "3", "+"]);
}
#[test]
fn infix_to_postfix_tokens_sub_mul() {
    let tokens: Vec<String> = vec!["1", "-", "2", "*", "3"]
        .iter()
        .map(|&s| s.to_string())
        .collect();
    let postfix_tokens = infix_to_postfix_tokens(tokens.clone());
    assert_eq!(postfix_tokens, vec!["1", "2", "3", "*", "-"]);
}
#[test]
fn infix_to_postfix_tokens_div_pow() {
    let tokens: Vec<String> = vec!["1", "/", "2", "**", "3"]
        .iter()
        .map(|&s| s.to_string())
        .collect();
    let postfix_tokens = infix_to_postfix_tokens(tokens.clone());
    assert_eq!(postfix_tokens, vec!["1", "2", "3", "**", "/"]);
}
#[test]
fn infix_to_postfix_tokens_pow_div() {
    let tokens: Vec<String> = vec!["1", "**", "2", "/", "3"]
        .iter()
        .map(|&s| s.to_string())
        .collect();
    let postfix_tokens = infix_to_postfix_tokens(tokens.clone());
    assert_eq!(postfix_tokens, vec!["1", "2", "**", "3", "/"]);
}
#[test]
fn infix_to_postfix_tokens_sum_mul_div() {
    let tokens: Vec<String> = vec!["1", "+", "2", "*", "3", "/", "4"]
        .iter()
        .map(|&s| s.to_string())
        .collect();
    let postfix_tokens = infix_to_postfix_tokens(tokens.clone());
    assert_eq!(postfix_tokens, vec!["1", "2", "3", "*", "4", "/", "+"]);
}
#[test]
fn infix_to_postfix_tokens_sum_div_mul() {
    let tokens: Vec<String> = vec!["1", "+", "2", "/", "3", "*", "4"]
        .iter()
        .map(|&s| s.to_string())
        .collect();
    let postfix_tokens = infix_to_postfix_tokens(tokens.clone());
    assert_eq!(postfix_tokens, vec!["1", "2", "3", "/", "4", "*", "+"]);
}
#[test]
fn eval_float64_defaultok() {
    let expr = "1+2*(3-4)**2+5**(3+4+5)";
    let expected = 1.0 + 2.0 * (3.0f64 - 4.0f64).powf(2.0) + 5.0f64.powf(3.0 + 4.0 + 5.0);
    let result = eval_float64(expr.to_string());
    assert_eq!(result, expected);
}
#[test]
fn eval_float64_sum() {
    let expr = "1+2";
    let expected = 1.0 + 2.0;
    let result = eval_float64(expr.to_string());
    assert_eq!(result, expected);
}
#[test]
fn eval_float64_sub() {
    let expr = "1-2";
    let expected = 1.0 - 2.0;
    let result = eval_float64(expr.to_string());
    assert_eq!(result, expected);
}
#[test]
fn eval_float64_mul() {
    let expr = "1*2";
    let expected = 1.0 * 2.0;
    let result = eval_float64(expr.to_string());
    assert_eq!(result, expected);
}
#[test]
fn eval_float64_div() {
    let expr = "1/2";
    let expected = 1.0 / 2.0;
    let result = eval_float64(expr.to_string());
    assert_eq!(result, expected);
}
#[test]
fn eval_float64_pow() {
    let expr = "1**2";
    let expected = 1.0f64.powf(2.0);
    let result = eval_float64(expr.to_string());
    assert_eq!(result, expected);
}
#[test]
fn eval_float64_sum_mul() {
    let expr = "1+2*3";
    let expected = 1.0 + 2.0 * 3.0;
    let result = eval_float64(expr.to_string());
    assert_eq!(result, expected);
}
#[test]
fn eval_float64_mul_sum() {
    let expr = "1*2+3";
    let expected = 1.0 * 2.0 + 3.0;
    let result = eval_float64(expr.to_string());
    assert_eq!(result, expected);
}
#[test]
fn eval_float64_sum_sub() {
    let expr = "1+2-3";
    let expected = 1.0 + 2.0 - 3.0;
    let result = eval_float64(expr.to_string());
    assert_eq!(result, expected);
}
#[test]
fn eval_float64_mul_pow() {
    let expr = "1.4*2**3";
    let expected = 1.4 * 2.0f64.powf(3.0);
    let result = eval_float64(expr.to_string());
    assert_eq!(result, expected);
}
#[test]
fn eval_float64_pow_sub() {
    let expr = "1**2-3";
    let expected = 1.0f64.powf(2.0) - 3.0;
    let result = eval_float64(expr.to_string());
    assert_eq!(result, expected);
}
#[test]
fn eval_float64_complexok() {
    let expr = "((3 + 5 * 2) / (4 - 7 * (2 + 3)) + 8) / (6 * (4 + 2 ** 3) - 9) + (5 ** 2 - (2 + 3 * (4 / 2)))";
    let expected = ((3.0 + 5.0 * 2.0) / (4.0 - 7.0 * (2.0 + 3.0)) + 8.0)
        / (6.0 * (4.0 + 2.0f64.powf(3.0)) - 9.0)
        + (5.0f64.powf(2.0) - (2.0 + 3.0 * (4.0 / 2.0)));
    let result = eval_float64(expr.to_string());
    assert_eq!(result, expected);
}
#[test]
fn eval_float64_complexok2() {
    let expr = "((3 + 5 * 2) / (4 - 7 * (2 + 3)) + 8) / (6 * (4 + 2 ** 3) - 9) ** (5 ** 2 - (2 + 3 * (4 / 2)))";
    let expected = ((3.0 + 5.0 * 2.0) / (4.0 - 7.0 * (2.0 + 3.0)) + 8.0)
        / (6.0 * (4.0 + 2.0f64.powf(3.0)) - 9.0).powf(5.0f64.powf(2.0) - (2.0 + 3.0 * (4.0 / 2.0)));
    let result = eval_float64(expr.to_string());
    assert_eq!(result, expected);
}
#[test]
fn eval_float64_floatcomplexok() {
    let expr = "((3.5 + 5.2 * 2.3) / (4.1 - 7.9 * (2.8 + 3.7)) + 8.6) / (6.4 * (4.2 + 2.3 ** 3.1) - 9.7) ** (5.3 ** 2.2 - (2.1 + 3.4 * (4.2 / 2.3)))";
    let expected = ((3.5 + 5.2 * 2.3) / (4.1 - 7.9 * (2.8 + 3.7)) + 8.6)
        / (6.4 * (4.2 + 2.3f64.powf(3.1)) - 9.7).powf(5.3f64.powf(2.2) - (2.1 + 3.4 * (4.2 / 2.3)));
    let result = eval_float64(expr.to_string());
    assert_eq!(result, expected);
}

#[test]
fn eval_float64_str_defaultok() {
    let expr = "1+2*(3-4)**2+5**(3+4+5)";
    let expected =
        (1.0 + 2.0 * (3.0f64 - 4.0f64).powf(2.0) + 5.0f64.powf(3.0 + 4.0 + 5.0)).to_string();
    let result = eval_float64_str(expr.to_string());
    assert_eq!(result, expected);
}
#[test]
fn eval_float64_str_floatok() {
    let expr = "54*0.7";
    let expected = (54.0f64 * 0.7f64).to_string();
    let result = eval_float64_str(expr.to_string());
    assert_eq!(result, expected);
}

#[test]
fn unary_minus_tokenizer() {
    let expr = "- 1 + 2 * - (2 * 2)";
    let tokens: Vec<String> = tokenizer(expr.to_string());
    assert_eq!(
        tokens,
        vec!["-1", "+", "2", "*", "-u", "(", "2", "*", "2", ")"]
    );
}

#[test]
fn unary_minus_at_start() {
    let expr = "- (1 + 2) * 3";
    let tokens = tokenizer(expr.to_string());
    assert_eq!(
        tokens,
        vec!["-u", "(", "1", "+", "2", ")", "*", "3"]
    );
}


#[test]
fn infix_to_postfix_tokens_unary_minus_at_start() {
    let tokens: Vec<String> = vec!["-u", "(", "1", "+", "2", ")", "*", "3"]
        .iter()
        .map(|&s| s.to_string())
        .collect();
    let postfix_tokens = infix_to_postfix_tokens(tokens.clone());
    assert_eq!(postfix_tokens, vec!["1", "2", "+", "-u", "3", "*"]);
}

#[test]
fn infix_to_postfix_tokens_complex_unary_minus() {
    let tokens: Vec<String> = vec![
        "3", "+", "4", "-", "-u", "(", "2", "*", "2", ")", "+", "-u", "(", "2", "+", "1", ")"
    ].iter().map(|&s| s.to_string()).collect();
    
    let postfix_tokens = infix_to_postfix_tokens(tokens.clone());
    assert_eq!(
        postfix_tokens, 
        vec!["3", "4", "+", "2", "2", "*", "-u", "-", "2", "1", "+", "-u", "+"]
    );
}

#[test]
fn eval_float64_unary_minus_complex1() {
    let expr = "3 + 4 - -(2 * 2) + - (2 + 1)";
    let expected = 3.0 + 4.0 - -(2.0 * 2.0) + - (2.0 + 1.0);
    let result = eval_float64(expr.to_string());
    assert_eq!(result, expected);
}

#[test]
fn eval_float64_unary_minus_complex2() {
    let expr = "- (1 + 2) * 3";
    let expected = -(1.0 + 2.0) * 3.0;
    let result = eval_float64(expr.to_string());
    assert_eq!(result, expected);
}
