use crate::expression_tree::{Evaluate, Priority};

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Sub,
    Div,
    Mul,
    Pow,
    Mod,
    Xor,
    Or,
    And,
}

impl BinaryOperator {
    pub fn parse_next(bytes: &[u8], idx: &mut usize) -> Option<BinaryOperator> {
        match bytes.get(*idx)? {
            b'+' => {
                *idx += 1;
                Some(BinaryOperator::Add)
            }
            b'-' => {
                *idx += 1;
                Some(BinaryOperator::Sub)
            }
            b'*' => {
                *idx += 1;
                if bytes.get(*idx) == Some(&b'*') {
                    *idx += 1;
                    return Some(BinaryOperator::Pow);
                }

                Some(BinaryOperator::Mul)
            }
            b'/' => {
                *idx += 1;
                Some(BinaryOperator::Div)
            }
            b'%' => {
                *idx += 1;
                Some(BinaryOperator::Mod)
            }
            b'^' => {
                *idx += 1;
                Some(BinaryOperator::Xor)
            }
            b'|' => {
                *idx += 1;
                Some(BinaryOperator::Or)
            }
            b'&' => {
                *idx += 1;
                Some(BinaryOperator::And)
            }
            _ => None,
        }
    }
}

impl Priority for BinaryOperator {
    fn priority(&self) -> usize {
        match self {
            BinaryOperator::Add => 0,
            BinaryOperator::Sub => 0,
            BinaryOperator::Div => 1,
            BinaryOperator::Mul => 1,
            BinaryOperator::Mod => 1,
            BinaryOperator::Pow => 2,
            BinaryOperator::Or => 2,
            BinaryOperator::And => 2,
            BinaryOperator::Xor => 2,
        }
    }
}

impl Evaluate<f64> for BinaryOperator {
    fn evaluate(&self, left: f64, right: f64) -> f64 {
        match self {
            BinaryOperator::Add => left + right,
            BinaryOperator::Sub => left - right,
            BinaryOperator::Div => left / right,
            BinaryOperator::Mul => left * right,
            BinaryOperator::Pow => left.powf(right),
            BinaryOperator::Mod => left % right,
            BinaryOperator::Or => (left as i128 | right as i128) as f64,
            BinaryOperator::And => (left as i128 & right as i128) as f64,
            BinaryOperator::Xor => (left as i128 ^ right as i128) as f64,
        }
    }
}
