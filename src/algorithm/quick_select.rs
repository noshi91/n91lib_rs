use crate::other::rand::rand_int;
use std::cmp::{Ord, Ordering::*};

pub fn quick_select<T>(a: &mut [T], k: usize)
where
    T: Ord,
{
    assert!(k < a.len());
    let (mut l, mut r) = (0, a.len());
    loop {
        a.swap(l, rand_int(l..r));
        let mut lt = l + 1;
        let mut i = l + 1;
        let mut gt = r;
        while i != gt {
            match a[i].cmp(&a[l]) {
                Less => {
                    a.swap(lt, i);
                    lt += 1;
                    i += 1;
                }
                Equal => {
                    i += 1;
                }
                Greater => {
                    gt -= 1;
                    a.swap(i, gt);
                }
            }
        }
        lt -= 1;
        a.swap(l, lt);
        if k < lt {
            r = lt;
        } else if k < gt {
            return;
        } else {
            l = gt;
        }
    }
}

#[test]
fn test_quick_select() {
    let test = |q, n, s: i64| {
        for _ in 0..q {
            let mut a = (0..rand_int(1..n))
                .map(|_| rand_int(-s..s))
                .collect::<Vec<_>>()
                .into_boxed_slice();
            let mut b = a.clone();
            let k = rand_int(0..a.len());
            quick_select(&mut a, k);
            {
                let p = &a[k];
                for a in &a[..k] {
                    assert!(a <= p);
                }
                for a in &a[k..] {
                    assert!(p <= a);
                }
            }
            a.sort();
            b.sort();
            assert_eq!(a, b);
        }
    };
    test(300, 1000, 100);
    test(300, 1000, 1000);
    test(300, 1000, 10000);
}
