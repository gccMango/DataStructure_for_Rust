use std::cell::{RefCell, Ref};
use std::rc::Rc;
#[derive(Debug)]
struct Node {
    value: String,
    next: Link,
}

type Link = Option<Rc<RefCell<Node>>>;

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}

#[derive(Debug)]
pub struct LinkedList {
    head: Link,
    tail: Link,
    pub length: usize,
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn push(&mut self, value: String) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        };
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("An error has occured.")
                .into_inner()
                .value
        })
    }

    //  pub fn read(&mut self,num:usize)->String {
    //     let mut current_index = 0;
    //     while current_index <num {
    //         if let Some(next) = self.head.take() {
    //             self.head = Some(next);
    //         }
    //         current_index+=1;
    //     }
    //     Rc::try_unwrap(self.head.unwrap()).ok().as_ref().
        
    // }
}
fn main() {
    let mut link_list = LinkedList::new();
    println!("{}", link_list.length);
    println!("{:#?}", link_list);
    
    link_list.push("A".to_string());
    link_list.push("B".to_string());
    link_list.push("C".to_string());
    // println!("read {},{:?}",1,link_list.read(1));
    // println!("{:#?}", link_list);
    // println!("linked list head : {:#?}", link_list.head);
    // println!("linked list tail : {:#?}", link_list.tail);

    
    // println!("{}", link_list.length);
    // println!("pop item=>{:?}",link_list.pop());
    // println!("{:#?}", link_list);
    // println!("{}", link_list.length);
}
