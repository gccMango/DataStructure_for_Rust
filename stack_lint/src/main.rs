#![allow(dead_code)]
#[derive(Debug)]
struct Stack<T> {
    item: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { item: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.item.push(item)
    }

    fn pop(&mut self) -> Option<T> {
        self.item.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.item.last()
    }
}

#[derive(Debug)]
struct Linter {
    stack: Stack<String>,
}
impl Linter {
    fn new(text: &str) -> Linter {
        Linter {
            stack: Stack {
                item: text
                    .chars()
                    .filter(|ch| *ch != ' ')
                    .map(|ch| ch.to_string())
                    .collect::<Vec<_>>(),
            },
        }
    }

    fn lint(&mut self) -> Result<(),()> {
        let mut store = Stack::new();
        let mut popped_opening_brace;
        for ch in self.stack.item.iter() {
            if self.is_opening_brace(ch) {
                store.push(ch);
            }
            if self.is_closing_brace(ch) {
                popped_opening_brace = store.pop();

                if let None = popped_opening_brace {
                    return Err(eprintln!("{ch}doesn't have opening here"));
                }

                if self.is_not_match(popped_opening_brace, ch) {
                    return Err(eprintln!("{ch}has mismatched opening brace"));
                }
            }
        }
        if let Some(item) = store.pop() {
            return Err(eprintln!("{item} does not have closing brace"));
        }
        Ok(())
    }
    fn is_opening_brace(&self,text: &String) -> bool {
        let mut flag = false;
        if "([{".contains(text) {
            flag = true
        }
        flag
    }
    fn is_closing_brace(&self,text: &String) -> bool {
        let mut flag = false;
        if ")]}".contains(text) {
            flag = true
        }
        flag
    }

    fn is_not_match(&self,cmp_opening: Option<&String>, cmp_closing: &String) -> bool {
        match cmp_opening.unwrap().as_str() {
            "(" if *cmp_closing == String::from(")") => false,
            "[" if *cmp_closing == String::from("]") => false,
            "{" if *cmp_closing == String::from("}") => false,
            _ => true,
        }
    }
}

fn main() {
    let mut linter = Linter::new("let x ={y = [1,2,3,]");
    println!("{linter:?}");
    println!("lint result {:?}",linter.lint());
} 
