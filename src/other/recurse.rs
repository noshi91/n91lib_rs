/*

shino16_cp による実装を参考にした。

トレイトオブジェクトを利用した再帰
オーバーヘッドがある上に不変でないと使えない
再帰深さが全体の計算と比べて小さい時には便利だと思われる

*/

pub fn recurse<A, R, F>(f: F) -> impl Fn(A) -> R
where
    F: Fn(&dyn Fn(A) -> R, A) -> R,
{
    fn call<A, R, F>(f: &F, a: A) -> R
    where
        F: Fn(&dyn Fn(A) -> R, A) -> R,
    {
        f(&|a: A| call::<A, R, F>(f, a), a)
    }
    move |a: A| call::<A, R, F>(&f, a)
}

#[test]
fn test_recurse() {
    let v = vec![2, 3, 5, 7];

    let f = recurse::<usize, i32, _>(|f, i: usize| -> i32 {
        if i == 4 {
            1
        } else {
            v[i] * f(i + 1)
        }
    });

    assert_eq!(f(0), 210);
}
