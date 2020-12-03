use std::cmp::Ord;
use std::mem::swap;

pub struct SkewHeap<T: Ord>(Option<Box<Node<T>>>);

struct Node<T: Ord> {
    item: T,
    left: SkewHeap<T>,
    right: SkewHeap<T>,
}

impl<T: Ord> SkewHeap<T> {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.take().map(|r| {
            let item = r.item;
            *self = r.left + r.right;
            item
        })
    }

    pub fn push(&mut self, item: T) {
        *self = Self(self.0.take())
            + Self(Some(Box::new(Node {
                item,
                left: Self::new(),
                right: Self::new(),
            })));
    }

    pub fn peek(&self) -> Option<&T> {
        self.0.as_ref().map(|p| &p.item)
    }
}

impl<T> std::ops::Add for SkewHeap<T>
where
    T: Ord,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(match (self.0, rhs.0) {
            (None, r) => r,
            (s, None) => s,
            (Some(mut s), Some(mut r)) => {
                if s.item > r.item {
                    swap(&mut s, &mut r);
                }
                s.right = s.right + Self(Some(r));
                swap(&mut s.left, &mut s.right);
                Some(s)
            }
        })
    }
}

#[test]
fn skew_heap_test() {
    let mut heap = SkewHeap::<i32>::new();
    assert_eq!(heap.peek(), None);
    heap.push(-1);
    heap.push(1);
    heap.push(0);
    assert_eq!(heap.peek(), Some(&-1));
    heap.pop();
    assert_eq!(heap.peek(), Some(&0));
}
