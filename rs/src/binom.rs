use std::ops::*;
pub struct Binom<Num: Clone + Mul<Output = Num> + Div<Output = Num> + From<usize>> {
    fact: Vec<Num>,
    ifact: Vec<Num>,
}

impl<Num: Clone + Mul<Output = Num> + Div<Output = Num> + From<usize>> Binom<Num> {
    pub fn new(n: usize) -> Self {
        let mut fact = vec![Num::from(0); n];
        fact[0] = Num::from(1);
        for i in 1..n {
            fact[i] = fact[i - 1].clone() * Num::from(i);
        }

        let mut ifact = vec![Num::from(0); n];
        ifact[n - 1] = Num::from(1) / fact[n - 1].clone();
        for i in (1..n).rev() {
            ifact[i - 1] = ifact[i].clone() * Num::from(i);
        }

        Self { fact, ifact }
    }

    pub fn calc(&self, n: usize, k: usize) -> Num {
        if k > n {
            Num::from(0)
        } else {
            self.fact[n].clone() * self.ifact[n - k].clone() * self.ifact[k].clone()
        }
    }

    pub fn divide(&self, sum_part: usize, num_part: usize) -> Num {
        match (sum_part, num_part) {
            (0, 0) => Num::from(1),
            (_, 0) => Num::from(0),
            (_, _) => self.calc(sum_part + num_part - 1, num_part - 1),
        }
    }
}
