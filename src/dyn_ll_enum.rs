#![allow(unused)]
use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};
use std::convert::From;

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

  pub fn set_next<U>(&mut self, next_wrapped_node: WrappedNode<U>) {
    let next_wrapped_node_owned = *next_wrapped_node.borrow_mut();
    let next_wrapped_next_obj_owned = next_wrapped_node_owned.next_obj;
    self.next_obj.next = Some(Rc::new(RefCell::new(next_wrapped_next_obj_owned)));
  }
}

pub enum DynLinkedList<T, U> {
    SameTypes(DynLinkedListSameTypes<T>),
    DiffTypes(DynLinkedListDiffTypes<T, U>),
}

struct DynLinkedListSameTypes<T> {
  head: Option<WrappedNode<T>>,
  tail: Option<WrappedNode<T>>,
  kind: SameOrDifferentType
}

struct DynLinkedListDiffTypes<T, U> {
  head: Option<WrappedNode<T>>,
  tail: Option<WrappedNode<U>>,
  kind: SameOrDifferentType,
}

impl<T, U> DynLinkedListDiffTypes<T, U> {
  pub fn into_same_types(mut self, new_tail: Node<T>) -> DynLinkedListSameTypes<T> {
    DynLinkedListSameTypes {
      head: self.head.take(),
      tail: Some(new_tail.into_wrapped()),
      kind: SameOrDifferentType::DifferentType,
    }
  }

  pub fn new_with_node(first_node: Node<T>) -> Self {
    let wrapped_first_node = first_node.into_wrapped();
    DynLinkedListDiffTypes::<T, U> {
        head: Some(wrapped_first_node.clone()),
        tail: None::<Rc<RefCell<Node<U>>>>,
        kind: SameOrDifferentType::DifferentType,
        }
    }

  pub fn append<P>(&mut self, node: Node<P>) -> impl DynLLTrait {
    if let Some(old_tail) = self.tail.take() {
        let wrapped_node = node.into_wrapped();
        let old_tail = &mut *old_tail.borrow_mut();
        old_tail.set_next(wrapped_node.clone());

        return DynLinkedList::<T, P>::DiffTypes(DynLinkedListDiffTypes {
          head: self.head.take(),
          tail: Some(wrapped_node),
          kind: SameOrDifferentType::DifferentType,
        })

      }
    DynLinkedList::<P, U>::DiffTypes(DynLinkedListDiffTypes::<P, U>::new_with_node(node))
    }
  }

pub enum SameOrDifferentType {
  SameType,
  DifferentType
}

pub trait DynLLTrait {
  fn is_dll() -> bool {
    true
  }
}

impl<T, U> DynLLTrait for DynLinkedListDiffTypes<T, U> {}
impl<T> DynLLTrait for DynLinkedListSameTypes<T> {}
impl<T, U> DynLLTrait for DynLinkedList<T, U> {}
