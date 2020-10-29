/*

Reference

[1] Alstrup, S., Gavoille, C., Kaplan, H., & Rauhe, T. (2002, August).
    Nearest common ancestors: a survey and a new distributed algorithm.
    In Proceedings of the fourteenth annual ACM symposium on Parallel algorithms and architectures (pp. 258-264).

[2] Range Minimum Query - Qiita
    https://qiita.com/okateim/items/e2f4a734db4e5f90e410


Description

区間最小値クエリを高速に処理するデータ構造

時間計算量
new: Θ(n)
min: Θ(1)

*/

use crate::data_structure::SparseTable;
use crate::other::Min;

pub struct RangeMinimumQuery<T>
where
    T: Ord + Clone,
{
    small: Box<[Block<T>]>,
    large: SparseTable<Min<T>>,
}

struct Block<T>
where
    T: Ord + Clone,
{
    data: Box<[T]>,
    bits: Box<[usize]>,
}

use crate::other::bit::WORD;
use std::ops::RangeInclusive;

impl<T> RangeMinimumQuery<T>
where
    T: Ord + Clone,
{
    pub fn new(a: &[T]) -> Self {
        let (mut small, large): (Vec<_>, Vec<_>) = a
            .chunks(WORD)
            .map(|a| (Block::new(a), Min(a.iter().min().unwrap().clone())))
            .unzip();
        if small.is_empty() {
            small.push(Block::new_empty());
        }
        Self {
            small: small.into_boxed_slice(),
            large: SparseTable::new(large.into_boxed_slice()),
        }
    }

    pub fn len(&self) -> usize {
        WORD * self.large.len() + self.small.last().unwrap().len()
    }

    pub fn min(&self, range: RangeInclusive<usize>) -> T {
        let (start, end) = range.into_inner();
        assert!(start <= end);
        assert!(end < self.len());

        let (st_d, st_r) = (start / WORD, start % WORD);
        let (en_d, en_r) = (end / WORD, end % WORD);

        if st_d == en_d {
            self.small[st_d].min(st_r..=en_r)
        } else {
            use std::cmp::min;

            let res = min(
                self.small[st_d].min(st_r..=WORD - 1),
                self.small[en_d].min(0..=en_r),
            );
            match self.large.fold(st_d + 1..en_d) {
                None => res,
                Some(Min(v)) => min(res, v),
            }
        }
    }
}

use crate::other::bit::{bsf, bsr};

impl<T> Block<T>
where
    T: Ord + Clone,
{
    fn new(a: &[T]) -> Self {
        let mut bit = 0;
        let bits: Vec<_> = a
            .iter()
            .enumerate()
            .map(|(i, a_i)| {
                while bit != 0 {
                    let j = bsr(bit);
                    if a[j] > *a_i {
                        bit &= !(1 << j);
                    } else {
                        break;
                    }
                }
                bit |= 1 << i;
                bit
            })
            .collect();
        Self {
            data: a.iter().cloned().collect::<Vec<_>>().into_boxed_slice(),
            bits: bits.into_boxed_slice(),
        }
    }

    fn new_empty() -> Self {
        Self {
            data: vec![].into_boxed_slice(),
            bits: vec![].into_boxed_slice(),
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn min(&self, range: RangeInclusive<usize>) -> T {
        let (start, end) = range.into_inner();
        self.data[bsf(self.bits[end] & !0 << start)].clone()
    }
}

#[test]
fn test_range_minimum_query() {
    fn testset(t: usize, n: usize, s: i32, q: usize) {
        use crate::other::rand::{rand_int, rand_range_nonempty};

        for _ in 0..t {
            let n = rand_int(1..n);
            let a = (0..n).map(|_| rand_int(-s..s)).collect::<Vec<_>>();

            let rmq = RangeMinimumQuery::new(&a);
            for _ in 0..q {
                let r = rand_range_nonempty(0..n);
                let r = r.start..=r.end - 1;

                let naive = *a[r.clone()].iter().min().unwrap();
                assert_eq!(naive, rmq.min(r));
            }
        }
    }

    testset(10, 1000, 1000, 100);
    testset(100, 100, 100, 100);
    testset(100, 100, 10, 100);
    testset(30, 10, 10, 100);

    {
        let a: Vec<i32> = vec![];
        let _x = RangeMinimumQuery::new(&a);
    }
}
