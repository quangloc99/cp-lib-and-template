#[rustfmt::skip]
mod modint {
    use std::ops::*;
    use crate::def_op;

    pub trait Modulus: Clone + Copy + PartialEq + Eq + Default {
        fn m() -> usize;
    }

    #[derive(Clone, Copy, PartialEq, Eq, Default)]
    pub struct ConstModulus<const M: usize>;
    impl<const M: usize> Modulus for ConstModulus<M> {
        fn m() -> usize { M }
    }

    #[derive(Clone, Copy, Eq, PartialEq, Default)]
    pub struct ModInt<Mod: Modulus>(pub usize, pub Mod);

    #[allow(dead_code)]
    impl<Mod: Modulus> ModInt<Mod> {
        pub fn m() -> usize { Mod::m() }
        pub fn raw(x: usize) -> Self { Self(x, Mod::default()) }
        pub fn pow(mut self, mut e: usize) -> Self {
            let mut result = Self::raw(1);
            while e > 0 {
                if e & 1 == 1 { result *= self; }
                self *= self ;
                e >>= 1;
            }
            result
        }
        pub fn inv(self) -> Self { self.pow(Mod::m() - 2) }
    }

    def_op!(ModInt::<Mod: Modulus>: Add::add | AddAssign::add_assign => (&mut self, rhs) { self.0 += rhs.into().0; if self.0 >= Mod::m() { self.0 -= Mod::m(); } });
    def_op!(ModInt::<Mod: Modulus>: Sub::sub | SubAssign::sub_assign => (&mut self, rhs) { self.0 = self.0.wrapping_sub(rhs.into().0); if self.0 >= Mod::m() { self.0 = self.0.wrapping_add(Mod::m()); } });
    def_op!(ModInt::<Mod: Modulus>: Mul::mul | MulAssign::mul_assign => (&mut self, rhs) { self.0 = self.0 * rhs.into().0 % Mod::m(); });
    def_op!(ModInt::<Mod: Modulus>: Div::div | DivAssign::div_assign => (&mut self, rhs) { *self *= rhs.into().inv(); });

    impl<Mod: Modulus> From<String> for ModInt<Mod> { fn from(s: String) -> Self { s.parse::<usize>().unwrap().into() } }
    impl<Mod: Modulus> From<u32> for ModInt<Mod> { fn from(x: u32) -> Self { (x as usize).into() } }
    impl<Mod: Modulus> From<u64> for ModInt<Mod> { fn from(x: u64) -> Self { (x as usize).into() } }
    impl<Mod: Modulus> From<i32> for ModInt<Mod> { fn from(x: i32) -> Self { (x as isize).into() } }
    impl<Mod: Modulus> From<i64> for ModInt<Mod> { fn from(x: i64) -> Self { (x as isize).into() } }
    impl<Mod: Modulus> From<usize> for ModInt<Mod> { fn from(x: usize) -> Self { Self::raw(x % Mod::m()) } }
    impl<Mod: Modulus> From<isize> for ModInt<Mod> {
        fn from(mut x: isize) -> Self {
            x %= Mod::m() as isize;
            if x < 0 { x += Mod::m() as isize; }
            ModInt::raw(x as usize)
        }
    }
    impl<Mod: Modulus> std::fmt::Display for ModInt<Mod> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> { write!(f, "{}", self.0) }
    }

    impl<Mod: Modulus> std::fmt::Debug for ModInt<Mod> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> { write!(f, "mint({})", self.0) }
    }
}

#[allow(unused_imports)]
use modint::*;

#[allow(dead_code)]
type Mint = ModInt<ConstModulus<998_244_353>>;
// type Mint = ModInt<ConstModulus<1_000_000_007>>;

#[cfg(test)]
mod test {
    use super::Mint;

    #[test]
    fn test_modint() {
        let a = Mint::raw(42);
        let b = Mint::raw(43);
        assert_eq!(a + b, Mint::raw(85), "Op +");
        assert_eq!(a - b, Mint::raw(Mint::m() - 1), "Op -");
        assert_eq!(a * b, Mint::raw(1806), "Op *");
        assert_eq!(a / b, Mint::raw(487_514_685), "Op /");
    }

    #[test]
    fn test_with_primitive() {
        let a = Mint::from(42);
        let b = 43;
        assert_eq!(a + b, Mint::raw(85), "Op +");
        assert_eq!(a - b, Mint::raw(Mint::m() - 1), "Op -");
        assert_eq!(a * b, Mint::raw(1806), "Op *");
        assert_eq!(a / b, Mint::raw(487_514_685), "Op /");
    }

    #[test]
    fn test_assign_ops() {
        let mut a = Mint::raw(42);
        let b = Mint::raw(43);
        a += b;
        assert_eq!(a, Mint::raw(85), "Op +=");
        a -= b;
        assert_eq!(a, Mint::raw(42), "Op -=");
        a *= b;
        assert_eq!(a, Mint::raw(1806), "Op *=");
        a /= b;
        assert_eq!(a, Mint::raw(42), "Op /=");
    }

    #[test]
    fn test_pow() {
        let a = Mint::raw(42);
        assert_eq!(a.pow(0), Mint::raw(1), "Pow 0");
        assert_eq!(a.pow(1), Mint::raw(42), "Pow 1");
        assert_eq!(a.pow(2), Mint::raw(1764), "Pow 2");
        assert_eq!(a.pow(3), Mint::raw(74_088), "Pow 3");
    }
}
