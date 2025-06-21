#[allow(unused)]
struct Fenwick {
    n: usize,
    data: Vec<usize>,
}

#[allow(unused)]
impl Fenwick {
    fn new(n: usize) -> Self {
        Self { n, data: vec![0; n] }
    }

    fn upd(&mut self, mut i: usize, delta: isize) {
        while i < self.n {
            self.data[i] = self.data[i].checked_add_signed(delta).unwrap();
            i += Fenwick::cov_len(i);
        }
    }

    fn get(&self, mut i: usize) -> usize {
        i = i.min(self.n);
        let mut res = 0;
        let mut jump = 1;
        while i >= jump {
            i -= jump;
            res += self.data[i];
            jump = Fenwick::cov_len(i);
        }
        res
    }

    fn cov_len(i: usize) -> usize {
        (i + 1) & !i
    }
}
