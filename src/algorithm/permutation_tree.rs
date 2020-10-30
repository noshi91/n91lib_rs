/*

Reference

[1] Tutorial on Permutation Tree (析合树） - Codeforces
    https://codeforces.com/blog/entry/78898

[2] 析合树学习小记_Cold_Chair的博客-CSDN博客
    https://blog.csdn.net/Cold_Chair/article/details/91358311


Description

p: {0, 1, ..., n-1} の順列

p の permutation tree を計算する。
p の成す permutation graph の modular decomposition と見ることも出来る。

時間計算量: Θ(n)

p の連続部分列であって、その値域も連続部分列になるものを考える。
これは連続部分列かつ module であると考えることが出来るため、
partitive family と近い性質を持ち、木構造で表現することが出来る。

*/

pub fn permutation_tree(p: &[usize]) -> PermutationTree {
    assert!(!p.is_empty());

    let n = p.len();

    {
        let mut check = vec![false; n].into_boxed_slice();
        for &p in p {
            assert!(p < n);
            assert!(!check[p]);
            check[p] = true;
        }
    }

    use crate::data_structure::RangeMinimumQuery;
    use std::cmp::Reverse;

    let (s_min, s_max) = (
        RangeMinimumQuery::new(p),
        RangeMinimumQuery::new(
            &p.iter()
                .map(|&p| Reverse(p))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        ),
    );
    let (i_min, i_max) = {
        let mut inv = vec![n; n].into_boxed_slice();
        for (i, &p) in p.iter().enumerate() {
            inv[p] = i;
        }
        (
            RangeMinimumQuery::new(&inv),
            RangeMinimumQuery::new(
                &inv.iter()
                    .map(|&i| Reverse(i))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
        )
    };

    let mut stack: Vec<(usize, Partial)> = vec![];
    let mut tent: Vec<usize> = vec![];
    for (y, &v) in p.iter().enumerate() {
        let mut cur_l = y;
        let mut cur_m = Leaf;
        while let Some(x) = tent.pop() {
            {
                let (il, ir) = (s_min.query(x..=y), s_max.query(x..=y).0);
                let l = i_min.query(il..=ir);
                if l < x {
                    continue;
                }
                let r = i_max.query(il..=ir).0;
                if y < r {
                    tent.push(x);
                    break;
                }
            }

            let k = stack.iter().rposition(|&(l, _)| l == x).unwrap();
            let mut ch = stack.split_off(k);
            if ch.len() != 1 {
                // prime node
                ch.push((cur_l, cur_m));
                cur_l = ch.first().unwrap().0;
                cur_m = Partial::Prime(
                    ch.into_iter()
                        .map(|(_, c)| c.into())
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                );
            } else {
                // parallel or series
                let (l, mut m) = ch.into_iter().next().unwrap();
                cur_l = l;
                cur_m = if p[l] < v {
                    match m {
                        Parallel(ref mut c) => {
                            c.push(cur_m.into());
                            m
                        }
                        _ => Parallel(vec![m.into(), cur_m.into()]),
                    }
                } else {
                    match m {
                        Series(ref mut c) => {
                            c.push(cur_m.into());
                            m
                        }
                        _ => Series(vec![m.into(), cur_m.into()]),
                    }
                };
            }
        }
        tent.push(cur_l);
        stack.push((cur_l, cur_m));
    }

    assert_eq!(stack.len(), 1);

    stack.into_iter().next().unwrap().1.into()
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PermutationTree {
    Leaf,
    Internal(NodeType, Box<[PermutationTree]>),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NodeType {
    Parallel,
    Series,
    Prime,
}

enum Partial {
    Leaf,
    Parallel(Vec<PermutationTree>),
    Series(Vec<PermutationTree>),
    Prime(Box<[PermutationTree]>),
}

use Partial::*;

impl Partial {
    fn into(self) -> PermutationTree {
        match self {
            Leaf => PermutationTree::Leaf,
            Parallel(c) => PermutationTree::Internal(NodeType::Parallel, c.into_boxed_slice()),
            Series(c) => PermutationTree::Internal(NodeType::Series, c.into_boxed_slice()),
            Prime(c) => PermutationTree::Internal(NodeType::Prime, c),
        }
    }
}

#[test]
fn test_permutation_tree() {
    use std::ops::Range;

    fn naive(p: &[usize]) -> Box<[Range<usize>]> {
        let n = p.len();
        let mut res = vec![];
        res.reserve((n + 1) * n / 2);
        let mut min = vec![n; n].into_boxed_slice();
        let mut max = vec![0; n].into_boxed_slice();
        for i in 0..n {
            for j in 0..=i {
                min[j] = min[j].min(p[i]);
                max[j] = max[j].max(p[i]);
            }
            for j in (0..=i).rev() {
                if max[j] - min[j] == i - j {
                    res.push(j..i + 1);
                }
            }
        }
        res.into_boxed_slice()
    }

    use NodeType::*;
    use PermutationTree::*;

    fn validate(t: &PermutationTree) {
        match *t {
            Internal(tp, ref cs) => {
                for c in cs.iter() {
                    validate(c);
                }
                match tp {
                    Parallel | Series => {
                        assert!(cs.len() >= 2);
                    }
                    Prime => {
                        assert!(cs.len() >= 4);
                    }
                }
            }
            _ => (),
        }
    }

    fn all_range(t: &PermutationTree, i: usize, buf: &mut Vec<Range<usize>>) -> usize {
        let mut idx = vec![i];
        match *t {
            Leaf => idx.push(i + 1),
            Internal(tp, ref cs) => {
                let n = cs.len();
                match tp {
                    Parallel | Series => {
                        for k in 0..n {
                            idx.push(all_range(&cs[k], idx[k], buf));
                            for j in (0..k).rev() {
                                buf.push(idx[j]..idx[k + 1]);
                            }
                        }
                        buf.pop();
                    }
                    Prime => {
                        for k in 0..n {
                            idx.push(all_range(&cs[k], idx[k], buf));
                        }
                    }
                }
            }
        }
        let &l = idx.last().unwrap();
        buf.push(i..l);
        l
    }

    fn from_pt(p: &[usize]) -> Box<[Range<usize>]> {
        let n = p.len();
        let mut res = vec![];
        res.reserve((n + 1) * n / 2);

        let t = permutation_tree(p);

        validate(&t);

        all_range(&t, 0, &mut res);

        res.into_boxed_slice()
    }

    fn testset(q: usize, n: usize) {
        use crate::other::rand::rand_int;

        for _ in 0..q {
            let n = rand_int(1..n);
            let mut p = (0..n).collect::<Vec<_>>().into_boxed_slice();
            for i in 0..n {
                let j = rand_int(0..i + 1);
                p.swap(j, i);
            }

            assert_eq!(naive(&p), from_pt(&p));
        }
    }

    testset(100, 10);
    testset(100, 100);
}
