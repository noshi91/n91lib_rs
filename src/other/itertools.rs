pub fn zip<A, B>(a: A, b: B) -> std::iter::Zip<A::IntoIter, B::IntoIter>
where
    A: IntoIterator,
    B: IntoIterator,
{
    a.into_iter().zip(b)
}
