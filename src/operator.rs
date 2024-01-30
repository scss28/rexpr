#[derive(Clone, Copy)]
pub struct Operator<T> {
    priority: usize,
    evaluation_func: fn(T, T) -> T,
}

impl<T> Operator<T> {
    pub fn new(priority: usize, evaluation_func: fn(T, T) -> T) -> Self {
        Self {
            priority,
            evaluation_func,
        }
    }

    pub fn priority(&self) -> usize {
        self.priority
    }

    pub fn evaluate(self, a: T, b: T) -> T {
        (self.evaluation_func)(a, b)
    }
}
