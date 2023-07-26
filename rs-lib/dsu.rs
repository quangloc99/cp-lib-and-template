struct DSU {
    dsu: Vec<isize>,
}

impl DSU {
    fn new(size: usize) -> DSU {
        DSU {
            dsu: vec![-1isize; size],
        }
    }

    fn find_set(&mut self, u: usize) -> usize {
        if self.dsu[u] < 0 {
            return u;
        }
        self.dsu[u] = self.find_set(self.dsu[u] as usize) as isize;
        return self.dsu[u] as usize;
    }

    fn join(&mut self, u: usize, v: usize) -> bool {
        let mut u = self.find_set(u);
        let mut v = self.find_set(v);
        if u == v {
            return false;
        }
        if -self.dsu[u] < -self.dsu[v] {
            swap(&mut u, &mut v);
        }
        self.dsu[u] += self.dsu[v];
        self.dsu[v] = u as isize;
        return true;
    }

    fn same_set(&mut self, u: usize, v: usize) -> bool {
        self.find_set(u) == self.find_set(v)
    }
}
