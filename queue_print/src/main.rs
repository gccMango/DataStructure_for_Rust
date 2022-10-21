#![allow(dead_code)]
#[derive(Debug)]
struct Queue<T> {
    item: Vec<T>,
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

    fn read(&self) -> Option<&T> {
        self.item.first()
    }
}

struct PrintManager {
    queue_list: Queue<String>,
}

impl PrintManager {
    fn new() -> Self {
        PrintManager { queue_list: Queue::new() }
    }

    fn queue_print_job(&mut self, document: &str){
        self.queue_list.enqueue(document.to_string())
    }

    fn run(&mut self) {
        while let Some(item) = self.queue_list.read(){
            Self::print(self.queue_list.dequeue())
        }
    }

    fn print(document: String) {
        println!("Job is complited : {}",document);
    }
}
fn main() {
    let mut print_manager = PrintManager::new();
    print_manager.queue_print_job("First Document");
    print_manager.queue_print_job("Second Document");
    print_manager.queue_print_job("Third Document");

    print_manager.run();
}
