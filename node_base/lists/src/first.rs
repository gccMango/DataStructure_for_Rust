// pub enum List {
//     Empty,
//     Elem(i32,Box<List>),
// }

// struct Node {
//     elem: i32,
//     next: List,
// }

// pub enum List {
//     Empty,
//     More(Box<Node>),
// }
use std::{mem };
pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            // next:self.head
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

// impl Drop for List {
//     fn drop(&mut self) {
//         let mut cur_link = mem::replace(&mut self.head, Link::Empty);

//         while let Link::More(mut boxed_node) = cur_link {
//             cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
//         }
//     }
// }
#[derive(PartialEq, Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}


#[derive(PartialEq, Debug)]
struct Node {
    elem: i32,
    next: Link,
}
// impl Drop for Link {
//     fn drop(&mut self) {
//         match *self {
//             Link::Empty =>  {},
//             Link::More(ref mut boxed_node) => {
//                 boxed_node.drop();
//             }
//         }
//     }
// }

// impl Drop for Box<Node> {
//     fn drop(&mut self) {
//         self.ptr.drop();
//         deallocate(self.ptr);
//     }
// }

// impl Drop for Node {
//     fn drop(&mut self) {
//         self.next.drop();
//     }
// }
#[cfg(test)]
mod tests {

    use super::{List,Link};

    #[test]
    fn it_makes_new() {
        let linked_list = List::new();
        assert_eq!(linked_list.head, Link::Empty);
    }
    
    #[test]
    fn push_someting() {
        let mut linked_list = List::new();

        linked_list.push(3);
        if let Link::More(x) = &linked_list.head {
            assert_eq!(x.elem, 3);
        }
    }

    #[test]
    fn pop_something() {
        let mut linked_list = List::new();

        linked_list.push(4);
        assert_eq!(linked_list.pop(), Some(4));
    }

    #[test]
    fn pop_none() {
        let mut linked_list= List::new();

        assert_eq!(linked_list.pop(),None);
    }

    #[test]
    fn push_pop() {
        let mut linked_list=List::new();
        linked_list.push(1);
        linked_list.push(2);
        linked_list.push(3);

        assert_eq!(linked_list.pop(), Some(3));
        assert_eq!(linked_list.pop(), Some(2));
        assert_eq!(linked_list.pop(), Some(1));
        assert_eq!(linked_list.pop(), None);
    }
}
