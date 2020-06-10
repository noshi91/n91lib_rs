use crate::data_structure::BitVector;
use crate::other::bit::access;
use std::ops::Range;

pub struct WaveletMatrix {
    data: Box<[(usize, BitVector)]>,
}

impl WaveletMatrix {
    pub fn new(bitlen: usize, mut seq: Vec<usize>) -> Self {
        let len = seq.len();
        let mut data = Vec::new();
        for l in (0..bitlen).rev() {
            let v = seq.iter().map(|&x| access(x, l)).collect::<BitVector>();
            data.push((v.rank0(len), v));
            let zeros = seq.iter().filter(|&&x| !access(x, l)).cloned();
            let ones = seq.iter().filter(|&&x| access(x, l)).cloned();
            seq = zeros.chain(ones).collect();
        }
        Self {
            data: data
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    pub fn access(&self, mut index: usize) -> usize {
        let mut ret = 0;
        for (l, &(z, ref v)) in self.base_iter().rev() {
            if !v.access(index) {
                index = v.rank0(index);
            } else {
                ret |= 1 << l;
                index = z + v.rank1(index);
            }
        }
        ret
    }

    pub fn rank(&self, value: usize, mut range: Range<usize>) -> usize {
        for (l, &(z, ref v)) in self.base_iter().rev() {
            if !access(value, l) {
                range.start = v.rank0(range.start);
                range.end = v.rank0(range.end);
            } else {
                range.start = z + v.rank1(range.start);
                range.end = z + v.rank1(range.end);
            }
        }
        range.end - range.start
    }

    pub fn select(&self, value: usize, k: usize) -> usize {
        let mut index = 0;
        for (l, &(z, ref v)) in self.base_iter().rev() {
            if !access(value, l) {
                index = v.rank0(index);
            } else {
                index = z + v.rank1(index);
            }
        }
        index += k;
        for (_, &(z, ref v)) in self.base_iter() {
            if index < z {
                index = v.select0(index);
            } else {
                index = v.select1(index - z);
            }
        }
        index
    }

    pub fn count(&self, idxrng: Range<usize>, valrng: Range<usize>) -> usize {
        self.count_to(idxrng.clone(), valrng.end) - self.count_to(idxrng, valrng.start)
    }

    pub fn quantile(&self, mut range: Range<usize>, mut k: usize) -> usize {
        let mut ret = 0;
        for (l, &(z, ref v)) in self.base_iter().rev() {
            let zeros = v.rank0(range.end) - v.rank0(range.start);
            if zeros > k {
                range.start = v.rank0(range.start);
                range.end = v.rank0(range.end);
            } else {
                k -= zeros;
                ret |= 1 << l;
                range.start = z + v.rank1(range.start);
                range.end = z + v.rank1(range.end);
            }
        }
        ret
    }

    fn count_to(&self, mut range: Range<usize>, val: usize) -> usize {
        let mut ret = 0;
        for (l, &(z, ref v)) in self.base_iter().rev() {
            if !access(val, l) {
                range.start = v.rank0(range.start);
                range.end = v.rank0(range.end);
            } else {
                ret += v.rank0(range.end) - v.rank0(range.start);
                range.start = z + v.rank1(range.start);
                range.end = z + v.rank1(range.end);
            }
        }
        ret
    }

    fn base_iter(&self) -> impl DoubleEndedIterator<Item = (usize, &(usize, BitVector))> {
        self.data.iter().enumerate()
    }
}

#[test]
fn test_wavelet_matrix() {
    use crate::other::rand::rand_int;
    use crate::other::rand::rand_range;
    use crate::other::rand::rand_range_nonempty;

    let n = 1 << 10;
    let bitlen = 5;
    let s = 1 << bitlen;
    let q = 1 << 10;

    let seq = (0..n).map(|_| rand_int(0..s)).collect::<Vec<_>>();

    let wm = WaveletMatrix::new(bitlen, seq.clone());

    // access
    for i in 0..n {
        assert_eq!(wm.access(i), seq[i]);
    }

    // rank
    for _ in 0..q {
        let range = rand_range(0..n);
        let val = rand_int(0..s);
        assert_eq!(
            wm.rank(val, range.clone()),
            seq[range].iter().filter(|&&x| x == val).count()
        );
    }

    //select
    for i in 0..n {
        let val = seq[i];
        assert_eq!(
            wm.select(val, seq[..i].iter().filter(|&&x| x == val).count()),
            i
        );
    }

    //count
    for _ in 0..q {
        let idxrng = rand_range(0..n);
        let valrng = rand_range(0..s - 1);
        assert_eq!(
            wm.count(idxrng.clone(), valrng.clone()),
            seq[idxrng].iter().filter(|&x| valrng.contains(x)).count()
        );
    }

    //quantile
    for _ in 0..q {
        let range = rand_range_nonempty(0..n);
        let k = rand_int(range.clone()) - range.start;
        let mut subseq = Vec::from(&seq[range.clone()]);
        subseq.sort_unstable();
        assert_eq!(wm.quantile(range, k), subseq[k]);
    }
}
