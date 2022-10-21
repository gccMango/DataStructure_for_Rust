#![allow(dead_code)]
#[derive(Debug)]
struct Queue<T>{
    item: Vec<T>
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { item: Vec::new() }
    }

    fn enqueue(&mut self, element: T) {
        self.item.push(element)
    }

    fn dequeue(&mut self) -> T {
        self.item.remove(0)
    }

    fn read(&self) ->Option<&T>{
        self.item.first()
    }
}

fn main() {
    let mut que_test = Queue::new();
    que_test.enqueue(5);
    que_test.enqueue(6);
    que_test.enqueue(7);
    println!("{:?}",que_test);
    que_test.dequeue();
    println!("{:?}",que_test);
    println!("{:?}",que_test.read());
    println!("{:?}",que_test);
}
