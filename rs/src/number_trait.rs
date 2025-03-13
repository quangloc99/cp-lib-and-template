use std::fmt::*;
use std::ops::*;
#[rustfmt::skip]
pub trait Number: PartialEq + Copy + Default + Add<Output = Self> + Sub<Output = Self> + Rem<Output = Self> + Mul<Output = Self> + Div<Output = Self> + From<bool> + Display {
    fn is_zero(&self) -> bool { self == &Self::default() }
    fn one() -> Self { Self::from(true) }
    fn zero() -> Self { Self::default() }
}
#[rustfmt::skip]
impl<T> Number for T where T: PartialEq + Copy + Default + Add<Output = Self> + Sub<Output = Self> + Rem<Output = Self> + Mul<Output = Self> + Div<Output = Self> + From<bool> + Display { }
