#![allow(dead_code)]
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::ops::{Bound, RangeBounds};
pub struct PRNG(DefaultHasher);

macro_rules! get_bound {
    ($r: ident, $t: ident) => {{
        let start = match $r.start_bound() {
            Bound::Included(&s) => s,
            Bound::Excluded(&s) => s + 1,
            Bound::Unbounded => $t::MIN,
        };
        let end = match $r.end_bound() {
            Bound::Included(&e) => e + 1,
            Bound::Excluded(&e) => e,
            Bound::Unbounded => $t::MAX,
        };
        assert!(start < end, "rand_usize: start < end");
        (start, end)
    }};
}

impl PRNG {
    pub fn new(seed: usize) -> Self {
        let mut h = DefaultHasher::new();
        h.write_usize(seed);
        Self(h)
    }

    pub fn next(&mut self) -> usize {
        let x = self.0.finish() as usize;
        self.0.write_usize(x);
        return x;
    }

    pub fn rand_usize(&mut self, r: impl RangeBounds<usize>) -> usize {
        let (start, end) = get_bound!(r, usize);
        self.next() % (end - start) + start
    }

    pub fn rand_i64(&mut self, r: impl RangeBounds<i64>) -> i64 {
        let (start, end) = get_bound!(r, i64);
        let len = end.wrapping_sub(start) as usize;
        (self.next() % len).wrapping_add_signed(start as isize) as i64
    }
}

pub fn seed_from_time_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    return SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_same_seed() {
        let mut prng1 = PRNG::new(42);
        let mut prng2 = PRNG::new(42);

        for _ in 0..100 {
            assert_eq!(prng1.next(), prng2.next());
        }
    }

    #[test]
    fn test_different_seed() {
        let mut prng1 = PRNG::new(42);
        let mut prng2 = PRNG::new(43);

        for _ in 0..100 {
            assert_ne!(prng1.next(), prng2.next());
        }
    }

    #[test]
    fn test_random_enough_value() {
        let mut prng = PRNG::new(42);
        let set: std::collections::HashSet<usize> =
            std::iter::repeat_with(|| prng.next()).take(10000).collect();
        assert_eq!(set.len(), 10000);
    }

    #[test]
    fn test_rand_usize() {
        let mut prng = PRNG::new(42);
        let r = 69..420;
        for _ in 0..1000 {
            let val = prng.rand_usize(r.clone());
            assert!(r.contains(&val));
        }
    }

    #[test]
    fn test_good_seed() {
        // to avoid being hack on Codeforces
        let seed1 = seed_from_time_ms();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let seed2 = seed_from_time_ms();
        assert_ne!(seed1, seed2);
    }
}
