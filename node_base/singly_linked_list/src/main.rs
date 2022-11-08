#[derive(Debug)]
struct ListNodeValue<T> {
    item: T,
    next: Box<ListNode<T>>,
}

impl<T> ListNodeValue<T>{
    fn new(item:T,next:Box<ListNode<T>>)->Self {
        Self { item: item, next: next }
    }
}

#[derive(Debug)]
enum ListNode<T> {
    Empty,
    NonEmpty(ListNodeValue<T>),
}

impl<T> ListNode<T>{
    fn new(item:T, next:Box<ListNode<T>>)->Self {
        ListNode::NonEmpty(ListNodeValue::new(item, next))
    }
    
    fn take(&mut self) -> Self {
        let mut cur=Self::Empty;
        std::mem::swap(&mut cur, self);
        cur
    }
    
}

#[derive(Debug)]
pub struct SinglyLinkedList<T> {
    head: Box<ListNode<T>>,
    size: usize,
}

impl<T> SinglyLinkedList<T>{
    pub fn new() ->Self{
        Self{
            head: Box::new(ListNode::Empty),
            size: 0,
        }
    }
    
    pub fn len(&self) -> usize {
        self.size
    }
    pub fn push(&mut self, item:T) {
        let cur_head = self.head.take();
        let new_node = Box::new(ListNode::new(item,Box::new(cur_head)));
        
        self.head= new_node;
        self.size+=1;

    }

    pub fn pop(&mut self) -> Option<T> {
        let node= self.head.take();

        if let ListNode::NonEmpty(node) = node {
            self.head=node.next;
            self.size-=1;
            Some(node.item)
        }else{
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SinglyLinkedList;

    #[test]
    fn it_works() {
        let mut linked_list: SinglyLinkedList<usize> = SinglyLinkedList::new();
        for item in 1..=4 {
            linked_list.push(item);
        }
        println!("{linked_list:#?}");
        for i in (1..=4).rev() {
            let cur=linked_list.pop();
            println!("{linked_list:#?}");
            assert_eq!(Some(i),cur);
        }

        assert_eq!(None, linked_list.pop());
    }
}


