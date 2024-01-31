use std::fmt::Debug;

use crate::operator::Operator;

#[derive(Clone)]
pub enum ASTNode<T> {
    Value(T),
    Expression {
        operator: Operator<T>,
        left: Box<ASTNode<T>>,
        right: Box<ASTNode<T>>,
    },
}

impl<T: Clone> ASTNode<T> {
    pub fn new_expression(left: ASTNode<T>, operator: Operator<T>, right: ASTNode<T>) -> Self {
        ASTNode::Expression {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn append(&mut self, operator: Operator<T>, right: ASTNode<T>) {
        match self {
            ASTNode::Value(value) => {
                *self = ASTNode::new_expression(ASTNode::Value(value.clone()), operator, right);
            }
            ASTNode::Expression {
                operator: self_operator,
                right: self_right,
                ..
            } => {
                if operator.priority() > self_operator.priority() {
                    self_right.append(operator, right);
                } else {
                    *self = ASTNode::new_expression(self.clone(), operator, right);
                }
            }
        }
    }

    pub fn evaluate(self) -> T {
        match self {
            ASTNode::Value(value) => value,
            ASTNode::Expression {
                operator,
                left,
                right,
            } => operator.evaluate(left.evaluate(), right.evaluate()),
        }
    }
}

impl<T: Debug> Debug for ASTNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(value) => write!(f, "{:?}", value),
            Self::Expression {
                operator,
                left,
                right,
            } => write!(f, "{:?} -> [ {:?} | {:?} ]", operator, left, right),
        }
    }
}
