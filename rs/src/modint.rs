#[rustfmt::skip]
pub mod modint {
    use std::ops::*;
    use crate::def_op;

    pub trait Modulus: Clone + Copy + PartialEq + Eq + Default {
        fn m() -> u32;
        fn wrap(x: u64) -> u32 { (x % Self::m() as u64) as u32 }
        fn unwrap(x: u32) -> u32 { x }
        #[inline(always)] fn reduce(x: u64) -> u32 { (x % (Self::m() as u64)) as u32 }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Default)]
    pub struct ConstModulus<const M: u32>;
    impl<const M: u32> Modulus for ConstModulus<M> { fn m() -> u32{ M } }

    #[derive(Clone, Copy, PartialEq, Eq, Default)]
    pub struct MontgomeryModulus<const M: u32>;
    impl<const M: u32> MontgomeryModulus<M> {
        pub const M_U64: u64 = M as u64;
        pub const B: u64 = 32;
        pub const R: u64 = (1u64 << Self::B) % Self::M_U64;
        pub const R2: u64 = (Self::R * Self::R) % Self::M_U64;
        pub const MASK: u64 = (1u64 << Self::B) - 1;
        pub const INV_M: u64 = {
            let (mut x, mut y) = (1u32, 0u32);
            while x != y { y = x; x = x.wrapping_mul(2u32.wrapping_sub(M.wrapping_mul(x))); }
            x as u64
        };
    }
    impl<const M: u32> Modulus for MontgomeryModulus<M> {
        fn m() -> u32{ M }
        fn wrap(x: u64) -> u32 { Self::reduce((x % Self::M_U64).wrapping_mul(Self::R2)) }
        fn unwrap(x: u32) -> u32 { Self::reduce(x as u64) }
        fn reduce(x: u64) -> u32 {
            let q = ((x & Self::MASK) * Self::INV_M) & Self::MASK;
            let mut r = x.wrapping_sub(q * Self::M_U64) >> Self::B;
            if r >= Self::M_U64 { r = r.wrapping_add(Self::M_U64); }
            r as u32
        }
    }

    #[repr(C)]
    #[derive(Clone, Copy, Eq, PartialEq, Default)]
    pub struct ModInt<Mod: Modulus>(pub u32, pub Mod);

    #[allow(dead_code)]
    impl<Mod: Modulus> ModInt<Mod> {
        pub fn m() -> u32{ Mod::m() }
        pub fn raw(x: u32) -> Self { Self(x, Mod::default()) }
        pub fn inner(&self) -> u32 { Mod::unwrap(self.0) }
        pub fn pow(mut self, mut e: usize) -> Self {
            let mut result = Self::from(1);
            while e > 0 {
                if e & 1 == 1 { result *= self; }
                self *= self;
                e >>= 1;
            }
            result
        }
        pub fn inv(self) -> Self { self.pow((Mod::m() - 2) as usize) }
    }

    def_op!(ModInt::<Mod: Modulus>: Add::add | AddAssign::add_assign => (&mut self, rhs) { self.0 += rhs.into().0; if self.0 >= Mod::m() { self.0 -= Mod::m(); } });
    def_op!(ModInt::<Mod: Modulus>: Sub::sub | SubAssign::sub_assign => (&mut self, rhs) { self.0 = self.0.wrapping_sub(rhs.into().0); if self.0 >= Mod::m() { self.0 = self.0.wrapping_add(Mod::m()); } });
    def_op!(ModInt::<Mod: Modulus>: Mul::mul | MulAssign::mul_assign => (&mut self, rhs) { self.0 = Mod::reduce((self.0 as u64).wrapping_mul(rhs.into().0 as u64)) });
    def_op!(ModInt::<Mod: Modulus>: Div::div | DivAssign::div_assign => (&mut self, rhs) { *self *= rhs.into().inv(); });

