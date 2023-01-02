# LinkedList in Rust

Having been learning Rust for a few months now, decided to try out on implementing a simple LinkedList in Rust.

## Static-typed LinkedList

There are many great resources showcasing LinkedList implementations in Rust, for static-typed LinkedList (i.e. `Node<T>` is of a generic type `T`). Implementing LinkedList in Rust is a rather non-trival exercise because:
1. Compiler has to know size of a type.
The snippet below will not work because it is impossible for the compiler to know, at compile time, the size of `Node<T>`. We will need to put `Node<T>` behind a smart pointer, e.g. `Box<T>`, `RefCell<T>`, ...
```rust
// will not work
struct Node<T> {
    data: T,
    next: Option<Node<T>>,
}

// will work
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
```
2. If we want to have a doubly-LinkedList, then we would need `std::rc::Rc` type for reference counting, since by default, Rust only allows a single variable binding to own a data.
3. If we need the `Interior Mutability` pattern, i.e. having the borrowing rules checked at runtime, instead of compile time, then we will need to use `std::cell::RefCell`.

### Resources
1. Tutorials by [Ryan Levick](https://github.com/rylev) on [YouTube](https://www.youtube.com/@RyanLevicksVideos) (excellent teacher for Rust). Two relevant videos are:
   * [LinkedList](https://www.youtube.com/watch?v=IiDHTIsmUi4)
   * [Static vs. Dynamic dispatch](https://www.youtube.com/watch?v=tM2r9HD4ivQ)
2. Hands-on data structures and algorithms with Rust learn programming techniques to build effective, maintainable, and readable code in Rust 2018 by Matzinger, Claus; Chapter titled: Lists, Lists, and More Lists. ([Amazon](https://www.amazon.com/Hands-Data-Structures-Algorithms-Rust-ebook/dp/B07N7D6PG4/))
3. The Complete Rust Programming Reference Guide by Rahul Sharma, Vesa Kaihlavirta and Claus Matzinger; Chapter 14. ([Amazon](https://www.amazon.com/Complete-Rust-Programming-Reference-Guide/dp/1838828109))
4. [Implementing a Linked List in Rust](https://medium.com/swlh/implementing-a-linked-list-in-rust-c25e460c3676)
5. [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
6. The Rust Book; [Chapter 15](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html) (on `RefCell` and the `Interior Mutability` Pattern)

## Dynamic-typed LinkedList
I implemented (copied) a static-typed LinkedList in `same_type_linked_list.rs` just to check if I got the fundamentals right.

Loving a challenge, I tried to implement a dynamic-typed LinkedList (with two fields, `head` and `tail`, although I didn't implement `pop(&mut self)` from `head` or `tail`.). Just to be sure, by 'dynamic-typed LinkedList', I mean a LinkedList whose nodes are of  different types, e.g.
```raw
node('hello world'): Node<String> -> node(522): Node<u32> -> node(12.34): Node<f32> -> ...
``` 
I have searched the net and there was almost no resource on dynamic-typed LinkedList, which means this could be a good practice.

### First Try
My first implementation was a disaster.

Tried to have two generic types `<T, U>` and a `pub trait Next` in the implementation of `Node<T, U>`. Soon, I realize that the implementation of `struct DynLinkedList` will be locked to these two generic types and there would be no possibility of say, linking two nodes of the same type consecutively. I have even tried consuming the `DynLinkedList<T, U>` to return say, `DynLinkedList<V, U>`: changing the head. Gave up on this first approach for the second.

### Second Go
Again, a disaster (maybe not).

Tried to decouple the `data` aspect from the `next` aspect of a `Node`.
e.g.
```rust
struct Node<T> {
    data: T,
    next_obj: Next,
}

struct Next {
    next: Option<Rc<RefCell<Next>>>,
}

pub trait CanNext {
    type HasNext: CanNext

    fn borrow_next(&self) -> Option<Ref<Self::HasNext>> {...}

    //other methods...
}
```
So we leave the 'ability' to go 'next' to the `next_obj` field of `Node<T>` instead of giving it to `Node<T>` itself.
