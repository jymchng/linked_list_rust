#![allow(dead_code)]

use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;
use std::fmt::Debug;
use crate::node_traits::{Next, Data, Linkable};

struct CanNext {
  next: Option<Rc<RefCell<CanNext>>>,
}

impl CanNext {
  fn new() -> Self {
    Self { next: None }
  }

  fn new_wrapped() -> Rc<RefCell<CanNext>> {
    Rc::new(RefCell::new(Self::new()))
  }
}

struct Node<T> {
  data: T,
  next_obj: CanNext,
}

impl<T> Node<T> {
  pub fn new(data: T) -> Node<T> {
    Self {
      next_obj: CanNext::new(),
      data,
    }
  }

  pub fn into_wrapped(self) -> Rc<RefCell<Node<T>>> {
    Rc::new(RefCell::new(self))
  }

  // pub fn into_wrapped(self) -> WrappedNode {
  //   Rc::new(RefCell::new(self))
  //   }
}

impl Next for CanNext {
    type HasNext = CanNext;
    
    fn borrow_next(&self) -> Option<Ref<CanNext>> {
      match &self.next {
        Some(next_exists) => {
          Some(next_exists.borrow())
        },
        None => {
          None
        }
      }
    }
  
    fn borrow_mut_next(&self) -> Option<RefMut<CanNext>> {
    match &self.next {
      Some(next_exists) => {
        Some(next_exists.borrow_mut())
      },
      None => {
        None
      }
    }
    }

    fn set_next(&mut self, next: Rc<RefCell<CanNext>>)
     {
      self.next = Some(next);
    }
  }

struct DynLinkedList<T, U>
  {
  head: Option<Rc<RefCell<Node<T>>>>,
  tail: Option<Rc<RefCell<Node<U>>>>,
  size: u32,
  }

trait DynLinkedListTrait {}
impl DynLinkedListTrait for DynLinkedList<T, U> {}

impl<T, U> DynLinkedList<T, U>
  {
    pub fn new_empty() -> Self {
      Self {
        head: None::<Rc<RefCell<Node<T>>>>,
        tail: None::<Rc<RefCell<Node<U>>>>,
        size: 0,
      }
    }

    pub fn new_with_node(first_node: Node<T>) -> Self {
      let wrapped_first_node = first_node.into_wrapped();
      DynLinkedList::<T, U> {
        head: Some(wrapped_first_node.clone()),
        tail: None::<Rc<RefCell<Node<U>>>>,
        size: 0,
      }
    }

    pub fn append<P>(node: Node<P>) -> impl DynLinkedListTrait {
      match self.tail.take() {
        Some(old_tail) => {
          
        }
      }
    }
  }