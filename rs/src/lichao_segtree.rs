#![allow(dead_code)]
//// This template has unroll operation. Remove that for better performance

const INF: i64 = 10i64.pow(15);

#[derive(Debug, Clone, Copy)]
struct Line {
    a: i64,
    b: i64,
}

impl Line {
    fn eval(&self, x: i64) -> i64 {
        self.a * x + self.b
    }

    fn smallest() -> Self {
        Self { a: 0, b: -INF }
    }

    fn zero() -> Self {
        Self { a: 0, b: 0 }
    }

    fn add(self, other: Self) -> Self {
        Self { a: self.a + other.a, b: self.b + other.b }
    }
}

struct LichaoSegtree {
    n: usize,
    data: Vec<Line>,
    modified_list: Vec<usize>,
    modified: Vec<bool>,
}

impl LichaoSegtree {
    fn new(n: usize) -> Self {
        let data = vec![Line::smallest(); n * 4];
        Self { n, data, modified_list: vec![], modified: vec![false; n * 4] }
    }

    fn insert(&mut self, l: Line) {
        self._insert(l, 1, 0, self.n);
    }

    fn _insert(&mut self, mut ln: Line, i: usize, l: usize, r: usize) {
        let mid = (l + r) / 2;
        if ln.eval(mid as i64) > self.data[i].eval(mid as i64) {
            eprintln!("insert {ln:?} at {i}");
            std::mem::swap(&mut self.data[i], &mut ln);
            if !self.modified[i] {
                self.modified[i] = true;
                self.modified_list.push(i);
            }
        }
        let l_bigger = ln.eval(l as i64) > self.data[i].eval(l as i64);
        let r_bigger = ln.eval(r as i64 - 1) > self.data[i].eval(r as i64 - 1);
        match (l_bigger, r_bigger) {
            (true, false) => self._insert(self.data[i], i * 2, l, mid),
            (false, true) => self._insert(ln, i * 2 + 1, mid, r),
            (true, true) => panic!("wtf"),
            (false, false) => { /* nothing */ }
        }
    }

    fn query(&mut self, x: usize) -> i64 {
        self._query(x, 1, 0, self.n)
    }

    fn _query(&mut self, x: usize, i: usize, l: usize, r: usize) -> i64 {
        let ans = self.data[i].eval(x as i64);
        if r - l <= 1 {
            return ans;
        }

        let mid = (l + r) / 2;
        if x < mid {
            self._query(x, i * 2, l, mid).max(ans)
        } else {
            self._query(x, i * 2 + 1, mid, r).max(ans)
        }
    }

    fn reset(&mut self) {
        for i in self.modified_list.iter() {
            self.data[*i] = Line::smallest();
            self.modified[*i] = false;
        }
        self.modified_list.clear();
    }
}
