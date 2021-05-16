/*

Reference

- 本稿の執筆時点 (2021-03-09) で削除されている


Description

a: F_p 上の d 次多項式を成分とする n 次正方行列
m: 非負整数

a(0) * a(1) * ... * a(m-1) を計算する。
m < p < 2^w, F_p 上の t 次の多項式乗算を O(t log(p)) で
行えると仮定している。

時間計算量: O(n^2 (n + d + log(p)) sqrt(dm))

m >> d の場合を説明する。
b を sqrt(m / d) 付近の整数とする。
S(w, t) を列 (\prod_{i=0}^{w-1} a(t+bk+i))_{k=0}^{dw} とする。
S(b, 0) が求まればそれらを掛け合わせて端数を処理すればよい。
S(w, 0) から多項式補間を用いて S(w, w), S(w, (dw+1)b), S(w, (dw+1)b+w) を
O(n^2 dw log(dw) + log(p)) で計算することが出来る。
それらを組み合わせることで S(2w, 0) を得る。
S(1, 0) から始めてこの操作を繰り返すことで、S(b, 0) を得ることが出来る。

p-recursive な数列の m 項目を計算することに利用できる。

逆元周りを丁寧にやると log(p) が log(m) になると思われる。
行列積や多点評価により高速なアルゴリズムを用いれば時間計算量は改善されるはずだが、
n と d がとても小さい場合はこの実装で十分だろう。

*/

use crate::matrix;
use crate::other::algebraic::{One, Zero};
use crate::other::fp_utils::FpUtils;
use crate::other::matrix::Matrix;
use crate::other::Fp;
use crate::other::Polynomial;

pub fn polynomial_matrix_prod(a: &Matrix<Polynomial<Fp>>, m: u64) -> Matrix<Fp> {
    assert_eq!(a.row_count(), a.col_count());
    let n: usize = a.row_count();
    assert!(n >= 1);
    let d: u64 = a.inner().map(|p| p.degree().unwrap_or(0)).max().unwrap() as u64;
    let b: u64 = (0..).find(|&b| (d * b + 1) * b > m).unwrap() - 1;

    let get = |i: u64| a.clone().map(|p| p.evaluate(&Fp::from(i)));
    let naive = |r: std::ops::Range<u64>| r.map(get).fold(Matrix::identity(n), |b, a| b * a);

    let fp_ut = FpUtils::new((d * b + 1) as usize);

    let interpolate = |part: &[Matrix<Fp>], t: u64, len: usize| -> Vec<Matrix<Fp>> {
        let mut ret = vec![matrix![Fp::zero(); n; n]; len];
        let t = Fp::from(t) / Fp::from(b) - Fp::from(part.len() - 1);
        let sfact = {
            let mut sfact = vec![Fp::zero(); part.len() + len];
            sfact[0] = Fp::one();
            for i in 0..sfact.len() - 1 {
                let temp = sfact[i] * (t + Fp::from(i));
                sfact[i + 1] = temp;
            }
            sfact
        };

        for ir in 0..n {
            for ic in 0..n {
                let mut s: Vec<Fp> = part.iter().map(|mat| mat[ir][ic]).collect();
                for (i, s) in s.iter_mut().enumerate() {
                    *s *= (-Fp::one()).pow((part.len() - 1 - i) as u64)
                        * (fp_ut.inv_fact(i) * fp_ut.inv_fact(part.len() - 1 - i));
                }
                let l: Vec<Fp> = (0..part.len() + len - 1)
                    .map(|i| Fp::one() / (t + Fp::from(i)))
                    .collect();
                let r =
                    crate::algorithm::number_theoretic_transform::fp_convolution(Fp::from(3), s, l);
                for i in 0..len {
                    ret[i][ir][ic] = r[part.len() - 1 + i] * sfact[part.len() + i] / sfact[i];
                }
            }
        }

        ret
    };

    use crate::other::recurse::recurse;

    let eval_s = recurse::<u64, Vec<Matrix<Fp>>, _>(|eval_s, w: u64| -> Vec<Matrix<Fp>> {
        if w == 0 {
            return vec![Matrix::identity(n)];
        }

        if w % 2 == 1 {
            let mut s = eval_s(w - 1);
            for (i, s) in s.iter_mut().enumerate() {
                *s = s.clone() * get(b * i as u64 + (w - 1));
            }
            s.extend((d * (w - 1) + 1..d * w + 1).map(|i| naive(b * i..b * i + w)));
            s
        } else {
            let mut s = eval_s(w / 2);
            let x = interpolate(&s, b * (d * (w / 2) + 1), (d * (w / 2)) as usize);
            let y = interpolate(&s, w / 2, (d * w) as usize + 1);
            s.extend(x);
            for (s, y) in s.iter_mut().zip(y) {
                *s = s.clone() * y;
            }
            s
        }
    });

    let s = eval_s(b);
    s.into_iter().fold(Matrix::identity(n), |b, a| b * a) * naive((d * b + 1) * b..m)
}

