#![allow(unused)]
use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;
use std::fmt::Debug;

// alias for trait set https://stackoverflow.com/questions/26070559/is-there-any-way-to-create-a-type-alias-for-multiple-traits
// pub trait IsData: Clone + Debug {}
// impl<Z: Clone + Debug> IsData for Z {}

pub trait Data<T> {
    
  fn borrow_data(&self) -> &T;

  fn set_data(&mut self, data: T);

}

// pub trait Data<T> {
//   fn borrow_data<T>(&self) -> &T;

//   fn set_data<T>(&mut self, data: T);
// }

// https://stackoverflow.com/questions/65845197/how-to-define-a-recursive-trait-bound-in-rust
pub trait Next {
  type HasNext: Next;
    
  fn borrow_next(&self) -> Option<Ref<Self::HasNext>>;

  fn borrow_mut_next(&self) -> Option<RefMut<Self::HasNext>>;

  fn set_next(&mut self, next: Rc<RefCell<Self::HasNext>>);
}

pub trait Linkable<T>: Data<T> + Next {}