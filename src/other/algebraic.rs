use num_traits::{One, Zero};
use std::marker::Sized;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

macro_rules! trait_alias {
    ($name:ident = $($t:tt)*) => {
        pub trait $name: $($t)* {}
        impl<T: $($t)*> $name for T {}
    };
}

trait_alias! {Semigroup = Add<Output = Self> + Sized}

trait_alias! {Monoid = Semigroup + Zero}

trait_alias! {CommutativeMonoid = Monoid + AddAssign}

trait_alias! {Group = Monoid + Neg<Output = Self>}

trait_alias! {Abelian = Group + CommutativeMonoid + Sub<Output = Self> + SubAssign}

trait_alias! {Semiring = CommutativeMonoid + Mul<Output = Self> + Sized + One}

trait_alias! {Ring = Semiring + Abelian}

trait_alias! {CommutativeRing = Ring + MulAssign}

trait_alias! {Field = CommutativeRing + Div<Output = Self> + DivAssign}
