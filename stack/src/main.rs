struct Stack<T> {
    stack: Vec<T>
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn length(&self) -> usize {
        self.stack.len()
    }

    fn peek(&self) ->Option<&T>{
        self.stack.last()
    }
}
fn main() {
    let mut stack:Stack<&str> = Stack::new();
    stack.push("{");
    stack.push("{");
    stack.push("(");
    stack.push("(");
    stack.push(")");
    stack.push(")");
    stack.push("}");
    stack.push("}");
    let last_item = stack.pop();
    assert_eq!(last_item.unwrap(),"}");
    println!("next top item: {:?}",stack.peek());
    assert_eq!(stack.peek(),Some(&"}"));
}
