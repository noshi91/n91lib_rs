pub const WORD: usize = (0 as usize).count_zeros() as usize;

pub fn access(bit: usize, index: usize) -> bool {
    bit & 1 << index != 0
}

pub fn rank(bit: usize, end: usize) -> usize {
    (bit & !(!0 << end)).count_ones() as usize
}

pub fn select(bit: usize, k: usize) -> usize {
    let (mut st, mut en) = (0, WORD);
    while en - st != 1 {
        let mid = (st + en) / 2;
        if rank(bit, mid) <= k {
            st = mid;
        } else {
            en = mid;
        }
    }
    st
}
