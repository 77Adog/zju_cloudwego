use std::cell::RefCell;

#[derive(Debug)]
pub struct MyStack<T> {
    stack: RefCell<Vec<T>>,
}

impl<T> MyStack<T> {

    pub fn new() -> MyStack<T> {
        MyStack {
            stack: RefCell::new(Vec::new())
        }
    }

    pub fn push(&self, item: T) {
        self.stack.borrow_mut().push(item);
    }

    pub fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }
}