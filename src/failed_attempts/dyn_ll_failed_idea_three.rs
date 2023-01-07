#![allow(unused)]
use std::cell::{Ref, RefCell, RefMut};
use std::convert::From;
use std::rc::Rc;

pub trait Next {
    type NextType: Next;

    fn borrow_next(&self) -> Option<Ref<Self::NextType>>;
    fn borrow_next_mut(&self) -> Option<RefMut<Self::NextType>>;
    fn set_next(&mut self, next_next_type: Rc<RefCell<Self::NextType>>);
}

pub trait NodeTrait {}

impl<T> NodeTrait for Node<T> {}

struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<dyn Next<NextType = dyn Next>>>>,
}

impl<T> Node<T> {
    pub fn new<U>(data: U) -> Node<U> {
        Node::<U> {
            data: data,
            next: None,
        }
    }

    pub fn borrow_data(&self) -> &T {
        &self.data
    }

    pub fn set_data(&mut self, new_data: T) {
        self.data = new_data;
    }
}

impl Next for dyn NodeTrait {
    type NextType = Node;

    fn borrow_next(&self) -> Option<Ref<Self::NextType>> {
        todo!()
    }
}
