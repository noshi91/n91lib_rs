/*

実装は不完全なので、必要になったら整備する。
実装されている関数を使う分には問題ないはずである。

*/

use std::cell::{Ref, RefCell, RefMut};
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct FibonacciHeap<T>(Option<NonEmpty<T>>)
where
    T: Ord;

#[derive(Debug)]
struct NonEmpty<T>
where
    T: Ord,
{
    head: Rc<Node<T>>,
    tail: Rc<Node<T>>,
}

#[derive(Debug)]
struct Inner<T> {
    parent: Option<Weak<Node<T>>>,
    child: Option<Rc<Node<T>>>,
    left: Option<Weak<Node<T>>>,
    right: Option<Rc<Node<T>>>,
    rank: usize,
    mark: bool,
}

#[derive(Debug)]
struct Node<T> {
    inner: RefCell<Inner<T>>,
    value: RefCell<T>,
}

pub struct RcHandle<T>(Rc<Node<T>>);

pub struct WeakHandle<T>(Weak<Node<T>>);

use std::mem::{drop, swap};

impl<T> FibonacciHeap<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self(None)
    }

    pub fn peek(&self) -> Option<Ref<'_, T>> {
        self.0.as_ref().map(|x| x.peek())
    }

    fn push_unit(&mut self, ptr: Rc<Node<T>>) {
        match self.0 {
            None => {
                self.0 = Some(NonEmpty {
                    head: ptr.clone(),
                    tail: ptr,
                })
            }
            Some(ref mut ne) => {
                ne.push_unit(ptr);
            }
        }
    }

    pub fn push(&mut self, value: T) -> RcHandle<T> {
        let ptr = Rc::new(Node {
            inner: RefCell::new(Inner {
                parent: None,
                child: None,
                left: None,
                right: None,
                rank: 0,
                mark: false,
            }),
            value: RefCell::new(value),
        });

        self.push_unit(ptr.clone());

        RcHandle(ptr)
    }

    fn merge(mut x: Rc<Node<T>>, mut y: Rc<Node<T>>) -> Rc<Node<T>> {
        if *x.peek() > *y.peek() {
            swap(&mut x, &mut y);
        }
        let mut xr = x.inner_mut();
        let mut yr = y.inner_mut();
        xr.rank += 1;
        if let Some(c) = xr.child.take() {
            c.inner_mut().left = Some(Rc::downgrade(&y));
            yr.right = Some(c);
        }
        yr.parent = Some(Rc::downgrade(&x));
        drop(yr);
        xr.child = Some(y);
        drop(xr);
        x
    }

    pub fn pop(&mut self) -> Option<RcHandle<T>> {
        if self.0.is_none() {
            return None;
        }
        let top = self.0.take().unwrap().head;

        let mut bucket: Vec<Option<Rc<Node<T>>>> = vec![];
        let mut add = |mut x: Rc<Node<T>>| loop {
            let r = x.inner.borrow().rank;
            while bucket.len() <= r {
                bucket.push(None);
            }
            let b = &mut bucket[r];
            match b.take() {
                None => {
                    *b = Some(x);
                    break;
                }
                Some(y) => {
                    x = Self::merge(x, y);
                }
            }
        };

        let mut add_list = |mut temp: Option<Rc<Node<T>>>| {
            while let Some(next) = temp {
                let mut nr = next.inner_mut();
                nr.parent = None;
                nr.left = None;
                temp = nr.right.take();
                nr.mark = false;
                drop(nr);
                add(next);
            }
        };

        add_list(top.inner_mut().right.take());
        add_list(top.inner_mut().child.take());

        for x in bucket.into_iter().filter_map(|x| x) {
            self.push_unit(x);
        }

        Some(RcHandle(top))
    }

    pub fn decrease_key<F>(&mut self, handle: &RcHandle<T>, f: F)
    where
        F: FnOnce(&mut T),
    {
        self.0.as_mut().unwrap().decrease_key(handle, f);
    }

    pub fn validate(&self, len: usize) {
        fn check_node<T>(node: Rc<Node<T>>, parent: Option<Rc<Node<T>>>) -> usize
        where
            T: Ord,
        {
            let mut res: usize = 1;
            let nr = node.inner.borrow();
            if let Some(p) = parent.clone() {
                assert!(Rc::ptr_eq(
                    &nr.parent.as_ref().unwrap().upgrade().unwrap(),
                    &p
                ));
                assert!(*p.peek() <= *node.peek());
            } else {
                assert!(nr.parent.is_none());
            }
            if let Some(c) = nr.child.clone() {
                res += check_node(c, Some(node.clone()));
            }
            if let Some(l) = nr.left.clone() {
                let l = l.upgrade().unwrap();
                assert!(Rc::ptr_eq(&l.inner.borrow().right.clone().unwrap(), &node));
            }
            if let Some(r) = nr.right.clone() {
                assert!(Rc::ptr_eq(
                    &r.inner.borrow().left.clone().unwrap().upgrade().unwrap(),
                    &node
                ));
                res += check_node(r, parent);
            }
            res
        }

        match self.0 {
            None => {}
            Some(ref ne) => {
                assert_eq!(len, check_node(ne.head.clone(), None));
                let mut prev = None;
                let mut temp = Some(ne.head.clone());
                while let Some(next) = temp {
                    assert!(*ne.head.peek() <= *next.peek());
                    temp = next.inner.borrow().right.clone();
                    prev = Some(next);
                }
                assert!(Rc::ptr_eq(&ne.tail, &prev.unwrap()));
            }
        }
    }
}

