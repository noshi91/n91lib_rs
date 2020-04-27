/*

References

Conchon, S., & Filliâtre, J. C. (2007, October).
A persistent union-find data structure.
In Proceedings of the 2007 workshop on Workshop on ML (pp. 37-46).

*/

use crate::data_structure::RerootingPersistentArray;
use std::cell;
use std::cmp::Ordering;
use std::iter;

#[derive(Clone)]
pub struct ConchonFilliatrePersistentUnionFind {
    rank: RerootingPersistentArray<u32>,
    parent: cell::RefCell<RerootingPersistentArray<usize>>,
}

impl ConchonFilliatrePersistentUnionFind {
    pub fn new(len: usize) -> Self {
        Self {
            rank: iter::repeat(0).take(len).collect(),
            parent: cell::RefCell::new((0..len).collect()),
        }
    }

    pub fn find(&self, x: usize) -> usize {
        let par = *self.parent.borrow().get(x);
        if par == x {
            x
        } else {
            let root = self.find(par);
            if root != par {
                self.parent.replace_with(|p| p.set(x, root));
            }
            root
        }
    }

    pub fn union(&self, x: usize, y: usize) -> Self {
        let cx = self.find(x);
        let cy = self.find(y);
        if cx != cy {
            let rx = *self.rank.get(cx);
            let ry = *self.rank.get(cy);
            match rx.cmp(&ry) {
                Ordering::Less => Self {
                    rank: self.rank.clone(),
                    parent: cell::RefCell::new(self.parent.borrow().set(cx, cy)),
                },
                Ordering::Equal => Self {
                    rank: self.rank.set(cx, rx + 1),
                    parent: cell::RefCell::new(self.parent.borrow().set(cy, cx)),
                },
                Ordering::Greater => Self {
                    rank: self.rank.clone(),
                    parent: cell::RefCell::new(self.parent.borrow().set(cy, cx)),
                },
            }
        } else {
            self.clone()
        }
    }
}

#[test]
fn test_conchon_filliatre_persistent_union_find() {
    let uf = ConchonFilliatrePersistentUnionFind::new(6);
    let uf1 = uf.union(2, 4);
    let uf2 = uf1.union(1, 2);
    let uf3 = uf1.union(5, 4);
    assert!(uf.find(2) != uf.find(4));
    assert!(uf1.find(2) == uf1.find(4));
    assert!(uf2.find(1) == uf2.find(4));
    assert!(uf3.find(5) == uf3.find(2));
    assert!(uf2.find(2) != uf2.find(5));
}
