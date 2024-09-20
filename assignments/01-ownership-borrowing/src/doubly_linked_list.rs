// Now try and implement a doubly linked version. Give an explanation
// for why this doesn't work.

struct Node {
    value: i32,
    next: Link,
    prev: Link,
}

type Link = Option<Box<Node>>;

pub struct LinkedStack {
    head: Link,
}

impl LinkedStack {
    fn new() -> Self {
        LinkedStack { head: None }
    }

    fn push(&mut self, val: i32) {
        let node = Box::new(Node {
            value: val,
            next: self.head.take(),
            prev: None,
        });
        self.head = Some(node);
    }

    fn pop(&mut self) -> Option<i32> {
        todo!();
    }
}
