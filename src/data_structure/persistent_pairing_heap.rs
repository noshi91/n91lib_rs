/*

Reference

[1] Chris Okasaki 著, 稲葉一浩、遠藤侑介 訳, 純粋関数型データ構造,
    アスキードワンゴ, 2017

*/

use crate::other::suspension::{LazyExpr, Suspension};
use std::rc::Rc;

#[derive(Clone)]
pub struct PersistentPairingHeap<T>(Option<NonEmpty<T>>)
where
    T: Ord + Clone;

type NonEmpty<T> = Rc<Node<T>>;

struct Node<T>
where
    T: Ord + Clone,
{
    odd: Option<NonEmpty<T>>,
    tail: Option<Rc<Suspension<Paired<T>>>>,
    value: T,
}

struct Paired<T>
where
    T: Ord + Clone,
{
    x: NonEmpty<T>,
    y: NonEmpty<T>,
    tail: Option<Rc<Suspension<Paired<T>>>>,
}

impl<T> PersistentPairingHeap<T>
where
    T: Ord + Clone,
{
    pub fn new() -> Self {
        Self(None)
    }

    pub fn peek(&self) -> Option<&T> {
        self.0.as_ref().map(|h| &h.value)
    }

    pub fn pop(&self) -> Option<Self> {
        self.0.as_ref().map(|s| {
            let forced = s.tail.as_ref().map(|s| s.force());
            meld(s.odd.as_ref(), forced.as_ref().map(|r| &**r))
        })
    }
}

impl<T> std::ops::Add for &PersistentPairingHeap<T>
where
    T: Ord + Clone,
{
    type Output = PersistentPairingHeap<T>;
    fn add(self, rhs: Self) -> Self::Output {
        meld(self.0.as_ref(), rhs.0.as_ref())
    }
}

impl<T> LazyExpr for Paired<T>
where
    T: Ord + Clone,
{
    type Output = NonEmpty<T>;
    fn evaluate(self) -> NonEmpty<T> {
        let t = merge(&self.x, &self.y);
        match self.tail {
            None => t,
            Some(tail) => merge(&t, &*tail.force()),
        }
    }
}

fn meld<'a, T>(x: Option<&NonEmpty<T>>, y: Option<&NonEmpty<T>>) -> PersistentPairingHeap<T>
where
    T: Ord + Clone,
{
    PersistentPairingHeap(match (x, y) {
        (None, y) => y.cloned(),
        (x, None) => x.cloned(),
        (Some(x), Some(y)) => Some(merge(x, y)),
    })
}

fn merge<'a, T>(mut x: &'a NonEmpty<T>, mut y: &'a NonEmpty<T>) -> NonEmpty<T>
where
    T: Ord + Clone,
{
    if x.value > y.value {
        std::mem::swap(&mut x, &mut y);
    }

    let node = match x.odd {
        None => Node {
            odd: Some(y.clone()),
            tail: x.tail.clone(),
            value: x.value.clone(),
        },
        Some(ref odd) => Node {
            odd: None,
            tail: Some(Rc::new(Suspension::new(Paired {
                x: y.clone(),
                y: odd.clone(),
                tail: x.tail.clone(),
            }))),
            value: x.value.clone(),
        },
    };

    Rc::new(node)
}
