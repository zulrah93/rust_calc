use std::env::args;

fn is_operator(token: &str) -> bool {
    match token {
        "+" | "-" | "*" | "/" => true,
        _ => false,
    }
}

fn evaluate(a: i64, b: i64, operator: &str) -> Option<i64> {
    match operator {
        "+" => Some(a + b),
        "-" => Some(a - b),
        "*" => Some(a * b),
        "/" => Some(a / b),
        _ => None,
    }
}

fn parse_prefix_expression(expression: &String) -> Option<i64> {

    let mut stack: Vec<i64> = vec![];
    let tokens = expression.split(' ').rev().collect::<Vec<&str>>();
    
    for token in tokens {
        if is_operator(token) {
            if let Some(operand_a) = stack.pop() {
                if let Some(operand_b) = stack.pop() {
                    stack.push(evaluate(operand_a, operand_b, token).unwrap());
                } else {
                    return None; // Expression must be invalid
                }
            } else {
                return None; // Expression must be invalid
            }
        } else {
            
            if let Ok(operand) = token.parse::<i64>() {
                stack.push(operand);
            }
            else {
                return None; // Expression must be invalid
            }
        }
    }

    stack.pop()

}

fn get_expression_from_argument() -> String {
    args().nth(1).unwrap_or_default()
}

fn main() {
    let expression = get_expression_from_argument();
    println!(
        "eval('{}') = {}",
        expression,
        if let Some(result) = parse_prefix_expression(&expression) {
            format!("{}", result)
        } else {
            "NaN".to_string() // We are going to treat NaN usually for floating point numbers as a way of showing to the user the expression was invalid or cannot compute
        }
    );
}


#[test]
fn test_case_1() {
    let expression = "* + 6 9 - 3 1".to_string();
    assert_eq!(parse_prefix_expression(&expression), Some(30));
}
