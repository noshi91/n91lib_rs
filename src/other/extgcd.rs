pub fn extgcd(x: i64, y: i64) -> (i64, i64, u64) {
    fn helper(x: u64, y: i64) -> (i64, i64, u64) {
        if y >= 0 {
            uextgcd(x, y as u64)
        } else {
            let (cx, cy, g) = uextgcd(x, (-y) as u64);
            (cx, -cy, g)
        }
    }
    if x >= 0 {
        helper(x as u64, y)
    } else {
        let (cx, cy, g) = helper((-x) as u64, y);
        (-cx, cy, g)
    }
}

fn uextgcd(mut x: u64, mut y: u64) -> (i64, i64, u64) {
    let mut cx: (i64, i64) = (1, 0);
    let mut cy: (i64, i64) = (0, 1);
    while x != 0 {
        let t = (y / x) as i64;
        y %= x;
        cy.0 -= t * cx.0;
        cy.1 -= t * cx.1;
        use std::mem::swap;
        swap(&mut x, &mut y);
        swap(&mut cx, &mut cy);
    }
    (cy.0, cy.1, y)
}
