/*

References

Conchon, S., & Filliâtre, J. C. (2007, October).
A persistent union-find data structure.
In Proceedings of the 2007 workshop on Workshop on ML (pp. 37-46).


Description

時間計算量: Ω(n) / query

直前のバージョンとの差分を持つシンプルな永続配列。
操作の度にバージョンの木を自身が根になるように組み替える
（rerooting）ため、特定の入力では著しく効率が上昇する。

*/

use std::cell;
use std::iter::FromIterator;
use std::mem;
use std::rc::Rc;

pub struct RerootingPersistentArray<T>(Rc<cell::RefCell<Node<T>>>);

enum Node<T> {
    Array(Box<[T]>),
    Diff {
        index: usize,
        value: T,
        base: RerootingPersistentArray<T>,
    },
}

impl<T> RerootingPersistentArray<T> {
    pub fn get(&self, index: usize) -> cell::Ref<'_, T> {
        self.reroot();
        cell::Ref::map(self.0.borrow(), |x| match x {
            &Node::Array(ref a) => &a[index],
            _ => unreachable!(),
        })
    }

    pub fn set(&self, index: usize, value: T) -> Self {
        Self(Rc::new(cell::RefCell::new(Node::Diff {
            index,
            value,
            base: self.clone(),
        })))
    }

    fn reroot(&self) {
        let s = &mut *self.0.borrow_mut();
        if let &mut Node::Diff {
            index,
            ref mut value,
            ref mut base,
        } = s
        {
            let par = mem::replace(base, self.clone());
            par.reroot();
            let p = &mut *par.0.borrow_mut();
            match p {
                &mut Node::Array(ref mut a) => {
                    mem::swap(value, &mut a[index]);
                }
                _ => unreachable!(),
            }
            mem::swap(s, p);
        }
    }
}

impl<T> FromIterator<T> for RerootingPersistentArray<T> {
    fn from_iter<U>(iter: U) -> Self
    where
        U: IntoIterator<Item = T>,
    {
        Self(Rc::new(cell::RefCell::new(Node::Array(
            iter.into_iter().collect::<Vec<_>>().into_boxed_slice(),
        ))))
    }
}

impl<T> Clone for RerootingPersistentArray<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[test]
fn test_rerooting_persistent_array() {
    use crate::other::rand::rand_int;

    let n = 1 << 5;
    let q = 1 << 5;
    let s = 1 << 20;
    let mut naive = vec![(0..n).map(|_| rand_int(0..s)).collect::<Vec<_>>()];
    let mut p_arr = vec![
        naive
            .last()
            .unwrap()
            .iter()
            .copied()
            .collect::<RerootingPersistentArray<_>>(),
    ];
    for i in 1..q {
        let t = rand_int(0..i);
        let ind = rand_int(0..n);
        let val = rand_int(0..s);
        naive.push(naive[t].clone());
        naive.last_mut().unwrap()[ind] = val;
        p_arr.push(p_arr[t].set(ind, val));
        for _ in 0..n * (i + 1) * 5 {
            let j = rand_int(0..i + 1);
            let k = rand_int(0..n);
            assert_eq!(naive[j][k], *p_arr[j].get(k));
        }
    }
}
