use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};

struct Next {
  next_node_type: SameOrDifferentType,
  next: Option<Rc<RefCell<Next>>>
}

struct Node<T> {
  data: T,
  next_obj: Next,
}

type WrappedNode<T> = Rc<RefCell<Node<T>>>;

impl<T> Node<T> {
  pub fn into_wrapped(self) -> WrappedNode<T> {
    Rc::new(RefCell::new(self))
  }
}

struct DynLinkedList<T> {
  head: WrappedNode<T>,
  tail: WrappedNode<T>,
  kind: SameOrDifferentType::DifferentType 
}

struct DynLinkedList<T, U> {
  head: WrappedNode<T>,
  tail: WrappedNode<U>,
  kind: SameOrDifferentType::SameType,
}

impl<T, U> DynLinkedList<T, U> {
  pub fn into_same_type(self, new_tail: Node<T>) -> DynLinkedList<T> {
    DynLinkedList {
      head: self.head.take(),
      tail: Some(new_tail.into_wrapped()),
    }
  }
}

enum SameOrDifferentType {
  SameType,
  DifferentType
}

pub trait DynLLTrait {
  fn is_dll() -> bool {
    true
  }

  fn head_tail_type() -> HeadTailTypes;
}

impl<T> DynLLTrait for DynLinkedList<T> {
  fn head_tail_type() -> HeadTailTypes {
    HeadTailTypes::SameType
  }
}

impl<T, U> DynLLTrait for DynLinkedList<T, U> {
  fn head_tail_type() -> HeadTailTypes {
    HeadTailTypes::DifferentType
  }
}

pub enum SameOrDifferentTypes()

