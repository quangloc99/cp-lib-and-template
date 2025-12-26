use crate::modint::modint::*;
use crate::number_trait::*;

trait FFTOmega: Number {
    fn omega(lv: usize) -> Self;
}

fn fft<Num: FFTOmega>(a: &mut [Num]) {
    let n = a.len();
    let l = n.trailing_zeros() as usize;
    assert!(n == 1 << l);

    {
        let mut rev_i = 0;
        for i in 0..n {
            if rev_i < i {
                (a[i], a[rev_i]) = (a[rev_i], a[i]);
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

    for lv in 1..=l {
        let s = 1 << lv;
        let omega = Num::omega(lv);
        for start in (0..n).step_by(s) {
            let mut w = Num::one();
            for (i, j) in (s / 2..s).enumerate() {
                let (u, v) = (a[i + start], a[j + start] * w);
                (a[i + start], a[j + start]) = (u + v, u - v);
                w = w * omega;
            }
        }
    }
}

fn convolution<Num: FFTOmega>(mut a: Vec<Num>, mut b: Vec<Num>) -> Vec<Num> {
    let s = a.len() + b.len() - 1;
    let n = s.next_power_of_two();
    a.resize(n, Num::zero());
    b.resize(n, Num::zero());
    fft(&mut a);
    fft(&mut b);
    let inv = Num::one() / Num::from(n);
    for i in 0..n {
        a[i] = a[i] * b[i] * inv;
    }
    a[1..].reverse();
    fft(&mut a);
    a.resize(s, Num::zero());
    a
}

impl FFTOmega for ModInt<ConstModulus<998_244_353>> {
    fn omega(lv: usize) -> Self {
        // For p < 2^30 there is also e.g. 5 << 25, 7 << 26, 479 << 21
        // and 483 << 21 (same root). The last two are > 10^9.

        let pw = Self::m() >> lv; // we actually need (Self::m() - 1) >> lv, but we never touch lv == 0
        Self::from(62).pow(pw)
    }
}

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
