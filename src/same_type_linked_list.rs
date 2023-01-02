use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

type WrappedNode<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Node<T>
where
    T: Clone,
{
    next: Option<WrappedNode<T>>,
    data: T,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct LinkedList<T>
where
    T: Clone,
{
    head: Option<WrappedNode<T>>,
    tail: Option<WrappedNode<T>>,
}

impl<T> Node<T>
where
    T: Clone,
{
    pub fn new(data: T) -> Node<T> {
        Self { next: None, data }
    }

    pub fn set_data(&mut self, data: T) {
        self.data = data;
    }

    pub fn borrow_data(&self) -> &T {
        &self.data
    }

    pub fn new_wrapped(data: T) -> WrappedNode<T> {
        Rc::new(RefCell::new(Self { next: None, data }))
    }

    pub fn into_wrapped(self) -> WrappedNode<T> {
        Rc::new(RefCell::new(self))
    }

    pub fn borrow_next(&self) -> Option<Ref<Node<T>>> {
        self.next
            .as_ref()
            .map_or(None, |next_| Some(next_.borrow()))
    }

    pub fn set_next(&mut self, next: WrappedNode<T>) {
        self.next = Some(next);
    }

    pub fn borrow_mut_next(&self) -> Option<RefMut<Node<T>>> {
        self.next
            .as_ref()
            .map_or(None, |next_| Some(next_.borrow_mut()))
    }
}

impl<T> LinkedList<T>
where
    T: Clone,
{
    pub fn borrow_head(&self) -> &Option<WrappedNode<T>> {
        &self.head
    }

    pub fn borrow_tail(&self) -> &Option<WrappedNode<T>> {
        &self.tail
    }

    pub fn new_empty() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn new_with_node(first_node: Node<T>) -> Self {
        let mut new_ll = Self::new_empty();
        Self::append(&mut new_ll, first_node);
        new_ll
    }

    pub fn append(&mut self, node: Node<T>) {
        assert!(node.next.is_none()); // make sure its not joining two linkedlists

        let appending_wrapped_node = node.into_wrapped();

        match self.tail.take() {
            Some(old_tail) => {
                old_tail
                    .borrow_mut()
                    .set_next(appending_wrapped_node.clone());
            }

            None => {
                self.head = Some(appending_wrapped_node.clone());
            }
        }
        self.tail = Some(appending_wrapped_node);
    }
}

// CRUD -> create new, add to head tail, read head tail, update anywhere, delete anywhere, delete head tail

#[cfg(test)]
mod tests {
    use super::{LinkedList, Node};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_node_ref_data() {
        let node = Node::new(5);
        assert_eq!(node.borrow_data(), &5);
    }

    #[test]
    fn test_node_ref_next() {
        let node = Node::new(5);
        assert!(node.borrow_next().is_none());
    }

    #[test]
    fn test_node_set_next() {
        let mut node = Node::new(33);
        let next_node = Node::new(34).into_wrapped();
        Node::set_next(&mut node, next_node);
        assert_eq!(node.borrow_next().unwrap().borrow_data(), &34);
    }

    #[test]
    fn test_ll_new_empty() {
        #[derive(Debug, PartialEq, PartialOrd, Clone)]
        struct Nothing;
        let new_ll = LinkedList::<Nothing>::new_empty();
        assert_eq!(
            new_ll,
            LinkedList {
                head: None,
                tail: None
            }
        )
    }

    #[test]
    fn test_ll_new_with_node_head_tail_correct_both_next_isnone() {
        let new_ll = LinkedList::new_with_node(Node::new(45));
        assert_eq!(
            new_ll.head.as_ref().unwrap().borrow().borrow_data(),
            new_ll.tail.as_ref().unwrap().borrow().borrow_data()
        );
        assert_eq!(
            new_ll.head.unwrap().borrow().borrow_next().is_none(),
            new_ll.tail.unwrap().borrow().borrow_next().is_none()
        );
    }

    #[test]
    fn test_ll_append_head_borrow_next_is_tail() {
        let mut new_ll = LinkedList::new_with_node(Node::new(45));
        new_ll.append(Node::new(77));
        assert_eq!(
            new_ll
                .head
                .unwrap()
                .borrow()
                .borrow_next()
                .unwrap()
                .borrow_data(),
            new_ll.tail.unwrap().borrow().borrow_data()
        );
    }

    #[test]
    fn test_ll_append_tail_head_correct() {
        let mut new_ll = LinkedList::new_with_node(Node::new(45));
        LinkedList::append(&mut new_ll, Node::new(77));
        assert_eq!(new_ll.tail.unwrap().borrow().borrow_data(), &77);
        assert_eq!(new_ll.head.unwrap().borrow().borrow_data(), &45);
    }

    #[test]
    fn test_ll_append_first_borrow_next() {
        let mut new_ll = LinkedList::new_with_node(Node::new(45));
        LinkedList::append(&mut new_ll, Node::new(77));
        assert_eq!(
            new_ll
                .head
                .unwrap()
                .borrow()
                .borrow_next()
                .unwrap()
                .borrow_data(),
            &77
        )
    }

    // end of tests
}
