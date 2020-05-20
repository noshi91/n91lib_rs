use num_traits::{One, Zero};
use std::ops::{Add, Mul, Neg};

pub trait Closed
where
    Self: Add<Output = Self> + Sized,
{
}

impl<T> Closed for T where T: Add<Output = T> {}

pub trait Associative
where
    Self: Closed,
{
}

pub trait Commutative
where
    Self: Closed,
{
}

pub trait Unital
where
    Self: Closed + Zero,
{
}

pub trait Invertible
where
    Self: Closed + Unital + Neg<Output = Self>,
{
}

pub trait ClosedMul
where
    Self: Mul<Output = Self> + Sized,
{
}

impl<T> ClosedMul for T where T: Mul<Output = T> + Sized {}

pub trait AssociativeMul
where
    Self: ClosedMul,
{
}

pub trait UnitalMul
where
    Self: ClosedMul + One,
{
}

pub trait Distributive
where
    Self: Closed + ClosedMul,
{
}

pub trait Annihilation
where
    Self: Closed + Unital + ClosedMul,
{
}

macro_rules! trait_alias {
    ($name:ident = $first:ident $(+ $rest:ident)*) => {
        pub trait $name: $first $(+ $rest)* {}
        impl<T: $first $(+ $rest)*> $name for T {}
    };
}

trait_alias! {Magma = Closed}

trait_alias! {Semigroup = Magma + Associative}

trait_alias! {Monoid = Semigroup + Unital}

trait_alias! {Group = Monoid + Invertible}

trait_alias! {MagmaMul = ClosedMul}

trait_alias! {SemigroupMul = MagmaMul + AssociativeMul}

trait_alias! {MonoidMul = SemigroupMul + UnitalMul}

trait_alias! {Semiring = Monoid + Commutative + MonoidMul + Distributive + Annihilation}

trait_alias! {Ring = Semiring + Invertible}
