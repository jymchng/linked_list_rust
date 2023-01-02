#![allow(dead_code, unused)]
use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

struct Node {
    data: Box<dyn Any>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(data: Box<dyn Any>) -> Node {
        Self {
            data,
            next: None::<Rc<RefCell<Node>>>,
        }
    }

    pub fn into_wrapped(self) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(self))
    }

    pub fn borrow_next(&self) -> Option<Ref<Node>> {
        self.next
            .as_ref()
            .map_or(None, |next_| Some(next_.borrow()))
    }

    pub fn borrow_next_mut(&self) -> Option<RefMut<Node>> {
        self.next
            .as_ref()
            .map_or(None, |next_| Some(next_.borrow_mut()))
    }

    pub fn set_next(&mut self, next_node: Node) {
        self.next = Some(next_node.into_wrapped())
    }

    pub fn set_wrapped_next(&mut self, next_node: Rc<RefCell<Node>>) {
        self.next = Some(next_node)
    }

    pub fn borrow_data(&self) -> &dyn Any {
        &*self.data
    }

    pub fn set_data<T: Any>(&mut self, data: T) {
        self.data = Box::new(data);
    }
}

struct DynLinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    size: u32,
}

impl DynLinkedList {
    pub fn new_empty() -> Self {
        Self {
            head: None::<Rc<RefCell<Node>>>,
            tail: None::<Rc<RefCell<Node>>>,
            size: 0,
        }
    }

    pub fn new_with_node(first_node: Node) -> Self {
        let wrapped_first_node = first_node.into_wrapped();
        DynLinkedList {
            head: Some(wrapped_first_node.clone()),
            tail: Some(wrapped_first_node),
            size: 1,
        }
    }

    pub fn borrow_head(&self) -> Option<Ref<Node>> {
        self.head
            .as_ref()
            .map_or(None, |next_| Some(next_.borrow()))
    }

    pub fn borrow_tail(&self) -> Option<Ref<Node>> {
        self.tail
            .as_ref()
            .map_or(None, |next_| Some(next_.borrow()))
    }

    pub fn append(&mut self, node: Node) {
        let wrapped_node = node.into_wrapped();
        self.tail.take().map_or_else(
            || self.head = Some(wrapped_node.clone()),
            |old_tail| old_tail.borrow_mut().set_wrapped_next(wrapped_node.clone()),
        );
        self.tail = Some(wrapped_node);
        self.size += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_node_new_string() {
        let node = Node::new(Box::new("hello".to_string()));
        assert_eq!(
            node.borrow_data().downcast_ref::<String>().unwrap(),
            &"hello".to_string()
        );
    }

    #[test]
    fn test_node_new_u32() {
        let node = Node::new(Box::new(0_u32));
        assert_eq!(node.borrow_data().downcast_ref::<u32>().unwrap(), &0_u32);
    }

    #[test]
    fn test_node_set_data() {
        let mut node = Node::new(Box::new("hello world".to_string()));
        assert_eq!(
            node.borrow_data().downcast_ref::<String>().unwrap(),
            &String::from("hello world")
        );
        node.set_data(1234_u32);
        assert_eq!(node.borrow_data().downcast_ref::<u32>().unwrap(), &1234_u32)
    }

    #[test]
    fn test_dynll_new() {
        let dynll = DynLinkedList::new_empty();
        assert_eq!(dynll.head.is_none(), dynll.tail.is_none());
    }

    #[test]
    fn test_dynll_new_with_node() {
        let node = Node::new(Box::new("hello".to_string()));
        let dynll = DynLinkedList::new_with_node(node);
        assert_eq!(
            dynll
                .head
                .unwrap()
                .borrow()
                .borrow_data()
                .downcast_ref::<String>()
                .unwrap(),
            dynll
                .tail
                .unwrap()
                .borrow()
                .borrow_data()
                .downcast_ref::<String>()
                .unwrap()
        );
        assert_eq!(dynll.size, 1_u32)
    }

    #[test]
    fn test_dynll_append() {
        let node = Node::new(Box::new("hello".to_string()));
        let node_two = Node::new(Box::new(123_u8));
        let mut dynll = DynLinkedList::new_with_node(node);
        dynll.append(node_two);
        assert_eq!(
            dynll
                .head
                .unwrap()
                .borrow()
                .borrow_next()
                .unwrap()
                .borrow_data()
                .downcast_ref::<u8>()
                .unwrap(),
            dynll
                .tail
                .unwrap()
                .borrow()
                .borrow_data()
                .downcast_ref::<u8>()
                .unwrap()
        );
        assert_eq!(dynll.size, 2_u32)
    }
}
