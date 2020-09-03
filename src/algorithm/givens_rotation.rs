/*

References

Takanori, M. (Apr 5, 2013)
競技プログラミングでの線型方程式系 - SlideShare
https://www.slideshare.net/tmaehara/ss-18244588


Description

a: n × n 行列
b: n 次元ベクトル

ax = b を満たす x を計算する。

時間計算量: Θ(n^3)

軽実装で誤差が小さい

*/

use num_traits::float::Float;
use num_traits::zero;

pub fn givens_rotation<T>(mut a: Vec<Vec<T>>, mut b: Vec<T>, eps: T) -> Vec<T>
where
    T: Float,
{
    let n = b.len();

    for i in 0..n {
        for j in i + 1..n {
            let (p, q) = get_two_pos(&mut a, i, j);
            let (c, s) = {
                let (x, y) = (p[i], q[i]);
                if y.abs() <= eps {
                    continue;
                }
                let r = (x * x + y * y).sqrt();
                (x / r, -y / r)
            };
            for k in i..n {
                rotate(c, s, &mut p[k], &mut q[k]);
            }
            let (x, y) = get_two_pos(&mut b, i, j);
            rotate(c, s, x, y);
        }
        assert!(a[i][i].abs() > eps, "a must be regular");
    }

    let mut ret = vec![zero(); n];

    for i in (0..n).rev() {
        ret[i] = b[i];
        for j in i + 1..n {
            ret[i] = ret[i] - a[i][j] * ret[j];
        }
        ret[i] = ret[i] / a[i][i];
    }

    ret
}

fn rotate<T>(c: T, s: T, x: &mut T, y: &mut T)
where
    T: Float,
{
    let nx = c * *x - s * *y;
    let ny = s * *x + c * *y;
    *x = nx;
    *y = ny;
}

fn get_two_pos<T>(v: &mut Vec<T>, i: usize, j: usize) -> (&mut T, &mut T) {
    let (l, r) = v.split_at_mut(j);
    (&mut l[i], &mut r[0])
}

#[test]
fn test_givens_rotation() {
    use crate::other::rand::rand_f64;

    let n: usize = 1 << 7;
    let eps = 1e-12;

    let rand_vec = || (0..n).map(|_| rand_f64()).collect::<Vec<_>>();

    let a: Vec<_> = (0..n).map(|_| rand_vec()).collect();
    let b = rand_vec();

    let x = givens_rotation(a.clone(), b.clone(), eps);

    let mut c = vec![0f64; n];
    for i in 0..n {
        for j in 0..n {
            c[i] += a[i][j] * x[j];
        }
    }

    for i in 0..n {
        assert!((b[i] - c[i]).abs() <= eps);
    }
}
