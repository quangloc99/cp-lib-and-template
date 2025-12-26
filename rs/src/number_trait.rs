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

// https://stackoverflow.com/a/61189128
// limitation: can not accept `const` generics
// see modint.rs for example usage
#[macro_export]
macro_rules! def_op {
    (
        $type:ident $(::< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)?
        : $trait:ident :: $fn:ident | $a_trait:ident :: $a_fn:ident
        => (&mut $self_param: ident, $rhs_param: ident) $impl:block
    ) => {
        impl<
            $($( $lt$( : $clt$(+ $dlt)* )? ,)+ )?
            Rhs: Into<$type$(< $($lt),+ >)?>
        > std::ops::$trait<Rhs> for $type$(< $($lt),+ >)? {
            type Output = Self;
            fn $fn(mut self, rhs: Rhs) -> Self { self.$a_fn(rhs); self }
        }
        impl<
            $($( $lt $( : $clt $(+ $dlt)* )? ),+ ,)?
            Rhs: Into<$type$(< $($lt),+ >)?>
        > std::ops::$a_trait<Rhs> for $type$(< $($lt),+ >)? {
            fn $a_fn(&mut $self_param, $rhs_param: Rhs) $impl
        }
    };
}
