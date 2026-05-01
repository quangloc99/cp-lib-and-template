#[derive(Clone)]
struct BitSet {
    data: Vec<u64>,
}

#[rustfmt::skip]
#[allow(unused)]
impl BitSet {
    fn new(num_words: usize) -> Self { Self { data: vec![0; num_words] } }

    fn new_from_rounded_num_bits(num_bits: usize) -> Self {
        Self::new((num_bits + 63) / 64)
    }

    fn num_bits(&self) -> usize { self.data.len() * 64 }
    fn get_bit(&self, i: usize) -> bool { self.data[i / 64] & (1 << (i % 64)) != 0 }

    fn set_bit(&mut self, i: usize) -> &mut Self { self.data[i / 64] |= 1 << (i % 64); self }
    fn unset_bit(&mut self, i: usize) -> &mut Self { self.data[i / 64] &= !(1 << (i % 64)); self }
    fn set_bit_as(&mut self, i: usize, value: bool) -> &mut Self  {
        if value { self.set_bit(i) } else { self.unset_bit(i) }
    }

    fn fill_zero(&mut self) -> &mut Self { self.data.fill(0); self }

    fn shl_upd(&mut self, x: usize) -> &mut Self {
        if x == 0 { return self; }
        if x > self.num_bits() { return self.fill_zero(); }

        let (num_words, num_shift_words, num_shift_bits) = (self.data.len(), x / 64, x % 64);

        if num_shift_words > 0 {
            self.data.copy_within(0..num_words - num_shift_words, num_shift_words);
            self.data[..num_shift_words].fill(0);
        }
        if num_shift_bits > 0 {
            for i in (num_shift_words + 1..num_words).rev() {
                self.data[i] <<= num_shift_bits;
                self.data[i] |= self.data[i - 1] >> (64 - num_shift_bits);
            }
            self.data[num_shift_words] <<= num_shift_bits;
        }
        self
    }

    fn shr_upd(&mut self, x: usize) -> &mut Self {
        if x == 0 { return self; }
        if x > self.num_bits() { return self.fill_zero(); }

        let (num_words, num_shift_words, num_shift_bits) = (self.data.len(), x / 64, x % 64);

        if num_shift_words > 0 {
            self.data.copy_within(num_shift_words..num_words, 0);
            self.data[num_words - num_shift_words..].fill(0);
        }
        if num_shift_bits > 0 {
            let last_word = num_words - num_shift_words - 1;
            for i in 0..last_word {
                self.data[i] >>= num_shift_bits;
                self.data[i] |= self.data[i + 1] << (64 - num_shift_bits);
            }
            self.data[last_word] >>= num_shift_bits;
        }

        self
    }

    fn not_upd(&mut self) -> &mut Self {
        for x in self.data.iter_mut() { *x = !*x; }
        self
    }

    fn popcount(&mut self) -> usize {
        self.data.iter().map(|&x| x.count_ones() as usize).sum::<_>()
    }

    fn copy_from(&mut self, other: &Self) -> &mut Self {
        assert!(self.data.len() == other.data.len());
        self.data.copy_from_slice(&other.data);
        self
    }

    fn or_upd(&mut self, other: &BitSet) -> &mut Self {
        assert!(self.data.len() == other.data.len());
        self.data.iter_mut().zip(other.data.iter()).for_each(|(a, b)| *a |= *b);
        self
    }

    fn and_upd(&mut self, other: &BitSet) -> &mut Self {
        assert!(self.data.len() == other.data.len());
        self.data.iter_mut().zip(other.data.iter()).for_each(|(a, b)| *a &= *b);
        self
    }

    fn xor_upd(&mut self, other: &BitSet) -> &mut Self {
        assert!(self.data.len() == other.data.len());
        self.data.iter_mut().zip(other.data.iter()).for_each(|(a, b)| *a ^= *b);
        self
    }
}

impl std::fmt::Debug for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for word in self.data.iter().rev() {
            s.push_str(&format!("{:064b}", word));
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod test {
    use super::BitSet;
    use crate::prng;

    #[test]
    fn test_shl() {
        let mut rng = prng::PRNG::new(0);
        for _test_case in 0..5000 {
            let num_words = rng.rand_usize(1..=128);
            let mut bs = BitSet::new(num_words);
            let n = rng.rand_usize(..100);
            let mut values = (0..n).map(|_| rng.rand_usize(..bs.num_bits())).collect::<Vec<_>>();
            let shift = rng.rand_usize(..=bs.num_bits());

            for &x in &values {
                bs.set_bit(x);
            }
            bs.shl_upd(shift);
            values.retain_mut(|x| {
                *x += shift;
                *x < bs.num_bits()
            });

            for bit in 0..bs.num_bits() {
                assert_eq!(bs.get_bit(bit), values.contains(&bit), "bit mismatched {bit}");
            }
        }
    }

    #[test]
    fn test_shr() {
        let mut rng = prng::PRNG::new(1);
        for _test_case in 0..5000 {
            let num_words = rng.rand_usize(1..=128);
            let mut bs = BitSet::new(num_words);
            let n = rng.rand_usize(..100);
            let mut values = (0..n).map(|_| rng.rand_usize(..bs.num_bits())).collect::<Vec<_>>();
            let shift = rng.rand_usize(..=bs.num_bits());

            for &x in &values {
                bs.set_bit(x);
            }
            bs.shr_upd(shift);
            values.retain_mut(|x| {
                if *x >= shift {
                    *x -= shift;
                    true
                } else {
                    false
                }
            });

            for bit in 0..bs.num_bits() {
                assert_eq!(bs.get_bit(bit), values.contains(&bit), "bit mismatched {bit}");
            }
        }
    }
}
