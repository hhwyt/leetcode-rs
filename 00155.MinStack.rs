use std::collections::VecDeque;

struct MinStack {
    min: i32,
    stack: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            min: std::i32::MAX,
            stack: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        if x <= self.min {
            self.stack.push_front(self.min);
            self.min = x;
        }
        self.stack.push_front(x);
    }

    fn pop(&mut self) {
        if !self.stack.is_empty() {
            if self.min == self.stack.pop_front().unwrap() {
                self.min = self.stack.pop_front().unwrap();
            }
        }
    }

    fn top(&self) -> i32 {
        *self.stack.front().unwrap()
    }

    fn get_min(&self) -> i32 {
        self.min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut min_stack1 = MinStack::new();
        min_stack1.push(2);
        min_stack1.push(1);
        min_stack1.push(3);
        assert_eq!(min_stack1.top(), 3);
        assert_eq!(min_stack1.get_min(), 1);
        min_stack1.pop();
        assert_eq!(min_stack1.top(), 1);
        assert_eq!(min_stack1.get_min(), 1);
    }
}
