use crate::number_trait::*;

#[allow(dead_code)]
fn gcd<T: Number>(mut u: T, mut v: T) -> T {
    while !v.is_zero() {
        (u, v) = (v, u % v);
    }
    u
}

#[allow(dead_code)]
fn lcm<T: Number>(u: T, v: T) -> T {
    if u.is_zero() || v.is_zero() {
        return T::zero();
    }
    u / gcd(u, v) * v
}

// return (g, x, y), where:
// - g = gcd(u, v)
// - x, y must satisfies: (u * x + v * y) % m == g % m
#[allow(dead_code)]
fn exgcd<T: Number>(u: T, v: T, m: T) -> (T, T, T) {
    // u * x + v * y == g
    // v * x1 + (u % v) * y1 == g
    // (v * (u / v) + u % v) * x + v * y == g
    // x == y1
    // u * q * x + v * y == v * x1
    // y = (v * x1 - v * q * x) / v

    if v.is_zero() {
        return (u, T::one(), T::zero());
    }
    let (g, x1, y1) = exgcd(v, u % v, m);
    let q = u / v;
    let x = y1;
    let y = (m + x1 - q * x % m) % m;
    return (g, x, y);
}

// Find x such that a * x == b (mod m)
// Return x and m / gcd(a, m)
// Note: passing b == 1 to find modulo inversion
#[allow(dead_code)]
fn solve_congruent<T: Number>(a: T, b: T, m: T) -> Option<(T, T)> {
    if a.is_zero() {
        return b.is_zero().then_some((T::zero(), T::one()));
    }
    let (g, x, _) = exgcd(a, m, m);
    let m = m / g;
    (b % g).is_zero().then_some((x * (b / g) % m, m))
}

// Find k such that
// k * a = r (mod h)
// k * b = c (mod w)
// This is basically CRT with extra steps
#[allow(dead_code)]
fn solve_congruent_2<T: Number>(a: T, r: T, h: T, b: T, c: T, w: T) -> Option<T> {
    let (r, h) = solve_congruent(a, r, h)?;
    let (c, w) = solve_congruent(b, c, w)?;
    // k = r (mod h)
    // k = c (mod w)
    // k = r + h * x
    // r + h * x = c (mod w)
    // h * x = c - r (mod w)
    let (x, _) = solve_congruent(h, (c + w - r % w) % w, w)?;
    let k = r + h * x;
    Some(k)
}

#[cfg(test)]
pub mod test {
    #[test]
    pub fn test_gcd() {
        assert_eq!(super::gcd(0, 0), 0, "special case gcd(0, 0)");
        const M: usize = 100;
        for u in 0..M {
            for v in 0..M {
                if u == 0 || v == 0 {
                    continue;
                }
                let expected = (1..=M).rev().find(|&x| u % x == 0 && v % x == 0).unwrap();
                assert_eq!(super::gcd(u, v), expected, "testing u = {}, v = {}", u, v);
            }
        }
    }

    #[test]
    pub fn test_lcm() {
        const M: usize = 100;
        for u in 0..M {
            for v in 0..M {
                let expected = match (u, v) {
                    (0, _) => 0,
                    (_, 0) => 0,
                    _ => (1..M * M).find(|&x| x % u == 0 && x % v == 0).unwrap(),
                };
                assert_eq!(super::lcm(u, v), expected, "testing u = {}, v = {}", u, v);
            }
        }
    }

    #[test]
    pub fn test_exgcd() {
        const M: usize = 100;
        for u in 0..10 {
            for v in 0..10 {
                let (g, x, y) = super::exgcd(u, v, M);
                assert_eq!(g, super::gcd(u, v), "testing u = {}, v = {}", u, v);
                assert_eq!((u * x + v * y) % M, g % M, "testing u = {}, v = {}", u, v);
            }
        }
    }

    #[test]
    pub fn test_solve_congruent() {
        for a in 0usize..100 {
            for m in 1usize..100 {
                for b in 0usize..m {
                    let expected_x = (0usize..m).find(|&x| (a * x) % m == b);
                    let expected_m = m / super::gcd(a, m);
                    let expected = expected_x.map(|x| (x, expected_m));
                    let actual = super::solve_congruent(a, b, m);
                    assert_eq!(expected, actual, "testing a = {}, b = {}, m = {}", a, b, m);
                }
            }
        }
    }

    #[test]
    pub fn test_solve_congruent_2_small() {
        const M: usize = 10;
        for (m1, m2) in (1..M).flat_map(|i| (1..M).map(move |j| (i, j))) {
            for (a, b) in (1..M).flat_map(|i| (1..M).map(move |j| (i, j))) {
                for (r, c) in (0..M).flat_map(|i| (0..M).map(move |j| (i, j))) {
                    congruent_2_test(a, r, m1, b, c, m2);
                }
            }
        }
    }

    #[test]
    pub fn test_solve_congruent_2_random() {
        use crate::prng::*;
        let mut rng = create_prng(0);
        const M: usize = 1000;
        for _ in 0..100 {
            let a = rng() as usize % M + 1;
            let b = rng() as usize % M + 1;
            let m1 = rng() as usize % M + 1;
            let r = rng() as usize % M;
            let m2 = rng() as usize % M + 1;
            let c = rng() as usize % M;
            congruent_2_test(a, r, m1, b, c, m2);
        }
    }

    fn congruent_2_test(a: usize, r: usize, h: usize, b: usize, c: usize, w: usize) {
        let expected = (0..w * h).find(|&k| (a * k) % h == r % h && (b * k) % w == c % w);
        let actual = super::solve_congruent_2(a, r, h, b, c, w);
        assert_eq!(
            expected, actual,
            "testing a = {}, r = {}, h = {}, b = {}, c = {}, w = {}",
            a, r, h, b, c, w
        );
    }
}
