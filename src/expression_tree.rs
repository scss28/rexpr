use std::{fmt::Debug, ptr};

pub trait Priority {
    fn priority(&self) -> usize;
}

pub trait Evaluate<T> {
    fn evaluate(&self, left: T, right: T) -> T;
}

pub enum ExpressionTree<T, Op> {
    Value(T),
    Expression {
        operator: Op,
        left: Box<ExpressionTree<T, Op>>,
        right: Box<ExpressionTree<T, Op>>,
    },
    Enclosed(Box<ExpressionTree<T, Op>>),
}

impl<T, Op> ExpressionTree<T, Op> {
    pub fn new_expression(
        left: ExpressionTree<T, Op>,
        operator: Op,
        right: ExpressionTree<T, Op>,
    ) -> Self {
        ExpressionTree::Expression {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn new_enclosed(tree: ExpressionTree<T, Op>) -> Self {
        ExpressionTree::Enclosed(Box::new(tree))
    }
}

impl<T, Op: Priority> ExpressionTree<T, Op> {
    pub fn append(&mut self, operator: Op, right: ExpressionTree<T, Op>) {
        match self {
            ExpressionTree::Value(_) | ExpressionTree::Enclosed(_) => unsafe {
                ptr::write(
                    self,
                    ExpressionTree::new_expression(ptr::read(self), operator, right),
                )
            },
            ExpressionTree::Expression {
                operator: self_operator,
                right: self_right,
                ..
            } => {
                if operator.priority() > self_operator.priority() {
                    self_right.append(operator, right);
                } else {
                    unsafe {
                        ptr::write(
                            self,
                            ExpressionTree::new_expression(ptr::read(self), operator, right),
                        )
                    }
                }
            }
        }
    }
}

impl<T, Op: Evaluate<T>> ExpressionTree<T, Op> {
    pub fn evaluate(self) -> T {
        match self {
            ExpressionTree::Value(value) => value,
            ExpressionTree::Expression {
                operator,
                left,
                right,
            } => operator.evaluate(left.evaluate(), right.evaluate()),
            ExpressionTree::Enclosed(tree) => tree.evaluate(),
        }
    }
}

impl<T: Debug, Op: Debug> Debug for ExpressionTree<T, Op> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(value) => write!(f, "{:?}", value),
            Self::Expression {
                operator,
                left,
                right,
            } => write!(f, "{:?} {:?} {:?}", left, operator, right),
            ExpressionTree::Enclosed(tree) => write!(f, "({:?})", tree),
        }
    }
}
