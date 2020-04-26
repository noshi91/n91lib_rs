/*

References

Ahuja, R. K., Mehlhorn, K., Orlin, J., & Tarjan, R. E. (1990). Faster algorithms for the shortest path problem. Journal of the ACM (JACM), 37(2), 213-223.

*/

use std::cmp;
use std::mem;

pub struct RadixHeap<V> {
    buckets: Box<[Bucket<V>]>,
}

struct Bucket<V> {
    start: u64,
    data: Vec<(u64, V)>,
}

impl<V> RadixHeap<V> {
    pub fn new(c: u64) -> Self {
        let b = 64 - c.leading_zeros() as usize + 2;
        let buckets = (0..b)
            .map(|i| Bucket {
                start: Self::size_sum(i),
                data: Vec::new(),
            })
            .collect::<Vec<_>>();
        Self {
            buckets: buckets.into_boxed_slice(),
        }
    }

    pub fn push(&mut self, key: u64, value: V) {
        let b = self.buckets
            .iter_mut()
            .rev()
            .find(|b| b.start <= key)
            .expect("monotonicity was violated.");
        b.data.push((key, value));
    }

    pub fn pop(&mut self) -> Option<(u64, V)> {
        let min_pos = self.buckets.iter().position(|b| !b.data.is_empty());
        min_pos.map(|i| {
            if i != 0 {
                let data = mem::replace(&mut self.buckets[i].data, Vec::new());
                let min_key = data.iter().map(|&(key, _)| key).min().unwrap();
                self.buckets[0].start = min_key;
                let end = self.buckets.get(i + 1).map_or(std::u64::MAX, |b| b.start);
                for (i, b) in self.buckets[..=i].iter_mut().enumerate() {
                    b.start = cmp::min(end, min_key + Self::size_sum(i));
                }
                for d in data {
                    let b = self.buckets[..i]
                        .iter_mut()
                        .rev()
                        .find(|b| b.start <= d.0)
                        .unwrap();
                    b.data.push(d);
                }
            }
            self.buckets[0].data.pop().unwrap()
        })
    }

    fn size_sum(i: usize) -> u64 {
        match i {
            0 => 0,
            i => 1 << i - 1,
        }
    }
}

#[test]
fn test_radix_heap() {
    let mut heap = RadixHeap::new(3);

    heap.push(1, 0);

    heap.push(3, 1);

    heap.push(2, 2);

    assert_eq!(Some((1, 0)), heap.pop());

    heap.push(4, 3);

    heap.push(4, 3);

    assert_eq!(Some((2, 2)), heap.pop());

    assert_eq!(Some((3, 1)), heap.pop());

    assert_eq!(Some((4, 3)), heap.pop());

    assert_eq!(Some((4, 3)), heap.pop());

    assert_eq!(None, heap.pop());
}
