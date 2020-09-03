use std::clone::Clone;
use std::rc::Rc;

pub struct PersistentList<T>(Option<Rc<Node<T>>>);

struct Node<T> {
    value: T,
    next: PersistentList<T>,
}

impl<T> PersistentList<T> {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn head(&self) -> Option<&T> {
        self.0.as_ref().map(|x| &x.value)
    }

    pub fn tail(&self) -> Option<Self> {
        self.0.as_ref().map(|x| x.next.clone())
    }

    pub fn cons(&self, value: T) -> Self {
        Self(Some(Rc::new(Node {
            value,
            next: self.clone(),
        })))
    }
}

impl<T> PersistentList<T>
where
    T: Clone,
{
    pub fn append(&self, other: Self) -> Self {
        match self.0 {
            None => other,
            Some(ref p) => p.next.append(other).cons(p.value.clone()),
        }
    }

    pub fn reverse(&self) -> Self {
        let mut res = Self::new();
        let mut pos = self;
        while let Some(p) = &pos.0 {
            res = res.cons(p.value.clone());
            pos = &p.next;
        }
        res
    }
}

impl<T> Clone for PersistentList<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
