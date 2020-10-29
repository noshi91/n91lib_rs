/*

https://scrapbox.io/data-structures/Sparse_Table

*/

use crate::other::algebraic::Band;

pub struct SparseTable<T>
where
    T: Band + Clone,
{
    data: Box<[Box<[T]>]>,
}

use crate::other::bit::bsr;
use std::ops::Range;

impl<T> SparseTable<T>
where
    T: Band + Clone,
{
    pub fn new(mut a: Box<[T]>) -> Self {
        if a.is_empty() {
            return Self {
                data: vec![vec![].into_boxed_slice()].into_boxed_slice(),
            };
        }

        let mut data = vec![];
        for w in (0..bsr(a.len())).map(|p| 1 << p) {
            let next = a
                .iter()
                .zip(&a[w..])
                .map(|(l, r)| l.clone() + r.clone())
                .collect::<Vec<_>>()
                .into_boxed_slice();
            data.push(a);
            a = next;
        }
        data.push(a);

        Self {
            data: data.into_boxed_slice(),
        }
    }

    pub fn len(&self) -> usize {
        self.data[0].len()
    }

    pub fn fold(&self, Range { start, end }: Range<usize>) -> Option<T> {
        assert!(start <= end);
        assert!(end <= self.len());

        if start == end {
            None
        } else {
            let p = bsr(end - start);
            Some(self.data[p][start].clone() + self.data[p][end - (1 << p)].clone())
        }
    }
}

#[test]
fn test_sparse_table() {
    fn testset(t: usize, n: usize, s: i32, q: usize) {
        use crate::other::rand::{rand_int, rand_range};
        use crate::other::Min;

        for _ in 0..t {
            let n = rand_int(0..n);
            let a = (0..n).map(|_| rand_int(-s..s)).collect::<Vec<_>>();

            let st = SparseTable::new(
                a.iter()
                    .map(|&s| Min(s))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            );
            for _ in 0..q {
                let r = rand_range(0..n);

                let naive = a[r.clone()].iter().min();
                assert_eq!(naive.map(|&v| v), st.fold(r).map(|Min(v)| v));
            }
        }
    }

    testset(100, 100, 100, 100);
    testset(100, 100, 10, 100);
    testset(30, 10, 10, 100);
}
