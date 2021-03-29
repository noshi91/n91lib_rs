/*

トレイトオブジェクトを利用した再帰
オーバーヘッドがある上に不変でないと使えない
再帰深さが全体の計算と比べて小さい時には便利だと思われる

*/

pub struct RecLambda<F> {
    f: F,
}

pub trait MyFn<A, R> {
    fn call(&self, a: A) -> R;
}

impl<F> RecLambda<F> {
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F, A, R> MyFn<A, R> for RecLambda<F>
where
    F: Fn(&dyn MyFn<A, R>, A) -> R,
{
    fn call(&self, a: A) -> R {
        (self.f)(self, a)
    }
}

#[test]
fn test_rec_lambda() {
    let v = vec![2, 3, 5, 7];

    let f = RecLambda::new(
        |f: &dyn MyFn<usize, i32>, i: usize| -> i32 {
            if i == 4 {
                1
            } else {
                v[i] * f.call(i + 1)
            }
        },
    );

    assert_eq!(f.call(0), 210);
}
