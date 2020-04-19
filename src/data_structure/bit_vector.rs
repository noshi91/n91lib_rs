use crate::other::bit::access;
use crate::other::bit::rank;
use crate::other::bit::select;
use crate::other::bit::WORD;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::iter::Iterator;

pub struct BitVector {
    data: Box<[Node]>,
}

struct Node {
    bit: usize,
    sum: usize,
}

impl BitVector {
    pub fn access(&self, index: usize) -> bool {
        access(self.data[index / WORD].bit, index % WORD)
    }

    pub fn rank0(&self, end: usize) -> usize {
        end - self.rank1(end)
    }

    pub fn rank1(&self, end: usize) -> usize {
        let t = &self.data[end / WORD];
        t.sum + rank(t.bit, end % WORD)
    }

    pub fn select0(&self, k: usize) -> usize {
        let (mut st, mut en) = (0, self.data.len());
        while en - st != 1 {
            let mid = (st + en) / 2;
            if mid * WORD - self.data[mid].sum <= k {
                st = mid;
            } else {
                en = mid;
            }
        }
        let rem = k - (st * WORD - self.data[st].sum);
        st * WORD + select(!self.data[st].bit, rem)
    }

    pub fn select1(&self, k: usize) -> usize {
        let (mut st, mut en) = (0, self.data.len());
        while en - st != 1 {
            let mid = (st + en) / 2;
            if self.data[mid].sum <= k {
                st = mid;
            } else {
                en = mid;
            }
        }
        let rem = k - self.data[st].sum;
        st * WORD + select(self.data[st].bit, rem)
    }
}

impl FromIterator<bool> for BitVector {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let mut v = Vec::new();
        let mut sum = 0;
        'base: loop {
            let mut bit = 0;
            for i in 0..WORD {
                match iter.next() {
                    Some(v) => if v {
                        bit |= 1 << i;
                    },
                    None => {
                        v.push(Node { bit: bit, sum: sum });
                        break 'base;
                    }
                }
            }
            v.push(Node { bit: bit, sum: sum });
            sum += bit.count_ones() as usize;
        }
        Self {
            data: v.into_boxed_slice(),
        }
    }
}

#[test]
fn test_bit_vector() {
    use crate::other::rand::rand_int;

    let n = 1 << 10;

    let seq = (0..n).map(|_| rand_int(0..2) != 0).collect::<Vec<_>>();

    let bv = BitVector::from_iter(seq.iter().cloned());

    // access
    for i in 0..n {
        assert_eq!(bv.access(i), seq[i]);
    }

    // rank
    for i in 0..n {
        assert_eq!(bv.rank0(i), seq[..i].iter().filter(|&&x| !x).count());
        assert_eq!(bv.rank1(i), seq[..i].iter().filter(|&&x| x).count());
    }

    //select
    for i in 0..n {
        if !seq[i] {
            assert_eq!(bv.select0(seq[..i].iter().filter(|&&x| !x).count()), i);
        } else {
            assert_eq!(bv.select1(seq[..i].iter().filter(|&&x| x).count()), i);
        }
    }
}
