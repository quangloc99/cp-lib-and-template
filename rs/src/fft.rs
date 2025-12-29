use crate::modint::modint::*;
use crate::number_trait::*;

trait FFTOmegaNum: Number {
    fn omega(max_lv: usize) -> impl FFTOmega<Self>;
}

trait FFTOmega<Num: FFTOmegaNum> {
    fn set_lv(&mut self, lv: usize) -> Num;
    fn next(&mut self, w: &mut Num);
}

fn fft<Num: FFTOmegaNum>(a: &mut [Num]) {
    let n = a.len();
    let l = n.trailing_zeros() as usize;
    assert!(n == 1 << l);

    let a = a.as_mut_ptr();
    {
        let mut rev_i = 0;
        for i in 0..n {
            if rev_i < i {
                unsafe { (*a.add(i), *a.add(rev_i)) = (*a.add(rev_i), *a.add(i)) }
            }
            let mut bit = 1 << l;
            loop {
                bit >>= 1;
                rev_i ^= bit;
                if rev_i & bit == bit {
                    break;
                }
            }
        }
    }

    let mut omega = Num::omega(l);

    for lv in 1..=l {
        let s = 1 << lv;
        for start in (0..n).step_by(s) {
            unsafe {
                let mut w = omega.set_lv(lv);
                let chunk = a.add(start);
                for (i, j) in (s / 2..s).enumerate() {
                    let (au, av) = (chunk.add(i), chunk.add(j));
                    let (u, v) = (*au, *av * w);
                    (*au, *av) = (u + v, u - v);
                    omega.next(&mut w);
                }
            }
        }
    }
}

fn convolution<Num: FFTOmegaNum>(a: Vec<Num>, mut b: Vec<Num>) -> Vec<Num> {
    let s = a.len() + b.len() - 1;
    let n = s.next_power_of_two();
    b.resize(n, Num::zero());
    fft(&mut b);
    partial_conv(a, &b)
}

/// Here b is already convoluted. Can help save one fft and also memory
fn partial_conv<Num: FFTOmegaNum>(mut a: Vec<Num>, conv_b: &[Num]) -> Vec<Num> {
    let n = conv_b.len();
    assert!(n.is_power_of_two());
    assert!(a.len() <= n);
    a.resize(n, Num::zero());
    fft(&mut a);
    let inv = Num::one() / Num::from(n);
    for i in 0..n {
        a[i] = a[i] * conv_b[i] * inv;
    }
    a[1..].reverse();
    fft(&mut a);

    while a.last() == Some(&Num::zero()) {
        a.pop();
    }
    a
}

#[rustfmt::skip]
macro_rules! ModIntFFTOmegaNumImpl {
    ($(($MOD:expr, $ROOT:expr))*) => {
        $(impl FFTOmegaNum for ModInt<ConstModulus<$MOD>> {
            fn omega(_lv: usize) -> impl FFTOmega<Self> {
                type Mint = ModInt<ConstModulus<$MOD>>;
                struct Omega { precal: &'static [Mint], omega: Mint, }
                impl FFTOmega<Mint> for Omega {
                    fn set_lv(&mut self, lv: usize) -> Mint { self.omega = self.precal[lv]; Mint::one() }
                    fn next(&mut self, w: &mut Mint) { *w *= self.omega; }
                }
                use std::sync::OnceLock;
                static PRECAL: OnceLock<Vec<Mint>> = OnceLock::<Vec<Mint>>::new();
                let precal = PRECAL.get_or_init(|| (0..32).map(|i| Mint::from($ROOT).pow($MOD >> i)).collect());
                Omega { precal, omega: Mint::default() }
            }
        })*
    }
}

// the last 2 are > 10^9
ModIntFFTOmegaNumImpl! {(998_244_353, 62) ({5 << 25 + 1}, 62) ({7 << 26 + 1}, 62) ({479 << 21 + 1}, 62) ({483 << 21 + 1}, 62)}

#[cfg(test)]
pub mod test_fft {
    use super::super::prng::*;
    use super::*;

    type Mint = ModInt<ConstModulus<998_244_353>>;

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
        for testcase in 0..num_cases {
            let mut rng = create_prng(testcase as u64);
            let n = (rng() as usize % n_max) + 1;
            let m = (rng() as usize % n_max) + 1;
            let a: Vec<Mint> = (0..n).map(|_| Mint::from(rng() as usize % val_max)).collect();
            let b: Vec<Mint> = (0..m).map(|_| Mint::from(rng() as usize % val_max)).collect();
            let expected = brute_force_convolution(&a, &b);
            let actual = convolution(a.clone(), b.clone());
            assert_eq!(expected, actual, "testing\na={:?}\nb={:?}\ntestcase={}", a, b, testcase);
        }
    }

    fn brute_force_convolution(a: &[Mint], b: &[Mint]) -> Vec<Mint> {
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
