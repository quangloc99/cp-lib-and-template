#![allow(dead_code)]
fn create_prng(seed: u64) -> impl Iterator<Item = u64> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;
    let mut hasher = DefaultHasher::new();
    hasher.write_u64(seed);

    return std::iter::repeat_with(move || {
        let x = hasher.finish();
        hasher.write_u64(x);
        return x;
    });
}

fn seed_from_time_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_same_seed() {
        let prng1 = create_prng(42);
        let prng2 = create_prng(42);

        for (a, b) in prng1.zip(prng2).take(100) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn test_different_seed() {
        let prng1 = create_prng(42);
        let prng2 = create_prng(43);

        for (a, b) in prng1.zip(prng2).take(100) {
            assert_ne!(a, b);
        }
    }

    #[test]
    fn test_random_enough_value() {
        let set: std::collections::HashSet<u64> = create_prng(42).take(10000).collect();
        assert_eq!(set.len(), 10000);
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
