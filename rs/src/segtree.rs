use std::*;

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
struct SegTree<Data: SegTreeData> {
    n: usize,
    data: Vec<Data>,
}

#[allow(dead_code)]
impl<Data: SegTreeData> SegTree<Data> {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![Data::default(); 4 * n],
        }
    }

    pub fn build(arr: &[Data::FromType]) -> Self {
        let mut new_tree = Self::new(arr.len());
        new_tree.build_rec(arr, 1, 0, new_tree.n);

        new_tree
    }

    fn build_rec(&mut self, arr: &[Data::FromType], i: usize, l: usize, r: usize) {
        if r - l == 1 {
            self.data[i] = Data::from(&arr[l], l);
            return;
        }
        let mid = (l + r) / 2;
        self.build_rec(arr, 2 * i, l, mid);
        self.build_rec(arr, 2 * i + 1, mid, r);
        self.data[i] = Data::merge(&self.data[2 * i], &self.data[2 * i + 1], l, r);
    }

    fn push_lazy(&mut self, i: usize, l: usize, r: usize) {
        let lazy = self.data[i].apply_lazy(l, r);
        if r - l > 1 {
            self.data[2 * i].accept_lazy(lazy);
            self.data[2 * i + 1].accept_lazy(lazy);
        }
    }

    pub fn range_upd(&mut self, from: usize, to: usize, val: Data::Lazy) {
        self.upd(from, to, val, 1, 0, self.n);
    }

    pub fn point_upd(&mut self, pos: usize, val: Data::Lazy) {
        self.upd(pos, pos + 1, val, 1, 0, self.n);
    }

    fn upd(&mut self, from: usize, to: usize, val: Data::Lazy, i: usize, l: usize, r: usize) {
        self.push_lazy(i, l, r);
        if from >= r || l >= to {
            return;
        }
        if from <= l && r <= to {
            self.data[i].accept_lazy(val);
            self.push_lazy(i, l, r);
            return;
        }
        let mid = (l + r) / 2;
        self.upd(from, to, val, 2 * i, l, mid);
        self.upd(from, to, val, 2 * i + 1, mid, r);
        self.data[i] = Data::merge(&self.data[2 * i], &self.data[2 * i + 1], l, r);
    }

    pub fn range_query(&mut self, from: usize, to: usize) -> Data {
        self.query(from, to, 1, 0, self.n)
    }

    pub fn get(&mut self, pos: usize) -> Data {
        self.query(pos, pos + 1, 1, 0, self.n)
    }

    fn query(&mut self, from: usize, to: usize, i: usize, l: usize, r: usize) -> Data {
        self.push_lazy(i, l, r);
        if from >= r || l >= to {
            return Data::none();
        }
        if from <= l && r <= to {
            return self.data[i];
        }
        let mid = (l + r) / 2;
        let u = &self.query(from, to, 2 * i, l, mid);
        let v = &self.query(from, to, 2 * i + 1, mid, r);
        Data::merge(u, v, l, r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seg_tree_test() {
        #[derive(Default, Clone, Copy, Debug)]
        struct Data {
            sum: i64,
            lazy: i64,
        }

        impl SegTreeData for Data {
            type FromType = i64;
            type Lazy = i64;

            fn from(&value: &i64, _pos: usize) -> Self {
                Self {
                    sum: value,
                    lazy: 0,
                }
            }
            fn merge(lhs: &Self, rhs: &Self, _l: usize, _r: usize) -> Self {
                Self {
                    sum: lhs.sum + rhs.sum,
                    lazy: 0,
                }
            }
            fn accept_lazy(&mut self, lazy: i64) {
                self.lazy = lazy;
            }
            fn apply_lazy(&mut self, l: usize, r: usize) -> i64 {
                self.sum += (r - l) as i64 * self.lazy;
                mem::replace(&mut self.lazy, 0) // remember to clear lazy
            }
        }

        let mut seg_tree = SegTree::<Data>::build(&vec![1, 2, 3, 4, 5]);

        assert_eq!(seg_tree.range_query(0, 5).sum, 15);
        assert_eq!(seg_tree.range_query(1, 4).sum, 9);
        seg_tree.range_upd(1, 4, 2);
        assert_eq!(seg_tree.range_query(0, 5).sum, 21);
        assert_eq!(seg_tree.range_query(1, 4).sum, 15);
        assert_eq!(seg_tree.get(2).sum, 5);
        seg_tree.point_upd(2, 10);
        assert_eq!(seg_tree.get(2).sum, 15);
        assert_eq!(seg_tree.range_query(0, 5).sum, 31);
    }
}
