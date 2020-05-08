use alga::general;

#[derive(PartialEq, Clone)]
pub struct Dual<T>(pub T);

impl<T, O> general::AbstractMagma<O> for Dual<T>
where
    T: general::AbstractMagma<O>,
    O: general::Operator,
{
    fn operate(&self, right: &Self) -> Self {
        Self(right.0.operate(&self.0))
    }
}

impl<T, O> general::AbstractSemigroup<O> for Dual<T>
where
    T: general::AbstractSemigroup<O>,
    O: general::Operator,
{
}

impl<T, O> general::Identity<O> for Dual<T>
where
    T: general::Identity<O>,
    O: general::Operator,
{
    fn identity() -> Self {
        Self(T::identity())
    }
}

impl<T, O> general::AbstractMonoid<O> for Dual<T>
where
    T: general::AbstractMonoid<O>,
    O: general::Operator,
{
}
