use std::sync::atomic::{spin_loop_hint, AtomicU64, Ordering::Relaxed};

static STATE: AtomicU64 = AtomicU64::new(91);

pub fn random() -> u64 {
    loop {
        let old = STATE.load(Relaxed);
        let mut x = old;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        let v = STATE.compare_and_swap(old, x, Relaxed);
        if v == old {
            return x;
        }
        spin_loop_hint();
    }
}
