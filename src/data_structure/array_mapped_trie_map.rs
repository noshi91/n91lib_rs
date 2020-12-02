/*

References

[1] Bagwell, P. (2001). Ideal hash trees (No. REP_WORK).


Description

空間計算量を効率化した trie の一種。
key の hash 値を array mapped trie で管理するデータ構造は
hash array mapped trie と呼ばれ、連想配列の実装としてよく使われているらしい。
実装の簡略化のため、key は u32 のみとした。

*/

pub struct ArrayMappedTrieMap<T> {
    root: Node<T>,
}

enum Node<T> {
    Internal { bit: u32, child: Vec<Node<T>> },
    Leaf(Option<T>),
}

use Node::*;

impl<T> ArrayMappedTrieMap<T> {
    pub fn new() -> Self {
        Self { root: Node::new() }
    }

    pub fn get_mut(&mut self, key: u32) -> Option<&mut T> {
        let mut ptr = &mut self.root;
        let mut h: u32 = 35;
        while h != 0 {
            h -= 5;
            let d = (key >> h) % 32;
            match *ptr {
                Internal { bit, ref mut child } => {
                    if bit >> d & 1 == 0 {
                        return None;
                    }
                    let i = (bit & !(!0 << d)).count_ones() as usize;
                    ptr = &mut child[i];
                }
                _ => unreachable!(),
            }
        }
        match *ptr {
            Leaf(ref mut v) => v.as_mut(),
            _ => unreachable!(),
        }
    }

    pub fn insert(&mut self, key: u32, value: T) -> Option<T> {
        let mut ptr = &mut self.root;
        let mut h: u32 = 35;
        while h != 0 {
            h -= 5;
            let d = (key >> h) % 32;
            match *ptr {
                Internal {
                    ref mut bit,
                    ref mut child,
                } => {
                    let i = (*bit & !(!0 << d)).count_ones() as usize;
                    if *bit >> d & 1 == 0 {
                        *bit |= 1 << d;
                        child.insert(i, if h == 0 { Leaf(None) } else { Node::new() });
                    }
                    ptr = &mut child[i];
                }
                _ => unreachable!(),
            }
        }
        match *ptr {
            Leaf(ref mut v) => std::mem::replace(v, Some(value)),
            _ => unreachable!(),
        }
    }

    pub fn remove(&mut self, key: u32) -> Option<T> {
        let mut ptr = &mut self.root;
        let mut h: u32 = 35;
        while h != 0 {
            h -= 5;
            let d = (key >> h) % 32;
            match *ptr {
                Internal { bit, ref mut child } => {
                    if bit >> d & 1 == 0 {
                        return None;
                    }
                    let i = (bit & !(!0 << d)).count_ones() as usize;
                    ptr = &mut child[i];
                }
                _ => unreachable!(),
            }
        }
        match *ptr {
            Leaf(ref mut v) => v.take(),
            _ => unreachable!(),
        }
    }
}

impl<T> Node<T> {
    fn new() -> Self {
        Internal {
            bit: 0,
            child: vec![],
        }
    }
}

#[test]
fn test_array_mapped_trie_map() {
    fn testset(cases: usize, n: u32, q: usize) {
        for _ in 0..cases {
            let mut amtm = ArrayMappedTrieMap::new();
            let mut btm = std::collections::BTreeMap::new();

            for _ in 0..q {
                use crate::other::rand::rand_int;

                let key: u32 = rand_int(0..n);
                match rand_int(0..3) {
                    0 => {
                        assert_eq!(amtm.get_mut(key), btm.get_mut(&key));
                    }
                    1 => {
                        let value = rand_int(0..u64::MAX);
                        assert_eq!(amtm.insert(key, value), btm.insert(key, value));
                    }
                    2 => {
                        assert_eq!(amtm.remove(key), btm.remove(&key));
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    testset(10, 1000, 10000);
    testset(10, u32::MAX, 10000);
    testset(1000, 10, 100);
    testset(1000, 100, 100);
}
