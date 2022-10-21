#[derive(Debug, Clone)]
pub struct Stack<T> {
    stack: Vec<T>,
    pub sp: u16
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            stack: Vec::new(),
            sp: 0
        }
    }

    pub fn push(&mut self, value: T) {
        self.stack.push(value);
        self.sp += 1;
    }

    pub fn pop(&mut self) -> T {
        self.sp -= 1;
        self.stack.pop().expect("Stack underflow")
    }
}