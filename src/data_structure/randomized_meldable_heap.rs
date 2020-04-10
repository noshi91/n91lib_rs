use crate::other::random;
use std::cmp::Ord;
use std::mem::swap;

pub struct RandomizedMeldableHeap<T: Ord>(Option<Box<Node<T>>>);

struct Node<T: Ord> {
    item: T,
    left: RandomizedMeldableHeap<T>,
    right: RandomizedMeldableHeap<T>,
}

impl<T: Ord> RandomizedMeldableHeap<T> {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.take().map(|mut r| {
            let item = r.item;
            *self = r.left;
            self.append(&mut r.right);
            item
        })
    }

    pub fn push(&mut self, item: T) {
        self.append(&mut Self(Some(Box::new(Node {
            item: item,
            left: Self::new(),
            right: Self::new(),
        }))));
    }

    pub fn peek(&self) -> Option<&T> {
        self.0.as_ref().map(|p| &p.item)
    }

    pub fn append(&mut self, other: &mut Self) {
        match other.0.take() {
            None => (),
            Some(o) => {
                self.append2(o);
            }
        }
    }

    fn append2(&mut self, mut other: Box<Node<T>>) {
        match &mut self.0 {
            None => {
                self.0 = Some(other);
            }
            Some(s) => {
                if s.item < other.item {
                    swap(s, &mut other);
                }
                if random() % 2 == 0 {
                    &mut s.left
                } else {
                    &mut s.right
                }.append2(other);
                swap(&mut s.left, &mut s.right);
            }
        }
    }
}

#[test]
fn randomized_meldable_heap_test() {
    let mut heap = RandomizedMeldableHeap::<i32>::new();
    assert_eq!(heap.peek(), None);
    heap.push(-1);
    heap.push(1);
    heap.push(0);
    assert_eq!(heap.peek(), Some(&1));
    heap.pop();
    assert_eq!(heap.peek(), Some(&0));
}
