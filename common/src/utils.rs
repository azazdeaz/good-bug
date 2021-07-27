use std::sync::{Arc, Mutex};

// TODO there must be an idiomatic way to do this
pub struct LastValue<T> {
    val: Vec<T>,
}
impl<T> LastValue<T> {
    pub fn new() -> Self {
        Self {
            val: Vec::with_capacity(1),
        }
    }
    pub fn set(&mut self, value: T) {
        self.val.clear();
        self.val.push(value);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.val.pop()
    }
}