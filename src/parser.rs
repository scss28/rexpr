use crate::{ast::ASTNode, operator::Operator};

#[derive(Debug, Clone)]
pub struct ParseError(pub usize, pub &'static str);

pub fn parse_expression(str: &str) -> Result<ASTNode<f64>, ParseError> {
    let arr = str.chars().collect::<Box<[char]>>();
    let mut idx = next_not_whitespace(&arr, 0usize);
    let left = parse_ast(&arr, &mut idx)?;
    if idx == arr.len() {
        return Ok(left);
    }

    let mut ast = ASTNode::new_expression(
        left,
        parse_operator(&arr, &mut idx)?,
        parse_ast(&arr, &mut idx)?,
    );

    while idx < arr.len() {
        ast.append(parse_operator(&arr, &mut idx)?, parse_ast(&arr, &mut idx)?);
    }

    Ok(ast)
}

fn next_not_whitespace(arr: &[char], mut idx: usize) -> usize {
    loop {
        let Some(char) = arr.get(idx) else {
            break arr.len();
        };

        if char.is_whitespace() {
            idx += 1;
            continue;
        }

        break idx;
    }
}

fn parse_ast(arr: &[char], idx: &mut usize) -> Result<ASTNode<f64>, ParseError> {
    if *idx >= arr.len() {
        return Err(ParseError(*idx, "Expected a number, found nothing."));
    }

    let start = *idx;
    if arr[*idx] == '(' {
        let mut open_count = 1usize;
        loop {
            *idx += 1;

            let Some(char) = arr.get(*idx) else {
                return Err(ParseError(start, "Mismatched parenthesis."));
            };

            match *char {
                ')' => {
                    open_count -= 1usize;
                    if open_count == 0 {
                        break;
                    }
                }
                '(' => {
                    open_count += 1;
                }
                _ => (),
            }
        }

        let ast = parse_expression(&arr[(start + 1)..*idx].iter().collect::<String>())
            .map_err(|err| ParseError(err.0 + start + 1, err.1))?;

        *idx = next_not_whitespace(arr, *idx + 1);
        return Ok(ast);
    }

    loop {
        let Some(char) = arr.get(*idx) else {
            break;
        };

        if !char.is_numeric() && *char != '.' {
            break;
        }

        *idx += 1;
    }

    let value = arr[start..*idx]
        .iter()
        .collect::<String>()
        .parse::<f64>()
        .map_err(|_| ParseError(start, "Expected a number."))?;

    *idx = next_not_whitespace(arr, *idx);
    Ok(ASTNode::Value(value))
}

fn parse_operator(arr: &[char], idx: &mut usize) -> Result<Operator<f64>, ParseError> {
    fn get_operator(char: char) -> Option<Operator<f64>> {
        match char {
            '+' => Some(Operator::new(3, |a, b| a + b)),
            '-' => Some(Operator::new(3, |a, b| a - b)),
            '*' => Some(Operator::new(4, |a, b| a * b)),
            '/' => Some(Operator::new(4, |a, b| a / b)),
            '%' => Some(Operator::new(4, |a, b| a % b)),
            '^' => Some(Operator::new(5, |a: f64, b| a.powf(b))),
            _ => None,
        }
    }

    let Some(char) = arr.get(*idx) else {
        return Err(ParseError(*idx, "Expected an operator, found nothing."));
    };

    match get_operator(*char) {
        Some(operator) => {
            *idx = next_not_whitespace(&arr, *idx + 1usize);
            Ok(operator)
        }
        None => Err(ParseError(*idx, "Expected an operator.")),
    }
}
