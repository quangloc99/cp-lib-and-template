#![allow(dead_code)]

struct XorBasis {
    basis: Vec<usize>,
}

impl XorBasis {
    fn new(num_bit: usize) -> Self {
        Self { basis: vec![0; num_bit] }
    }

    fn insert(&mut self, mut x: usize) -> bool {
        for bit in (0..self.basis.len()).rev() {
            if x & (1 << bit) == 0 {
                continue;
            }
            if self.basis[bit] == 0 {
                self.basis[bit] = x;
                return true;
            }
            x ^= self.basis[bit];
        }
        false
    }

    fn num_basis(&self) -> usize {
        self.basis.iter().filter(|&x| *x != 0).count()
    }

    fn all_possible_values(&self) -> Vec<usize> {
        let mut res = vec![0]; // TODO: precise pre-allocation
        for &x in self.basis.iter().filter(|&&x| x != 0) {
            let mut new_res = vec![];
            for &y in &res {
                new_res.push(y ^ x);
            }
            res.extend(new_res);
        }
        res
    }
}
