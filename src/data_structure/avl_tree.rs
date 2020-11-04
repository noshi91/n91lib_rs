/*

機能のほとんどない AVL Tree
平衡二分探索木の実装例として

*/

#[derive(Debug)]
pub struct AVLTree<T>(Option<Box<Node<T>>>)
where
    T: Ord;

#[derive(Debug)]
struct Node<T>
where
    T: Ord,
{
    left: AVLTree<T>,
    right: AVLTree<T>,
    rank: usize,
    key: T,
}

trait LeftOrRight {
    type Opposite: LeftOrRight;
    fn get<T>(l: T, r: T) -> (T, T);
}

#[derive(Debug)]
enum Left {}

impl LeftOrRight for Left {
    type Opposite = Right;
    fn get<T>(l: T, r: T) -> (T, T) {
        (l, r)
    }
}

#[derive(Debug)]
enum Right {}

impl LeftOrRight for Right {
    type Opposite = Left;
    fn get<T>(l: T, r: T) -> (T, T) {
        (r, l)
    }
}

use std::cmp::Ordering::*;
use std::mem::swap;

impl<T> AVLTree<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self(None)
    }

    fn rank(&self) -> usize {
        self.0.as_ref().map_or(0, |p| p.rank)
    }

    pub fn insert(&mut self, value: T) -> bool {
        match self.0 {
            None => {
                self.0 = Some(Box::new(Node::new(value)));
                true
            }
            Some(ref mut link) => match value.cmp(&link.key) {
                Less => Self::insert_helper::<Left>(link, value),
                Equal => false,
                Greater => Self::insert_helper::<Right>(link, value),
            },
        }
    }

    fn insert_helper<Side>(p: &mut Box<Node<T>>, value: T) -> bool
    where
        Side: LeftOrRight,
    {
        let res = p.child_mut::<Side>().insert(value);
        Self::balance::<Side::Opposite>(p);
        res
    }

    pub fn remove(&mut self, value: &T) -> bool {
        match self.0 {
            None => false,
            Some(ref mut link) => match value.cmp(&link.key) {
                Less => Self::remove_helper::<Left>(link, value),
                Equal => {
                    match link.right.remove_first() {
                        None => {
                            *self = link.left.take();
                        }
                        Some(mut x) => {
                            x.left = link.left.take();
                            x.right = link.right.take();
                            x.fix();
                            self.0 = Some(x);
                        }
                    }
                    true
                }
                Greater => Self::remove_helper::<Right>(link, value),
            },
        }
    }

    fn remove_helper<Side>(p: &mut Box<Node<T>>, value: &T) -> bool
    where
        Side: LeftOrRight,
    {
        let res = p.child_mut::<Side>().remove(value);
        Self::balance::<Side>(p);
        res
    }

    fn remove_first(&mut self) -> Option<Box<Node<T>>> {
        self.0.take().map(|mut p| match p.left.remove_first() {
            None => {
                *self = p.right.take();
                p
            }
            Some(res) => {
                Self::balance::<Left>(&mut p);
                self.0 = Some(p);
                res
            }
        })
    }

    pub fn contains(&self, value: &T) -> bool {
        let mut p = self;
        while let Some(ref x) = p.0 {
            match value.cmp(&x.key) {
                Less => {
                    p = &x.left;
                }
                Equal => {
                    return true;
                }
                Greater => {
                    p = &x.right;
                }
            }
        }
        false
    }

    fn take(&mut self) -> Self {
        Self(self.0.take())
    }

    fn rotate<Side>(p: &mut Box<Node<T>>)
    where
        Side: LeftOrRight,
    {
        let r = p.child_mut::<Side::Opposite>();
        let mut t = r.0.take().unwrap();
        let rl = t.child_mut::<Side>();
        swap(r, rl);
        p.fix();
        swap(p, &mut t);
        p.child_mut::<Side>().0 = Some(t);
    }

    fn balance<Side>(p: &mut Box<Node<T>>)
    where
        Side: LeftOrRight,
    {
        let (l, r) = p.children_mut::<Side>();
        if l.rank() + 2 == r.rank() {
            let r = r.0.as_mut().unwrap();
            let (rl, rr) = r.children_mut::<Side>();
            if rl.rank() == rr.rank() + 1 {
                Self::rotate::<Side::Opposite>(r);
            }
            Self::rotate::<Side>(p);
        }
        p.fix();
    }
}

impl<T> Node<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        Self {
            left: AVLTree::new(),
            right: AVLTree::new(),
            rank: 1,
            key: value,
        }
    }

    fn fix(&mut self) {
        self.rank = std::cmp::max(self.left.rank(), self.right.rank()) + 1;
    }

    fn children_mut<Side>(&mut self) -> (&mut AVLTree<T>, &mut AVLTree<T>)
    where
        Side: LeftOrRight,
    {
        Side::get(&mut self.left, &mut self.right)
    }

    fn child_mut<Side>(&mut self) -> &mut AVLTree<T>
    where
        Side: LeftOrRight,
    {
        self.children_mut::<Side>().0
    }
}

#[test]
fn test_avl_tree() {
    fn test_set(cases: usize, n: usize, s: i32, q: usize) {
        use crate::other::rand::{rand_from_ratio, rand_int, random};
        use std::collections::BTreeSet;

        for _ in 0..cases {
            let mut avl = AVLTree::new();
            let mut bt = BTreeSet::new();

            for _ in 0..q {
                let v = rand_int(-s..s);
                if random() {
                    assert_eq!(avl.contains(&v), bt.contains(&v));
                } else if rand_from_ratio(bt.len() as u32, 2 * n as u32) {
                    assert_eq!(avl.remove(&v), bt.remove(&v));
                } else {
                    assert_eq!(avl.insert(v), bt.insert(v));
                }
            }
        }
    }

    test_set(100, 100, 10, 1000);
    test_set(100, 100, 100, 1000);
    test_set(100, 100, 1000, 1000);
    test_set(1, 100000, 100000, 100000);
}
