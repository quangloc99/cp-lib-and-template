use crate::modint::modint::*;
use crate::number_trait::*;

pub trait FFTOmegaNum: Number {
    type Omega: FFTOmega<Self>;
    fn omega(max_lv: usize) -> Self::Omega;
}

#[rustfmt::skip]
pub trait FFTOmega<Num: FFTOmegaNum> {
    type OmegaLv<'a>: Iterator<Item = Num> where Self: 'a;
    fn get_lv<'a>(&'a self, lv: usize) -> Self::OmegaLv<'a>;
}

#[rustfmt::skip]
struct FFT<Num: FFTOmegaNum> {
    rev: Vec<usize>,
    omega: Num::Omega,
}

#[rustfmt::skip]
#[allow(dead_code)]
impl<Num: FFTOmegaNum> FFT<Num> {
    pub fn new(max_lv: usize) -> Self {
        let max_lv = max_lv.max(1);
        let mut rev = vec![0; 1 << max_lv];
        for i in 0..rev.len() {
            rev[i] = (rev[i >> 1] >> 1) | ((i & 1) << (max_lv - 1));
        }
        Self { omega: Num::omega(max_lv), rev }
    }

    pub fn fft(&self, a: &mut [Num]) {
        let n = a.len();
        let l = n.trailing_zeros() as usize;
        assert!(n == 1 << l);

        let shift = self.rev.len().trailing_zeros() as usize - l;
        for i in 1..n {
            let rev_i = self.rev[i] >> shift;
            if rev_i < i { a.swap(i, rev_i); }
        }

        for lv in 1..=l {
            for chunks in a.chunks_exact_mut(1 << lv) {
                let (ci, cj) = chunks.split_at_mut(1 << lv >> 1);
                for ((au, av), w) in ci.iter_mut().zip(cj.iter_mut()).zip(self.omega.get_lv(lv)) {
                    let (u, vw) = (*au, *av * w);
                    (*au, *av) = (u + vw, u - vw);
                }
            }
        }
    }

    pub fn convolution(&self, a: Vec<Num>, b: Vec<Num>) -> Vec<Num> {
        if a.len().min(b.len()) <= 150 { self.brute_force_convolution(a, b) } else { self.fft_convolution(a, b) }
    }

    pub fn fft_convolution(&self, a: Vec<Num>, mut b: Vec<Num>) -> Vec<Num> {
        let s = a.len() + b.len() - 1;
        let n = s.next_power_of_two();
        b.resize(n, Num::zero());
        self.fft(&mut b);
        self.partial_conv(a, &b)
    }

    /// Here b is already convoluted. Can help save one fft and also memory
    pub fn partial_conv(&self, mut a: Vec<Num>, conv_b: &[Num]) -> Vec<Num> {
        let n = conv_b.len();
        assert!(n.is_power_of_two() && a.len() <= n);
        a.resize(n, Num::zero());
        if n == 1 {
            a[0] = a[0] * conv_b[0];
        } else {
            self.fft(&mut a);
            let inv = Num::one() / Num::from(n);
            (a[0], a[n / 2]) = (a[0] * conv_b[0] * inv, a[n / 2] * conv_b[n / 2] * inv);
            for i in 1..n / 2 {
                let j = n - i;
                (a[j], a[i]) = (a[i] * conv_b[i] * inv, a[j] * conv_b[j] * inv);
            }
            self.fft(&mut a);
        }

        while a.last() == Some(&Num::zero()) { a.pop(); }
        a
    }

    pub fn brute_force_convolution(&self, mut a: Vec<Num>, b: Vec<Num>) -> Vec<Num> {
        if a.len() < b.len() { return self.brute_force_convolution(b, a); }
        if a.is_empty() || b.is_empty() { return a; }
        let (max_ai, s) = (a.len() - 1, a.len() + b.len() - 1);
        a.resize(s, Num::zero());
        for i in (0..s).rev() {
            let start = i.checked_sub(max_ai).unwrap_or(0);
            a[i] = (start..b.len().min(i + 1)).fold(Num::zero(), |acc, j| acc + a[i - j] * b[j]);
        }
        while a.last() == Some(&Num::zero()) { a.pop(); }
        a
    }
}

pub struct MintOmega<M: 'static + Modulus> {
    precal: Vec<ModInt<M>>,
}
#[rustfmt::skip]
impl<M: 'static + Modulus> FFTOmega<ModInt<M>> for MintOmega<M> where ModInt<M>: FFTOmegaNum {
    type OmegaLv<'a> = std::iter::Copied<std::slice::Iter<'a, ModInt<M>>>;
    #[inline(always)] fn get_lv<'a>(&'a self, lv: usize) -> Self::OmegaLv<'a> { self.precal[1 << lv..2 << lv].iter().copied() }
}