impl<T> std::ops::Add for FibonacciHeap<T>
where
    T: Ord,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        match (self.0, rhs.0) {
            (None, r) => Self(r),
            (s, None) => Self(s),
            (Some(x), Some(y)) => Self(Some(x + y)),
        }
    }
}

impl<T> std::ops::Add for NonEmpty<T>
where
    T: Ord,
{
    type Output = Self;
    fn add(mut self, mut rhs: Self) -> Self {
        if *self.peek() > *rhs.peek() {
            swap(&mut self, &mut rhs);
        }
        rhs.head.inner_mut().left = Some(Rc::downgrade(&self.tail));
        self.tail.inner_mut().right = Some(rhs.head);
        Self {
            head: self.head,
            tail: rhs.tail,
        }
    }
}

impl<T> NonEmpty<T>
where
    T: Ord,
{
    fn peek(&self) -> Ref<'_, T> {
        self.head.peek()
    }

    fn push_unit(&mut self, mut ptr: Rc<Node<T>>) {
        let NonEmpty {
            ref mut head,
            ref mut tail,
        } = *self;
        if *ptr.peek() < *head.peek() {
            swap(&mut ptr, head);
            ptr.inner_mut().left = Some(Rc::downgrade(head));
            head.inner_mut().right = Some(ptr);
        } else {
            ptr.inner_mut().left = Some(Rc::downgrade(tail));
            tail.inner_mut().right = Some(ptr.clone());
            *tail = ptr;
        }
    }

    pub fn decrease_key<F>(&mut self, handle: &RcHandle<T>, f: F)
    where
        F: FnOnce(&mut T),
    {
        let mut c = handle.0.clone();
        f(&mut *c.value.borrow_mut());
        c.inner_mut().mark = true;
        loop {
            let mut cr = c.inner_mut();
            cr.mark ^= true;
            if cr.mark {
                break;
            }
            match cr.parent.take() {
                None => {
                    if let Some(l) = cr.left.take() {
                        let ls = l.upgrade().unwrap();
                        let r = cr.right.take();
                        match r {
                            None => {
                                self.tail = ls.clone();
                            }
                            Some(ref r) => {
                                r.inner_mut().left = Some(l);
                            }
                        }
                        ls.inner_mut().right = r;
                        drop(cr);
                        self.push_unit(c);
                    }
                    break;
                }
                Some(p) => {
                    let p = p.upgrade().unwrap();
                    let mut pr = p.inner_mut();
                    pr.rank -= 1;
                    let l = cr.left.take();
                    let r = cr.right.take();
                    if let Some(ref r) = r {
                        r.inner_mut().left = l.clone();
                    }
                    match l {
                        None => {
                            pr.child = r;
                        }
                        Some(l) => {
                            l.upgrade().unwrap().inner_mut().right = r;
                        }
                    }
                    drop(pr);
                    drop(cr);
                    self.push_unit(c);
                    c = p;
                }
            }
        }
    }
}

impl<T> Node<T> {
    fn peek(&self) -> Ref<'_, T> {
        self.value.borrow()
    }

    fn inner_mut(&self) -> RefMut<'_, Inner<T>> {
        self.inner.borrow_mut()
    }
}

impl<T> RcHandle<T> {
    pub fn downgrade(this: &RcHandle<T>) -> WeakHandle<T> {
        WeakHandle(Rc::downgrade(&this.0))
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        self.0.peek()
    }
}

impl<T> WeakHandle<T> {
    pub fn upgrade(&self) -> Option<RcHandle<T>> {
        self.0.upgrade().map(|h| RcHandle(h))
    }
}

impl<T> Clone for RcHandle<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
