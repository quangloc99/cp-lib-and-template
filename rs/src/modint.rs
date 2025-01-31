#[rustfmt::skip]
mod modint {
    use std::ops::*;

    pub const MOD: usize = 998_244_353;

    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct ModInt(pub usize);

    #[allow(dead_code)]
    impl ModInt {
        pub fn pow(mut self, mut e: usize) -> Self {
            let mut result = ModInt(1);
            while e > 0 {
                if e & 1 == 1 { result *= self; }
                self *= self ;
                e >>= 1;
            }
            result
        }
        pub fn inv(self) -> Self { self.pow(MOD - 2) }
    }

    macro_rules! def_op {
        (
            $type:ident : $trait:ident :: $fn:ident | $a_trait:ident :: $a_fn:ident
            => (&mut $self_param: ident, $rhs_param: ident) $impl:block
        ) => {
            impl<Rhs: Into<$type>> std::ops::$trait<Rhs> for $type {
                type Output = Self;
                fn $fn(mut self, rhs: Rhs) -> Self { self.$a_fn(rhs); self }
            }
            impl<Rhs: Into<$type>> std::ops::$a_trait<Rhs> for $type {
                fn $a_fn(&mut $self_param, $rhs_param: Rhs) $impl
            }
        };
    }

    def_op!(ModInt: Add::add | AddAssign::add_assign => (&mut self, rhs) { self.0 += rhs.into().0; if self.0 >= MOD { self.0 -= MOD; } });
    def_op!(ModInt: Sub::sub | SubAssign::sub_assign => (&mut self, rhs) { self.0 = self.0.wrapping_sub(rhs.into().0); if self.0 >= MOD { self.0 = self.0.wrapping_add(MOD); } });
    def_op!(ModInt: Mul::mul | MulAssign::mul_assign => (&mut self, rhs) { self.0 = self.0 * rhs.into().0 % MOD; });
    def_op!(ModInt: Div::div | DivAssign::div_assign => (&mut self, rhs) { *self *= rhs.into().inv(); });

    impl From<String> for ModInt { fn from(s: String) -> Self { s.parse::<usize>().unwrap().into() } }
    impl From<u32> for ModInt { fn from(x: u32) -> Self { (x as usize).into() } }
    impl From<u64> for ModInt { fn from(x: u64) -> Self { (x as usize).into() } }
    impl From<i32> for ModInt { fn from(x: i32) -> Self { (x as isize).into() } }
    impl From<i64> for ModInt { fn from(x: i64) -> Self { (x as isize).into() } }
    impl From<usize> for ModInt { fn from(x: usize) -> Self { ModInt(x % MOD) } }
    impl From<isize> for ModInt {
        fn from(mut x: isize) -> Self {
            x %= MOD as isize;
            if x < 0 { x += MOD as isize; }
            ModInt(x as usize)
        }
    }
    impl std::fmt::Display for ModInt {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> { write!(f, "{}", self.0) }
    }
}

#[allow(unused_imports)]
use modint::*;

#[cfg(test)]
mod test {
    use super::modint::*;

    #[test]
    fn test_modint() {
        let a = ModInt(42);
        let b = ModInt(43);
        assert_eq!(a + b, ModInt(85), "Op +");
        assert_eq!(a - b, ModInt(MOD - 1), "Op -");
        assert_eq!(a * b, ModInt(1806), "Op *");
        assert_eq!(a / b, ModInt(487_514_685), "Op /");
    }

    #[test]
    fn test_with_primitive() {
        let a = ModInt::from(42);
        let b = 43;
        assert_eq!(a + b, ModInt(85), "Op +");
        assert_eq!(a - b, ModInt(MOD - 1), "Op -");
        assert_eq!(a * b, ModInt(1806), "Op *");
        assert_eq!(a / b, ModInt(487_514_685), "Op /");
    }

    #[test]
    fn test_assign_ops() {
        let mut a = ModInt(42);
        let b = ModInt(43);
        a += b;
        assert_eq!(a, ModInt(85), "Op +=");
        a -= b;
        assert_eq!(a, ModInt(42), "Op -=");
        a *= b;
        assert_eq!(a, ModInt(1806), "Op *=");
        a /= b;
        assert_eq!(a, ModInt(42), "Op /=");
    }

    #[test]
    fn test_pow() {
        let a = ModInt(42);
        assert_eq!(a.pow(0), ModInt(1), "Pow 0");
        assert_eq!(a.pow(1), ModInt(42), "Pow 1");
        assert_eq!(a.pow(2), ModInt(1764), "Pow 2");
        assert_eq!(a.pow(3), ModInt(74_088), "Pow 3");
    }
}
