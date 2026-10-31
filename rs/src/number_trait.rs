use std::fmt::*;
use std::ops::*;
#[rustfmt::skip]
pub trait Number: PartialEq + Copy + Default + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + From<usize> + Display {
    fn is_zero(&self) -> bool { self == &Self::default() }
    fn one() -> Self { Self::from(1) }
    fn zero() -> Self { Self::default() }
}
#[rustfmt::skip]
impl<T> Number for T where T: PartialEq + Copy + Default + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + From<usize> + Display { }

pub trait NumberWithRem: Number + Rem<Output = Self> {}

#[rustfmt::skip]
impl<T> NumberWithRem for T where T: Number + Rem<Output = Self> {}