#[test]
fn test_polynomial_matrix_prod() {
    use crate::other::rand::rand_int;
    {
        // general
        fn test(n: usize, d: usize, m: u64, u: u32) {
            let a = {
                let mut a = matrix![Polynomial::new(); n; n];
                for i in 0..n {
                    for j in 0..n {
                        a[i][j].coef = (0..d).map(|_| Fp(rand_int(0..u))).collect();
                    }
                }
                a
            };
            let naive = (0..m)
                .map(|i| a.clone().map(|p| p.evaluate(&Fp::from(i))))
                .fold(Matrix::identity(n), |b, a| b * a);
            let fast = polynomial_matrix_prod(&a, m);
            assert_eq!(naive, fast);
        }
        for _ in 0..100 {
            let n = rand_int(1..4);
            let d = rand_int(0..5);
            let m = rand_int(0..100);
            test(n, d, m, crate::other::fp::P);
        }
        for _ in 0..100 {
            let n = rand_int(1..4);
            let d = rand_int(0..5);
            let m = rand_int(0..5);
            test(n, d, m, crate::other::fp::P);
        }
    }
    let fp_ut = FpUtils::new(100000);
    {
        // factorial
        let test = |n: usize| {
            let naive: Fp = fp_ut.fact(n);
            let fast: Fp = polynomial_matrix_prod(
                &matrix![Polynomial{ coef: vec![Fp::one(), Fp::one()] }; 1; 1],
                n as u64,
            )[0][0];
            assert_eq!(naive, fast);
        };
        for n in 0..1000 {
            test(n);
        }
    }
    {
        // binomial sum
        let test = |n: usize, m: usize| {
            let naive: Fp = (0..m).map(|i| fp_ut.binom(n, i)).sum();
            let fast: Fp = {
                let a = matrix![
                    [
                        Polynomial {
                            coef: vec![Fp::one(), Fp::one()]
                        },
                        Polynomial {
                            coef: vec![Fp::one(), Fp::one()]
                        }
                    ],
                    [
                        Polynomial { coef: vec![] },
                        Polynomial {
                            coef: vec![Fp::from(n), -Fp::one()]
                        }
                    ]
                ]
                .transpose();
                let prod = polynomial_matrix_prod(&a, m as u64).transpose();
                let init = matrix![[Fp::zero()], [Fp::one()]];
                let t = (prod * init)[0][0];
                let fact = polynomial_matrix_prod(
                    &matrix![[Polynomial {
                        coef: vec![Fp::one(), Fp::one()]
                    }]],
                    m as u64,
                )[0][0];
                t / fact
            };
            assert_eq!(naive, fast);
        };
        for _ in 0..1000 {
            let n = rand_int(0..1000);
            let m = rand_int(0..n + 2);
            test(n, m);
        }
    }
}
