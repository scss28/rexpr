use crate::{binary_operator::BinaryOperator, expression_tree::ExpressionTree};

#[derive(Debug, Clone)]
pub struct ParseError(pub usize, pub String);

pub fn parse_expression(bytes: &[u8]) -> Result<ExpressionTree<f64, BinaryOperator>, ParseError> {
    let mut idx = next_not_whitespace(bytes, 0usize);
    let mut tree = parse_tree(bytes, &mut idx)?;
    if idx == bytes.len() {
        return Ok(tree);
    }

    while idx < bytes.len() {
        let operator = BinaryOperator::parse_next(bytes, &mut idx)
            .ok_or(ParseError(idx, String::from("Expected an operator.")))?;

        idx = next_not_whitespace(bytes, idx);
        tree.append(operator, parse_tree(bytes, &mut idx)?);
    }

    Ok(tree)
}

/// Returns the first non-white-space character index.
fn next_not_whitespace(arr: &[u8], mut idx: usize) -> usize {
    while let Some(byte) = arr.get(idx) {
        if byte.is_ascii_whitespace() {
            idx += 1;
            continue;
        }

        return idx;
    }

    arr.len()
}

/// Parses a "value" ie. either `f64` or recursively an expression inside parenthesis.
/// Then moves `idx` to another non-white-space character.
fn parse_tree(
    bytes: &[u8],
    idx: &mut usize,
) -> Result<ExpressionTree<f64, BinaryOperator>, ParseError> {
    if *idx >= bytes.len() {
        return Err(ParseError(
            *idx,
            String::from("Expected a number, found nothing."),
        ));
    }

    let start = *idx;
    if bytes[*idx] == b'(' {
        let mut open_count = 1usize;
        loop {
            *idx += 1;
            let Some(byte) = bytes.get(*idx) else {
                return Err(ParseError(start, String::from("Mismatched parenthesis.")));
            };

            match *byte {
                b')' => {
                    open_count -= 1usize;
                    if open_count == 0 {
                        break;
                    }
                }
                b'(' => {
                    open_count += 1;
                }
                _ => (),
            }
        }

        let tree = match parse_expression(&bytes[(start + 1)..*idx]) {
            Ok(tree) => ExpressionTree::new_enclosed(tree),
            Err(err) => return Err(ParseError(err.0 + start + 1, err.1)),
        };

        *idx = next_not_whitespace(bytes, *idx + 1);
        return Ok(tree);
    }

    match bytes.get(*idx) {
        Some(byte) if *byte == b'-' || byte.is_ascii_digit() => (),
        Some(byte) => {
            return Err(ParseError(
                *idx,
                format!("Expected a number, found \"{}\".", *byte as char),
            ))
        }
        None => {
            return Err(ParseError(
                *idx,
                String::from("Expected a number, found nothing."),
            ))
        }
    }

    *idx += 1;
    loop {
        let Some(byte) = bytes.get(*idx) else {
            break;
        };

        if !byte.is_ascii_digit() && *byte != b'.' {
            break;
        }

        *idx += 1;
    }

    let value = std::str::from_utf8(&bytes[start..*idx])
        .map_err(|_| ParseError(start, String::from("Non utf-8 characters.")))?
        .parse::<f64>()
        .map_err(|_| ParseError(start, String::from("Expected a number.")))?;

    *idx = next_not_whitespace(bytes, *idx);
    Ok(ExpressionTree::Value(value))
}