macro_rules! ModIntFFTOmegaNumImpl {
    ($(($MOD:expr, $ROOT:expr))*) => {
        $(impl FFTOmegaNum for ModInt<MontgomeryModulus<$MOD>> {
            type Omega = MintOmega<MontgomeryModulus<$MOD>>;
            fn omega(max_lv: usize) -> Self::Omega {
                let max_lv = max_lv + 1;
                type Mint = ModInt<MontgomeryModulus<$MOD>>;
                assert!(max_lv <= ($MOD - 1usize).trailing_zeros() as usize);
                let mut precal = vec![Mint::one(); 1 << max_lv];
                for lv in 0..max_lv {
                    let omega = Mint::from($ROOT).pow($MOD as usize >> lv);
                    for i in ((1 << lv) + 1)..2 << lv { precal[i] = precal[i - 1] * omega; }
                }
                MintOmega { precal }
            }
        })*
    }
}

// the last 2 are > 10^9
ModIntFFTOmegaNumImpl! {(998_244_353, 3) ({5 << 25 + 1}, 62) ({7 << 26 + 1}, 62) ({479 << 21 + 1}, 62) ({483 << 21 + 1}, 62)}

#[cfg(test)]
pub mod test_fft {
    use super::super::prng::*;
    use super::*;

    type Mint = ModInt<MontgomeryModulus<998_244_353>>;

    #[test]
    fn test_fft_convolution_very_small() {
        _test(1000, 5, 10);
    }

    #[test]
    fn test_fft_convolution_small() {
        _test(100, 64, 100);
    }

    #[test]
    fn test_fft_convolution_medium() {
        _test(30, 1024, 10000);
    }

    fn _test(num_cases: usize, n_max: usize, val_max: usize) {
        let fft = FFT::<Mint>::new(20);
        for testcase in 0..num_cases {
            let mut rng = create_prng(testcase as u64);
            let n = (rng() as usize % n_max) + 1;
            let m = (rng() as usize % n_max) + 1;
            let a: Vec<Mint> = (0..n).map(|_| Mint::from(rng() as usize % val_max)).collect();
            let b: Vec<Mint> = (0..m).map(|_| Mint::from(rng() as usize % val_max)).collect();
            let expected = checker_convolution(&a, &b);
            let actual = fft.convolution(a.clone(), b.clone());
            let actual_fft_only = fft.fft_convolution(a.clone(), b.clone());
            assert_eq!(expected, actual, "testing\na={:?}\nb={:?}\ntestcase={}", a, b, testcase);
            assert_eq!(
                expected, actual_fft_only,
                "testing\na={:?}\nb={:?}\ntestcase={}",
                a, b, testcase
            );
        }
    }

    fn checker_convolution(a: &[Mint], b: &[Mint]) -> Vec<Mint> {
        let s = a.len() + b.len() - 1;
        let mut c = vec![Mint::zero(); s];
        for i in 0..a.len() {
            for j in 0..b.len() {
                c[i + j] += a[i] * b[j];
            }
        }

        while c.last() == Some(&Mint::zero()) {
            c.pop();
        }
        c
    }
}

/*
// Derivation from Cooley-Tukey FFT
//
// res[i] = f(omega^i)
// - mul by omega is rotating ccw
// - mul by omega^(n/2) is mul by -1
// f(omega^k) = even(omega^(2 * k)) + omega^k * odd(omega^(2 * k))
//            = even((omega ^ 2)^k) + omega^k * odd((omega ^ 2)^k)
// for k > n / 2:
// even((omega^2)^k) = even((omega^2)^(k - n / 2))
// omega^k * odd((omega^2)^k) = -omega^(k - n / 2) * odd((omega^2)^(k - n / 2))
fn fft(a: Vec, omega) -> Vec {
    if a.len() == 1 {
        return a;
    }
    let even, odd = /* split by position */;
    let even = fft(even, omega * omega);
    let odd = fft(odd, omega * omega);

    let omega = w(a.len());
    let mut w = 1;

    let res = vec![0; a.len()];
    for i in 0..a.len() / 2 {
        res[i] = even[i] + odd[i] * w;
        res[i + a.len() / 2] = even[i] - odd[i] * w;
        w *= omega;
    }
    return res;
}
*/