    impl<Mod: Modulus> From<u32> for ModInt<Mod> { fn from(x: u32) -> Self { (x as u64).into() } }
    impl<Mod: Modulus> From<usize> for ModInt<Mod> { fn from(x: usize) -> Self { (x as u64).into() } }
    impl<Mod: Modulus> From<String> for ModInt<Mod> { fn from(s: String) -> Self { s.parse::<u64>().unwrap().into() } }
    impl<Mod: Modulus> From<i32> for ModInt<Mod> { fn from(x: i32) -> Self { (x as i64).into() } }
    impl<Mod: Modulus> From<isize> for ModInt<Mod> { fn from(x: isize) -> Self { (x as i64).into() } }

    impl<Mod: Modulus> From<u64> for ModInt<Mod> { fn from(x: u64) -> Self { Self::raw(Mod::wrap(x)) } }
    impl<Mod: Modulus> From<i64> for ModInt<Mod> {
        fn from(mut x: i64) -> Self {
            x %= Mod::m() as i64;
            if x < 0 { x += Mod::m() as i64; }
            (x as u64).into()
        }
    }
    impl<Mod: Modulus> std::fmt::Display for ModInt<Mod> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> { write!(f, "{}", self.inner()) }
    }

    impl<Mod: Modulus> std::fmt::Debug for ModInt<Mod> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> { write!(f, "mint({})", self.inner()) }
    }
}

#[allow(unused_imports)]
use modint::*;

#[allow(dead_code)]
type Mint = ModInt<MontgomeryModulus<998_244_353>>;
// type Mint = ModInt<ConstModulus<998_244_353>>;
// type Mint = ModInt<ConstModulus<1_000_000_007>>;

#[cfg(test)]
mod test {
    use super::modint::*;

    macro_rules! test_cases {
        ($mint: ty) => {
            type MintType = $mint;

            #[test]
            fn test_modint() {
                let a = MintType::from(42);
                let b = MintType::from(43);
                assert_eq!((a + b).inner(), 85, "Op +");
                assert_eq!((a - b).inner(), MintType::m() - 1, "Op -");
                assert_eq!((a * b).inner(), 1806, "Op *");
                assert_eq!((a / b).inner(), 487_514_685, "Op /");
            }

            #[test]
            fn test_with_primitive() {
                let a = MintType::from(42);
                let b = 43;
                assert_eq!((a + b).inner(), 85, "Op +");
                assert_eq!((a - b).inner(), MintType::m() - 1, "Op -");
                assert_eq!((a * b).inner(), 1806, "Op *");
                assert_eq!((a / b).inner(), 487_514_685, "Op /");
            }

            #[test]
            fn test_assign_ops() {
                let mut a = MintType::from(42);
                let b = MintType::from(43);
                a += b;
                assert_eq!(a.inner(), 85, "Op +=");
                a -= b;
                assert_eq!(a.inner(), 42, "Op -=");
                a *= b;
                assert_eq!(a.inner(), 1806, "Op *=");
                a /= b;
                assert_eq!(a.inner(), 42, "Op /=");
            }

            #[test]
            fn test_pow() {
                let a = MintType::from(42);
                assert_eq!(a.pow(0).inner(), 1, "Pow 0");
                assert_eq!(a.pow(1).inner(), 42, "Pow 1");
                assert_eq!(a.pow(2).inner(), 1764, "Pow 2");
                assert_eq!(a.pow(3).inner(), 74_088, "Pow 3");
            }
        };
    }

    mod test_const_modulus_998244353 {
        test_cases! { super::ModInt<super::ConstModulus<998_244_353>>}
    }
    mod test_montgomery_modulus_998244353 {
        #[test]
        fn test_wrap_unwrap() {
            type MintType = super::ModInt<super::MontgomeryModulus<998_244_353>>;
            use crate::prng::create_prng;
            let mut rng = create_prng(0);

            for val in [0, 1, 2, 10, 20, 100, 123456]
                .iter()
                .copied()
                .chain((0..1000).map(|_| rng() % 998_244_353))
            {
                let wrapped = MintType::from(val);
                let unwrapped = wrapped.inner();
                assert_eq!(val as u32, unwrapped, "Montgomery wrap/unwrap {}", val);
            }
        }

        test_cases! {super::ModInt<super::MontgomeryModulus<998_244_353>>}
    }
}
