// The interface was made compatible with the original SegTree
// The tree does not maintain l and r so 0, 0 is passed to merge and apply_lazy methods instead.
// Used in https://codeforces.com/contest/650/submission/325407174

#[allow(dead_code)]
trait SegTreeData: Default + Copy {
    type FromType;
    type Lazy: Copy;

    fn from(value: &Self::FromType, pos: usize) -> Self;
    fn merge(rhs: &Self, lhs: &Self, l: usize, r: usize) -> Self;
    fn accept_lazy(&mut self, lazy: Self::Lazy);
    fn apply_lazy(&mut self, l: usize, r: usize) -> Self::Lazy;
    fn none() -> Self {
        Self::default()
    }
}

#[allow(dead_code)]
struct IterativeSegTree<Data: SegTreeData> {
    n: usize,
    data: Vec<Data>,
}

#[allow(dead_code)]
impl<Data: SegTreeData> IterativeSegTree<Data> {
    pub fn new(n: usize) -> Self {
        Self { n, data: vec![Data::default(); 2 * n] }
    }

    pub fn build(arr: &[Data::FromType]) -> Self {
        let n = arr.len();
        let mut tree = Self::new(n);
        for i in 0..n {
            tree.data[n + i] = Data::from(&arr[i], i);
        }
        for i in (0..n).rev() {
            tree.data[i] = Data::merge(&tree.data[2 * i], &tree.data[2 * i + 1], 0, 0);
        }
        tree
    }

    pub fn height(&self) -> usize {
        self.n.next_power_of_two().trailing_zeros() as usize
    }

    fn push_all_lazy(&mut self, u: usize) {
        for i in (1..=self.height()).rev() {
            let v = u >> i;
            let lazy = self.data[v].apply_lazy(0, 0);
            self.data[2 * v].accept_lazy(lazy);
            self.data[2 * v + 1].accept_lazy(lazy);
        }
        self.data[u].apply_lazy(0, 0); // apply lazy to the node itself
    }

    fn rebuild(&mut self, mut u: usize) {
        while u > 1 {
            u >>= 1;
            self.data[u] = Data::merge(&self.data[2 * u], &self.data[2 * u + 1], 0, 0);
        }
    }

    pub fn range_upd(&mut self, mut l: usize, mut r: usize, val: Data::Lazy) {
        l += self.n;
        r += self.n;
        let (l0, r0) = (l, r);

        while l < r {
            if l & 1 == 1 {
                self.data[l].accept_lazy(val);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                self.data[r].accept_lazy(val);
            }
            l >>= 1;
            r >>= 1;
        }

        self.push_all_lazy(l0);
        self.push_all_lazy(r0 - 1);

        self.rebuild(l0);
        self.rebuild(r0 - 1);
    }

    pub fn point_upd(&mut self, pos: usize, val: Data::Lazy) {
        let u = pos + self.n;
        self.data[u].accept_lazy(val);
        self.push_all_lazy(u);
        self.rebuild(u);
    }

    pub fn range_query(&mut self, mut l: usize, mut r: usize) -> Data {
        l += self.n;
        r += self.n;

        self.push_all_lazy(l);
        self.push_all_lazy(r - 1);

        let (mut left_res, mut right_res) = (Data::none(), Data::none());
        while l < r {
            if l & 1 == 1 {
                left_res = Data::merge(&left_res, &self.data[l], 0, 0);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                right_res = Data::merge(&self.data[r], &right_res, 0, 0);
            }
            l >>= 1;
            r >>= 1;
        }
        Data::merge(&left_res, &right_res, 0, 0)
    }

    pub fn query_whole_tree(&mut self) -> Data {
        // can be proven that we do not need to push lazy here
        self.data[1]
    }

    // for debug only
    pub fn to_vec(&mut self) -> Vec<Data> {
        let mut res = vec![Data::none(); self.n];
        for i in 0..self.n {
            self.push_all_lazy(i + self.n);
            res[i] = self.range_query(i, i + 1);
        }
        res
    }
}
