//! explore.rs — enumeration harness for finding observations.
//!
//! Drop next to your solution file and `mod explore;`. Works with plain rustc,
//! no Cargo, no crates.
//!
//!   solve:    rustc -O a.rs -o a && ./a < in
//!   explore:  rustc --test -O a.rs -o a && ./a --ignored --nocapture
//!   one probe: ./a by_answer --ignored --nocapture
//!   stress:   ./a            (non-ignored tests only)

#![allow(dead_code)]

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

// ============================================================== generators
//
// All lazy iterators, so you can .filter() / .take() / .map() them instead of
// generating everything. `arrays(4, 0, 3).filter(|a| a[0] == 0)` etc.

pub struct Arrays {
    n: usize,
    lo: i64,
    hi: i64,
    cur: Vec<i64>,
    done: bool,
}

/// every Vec of length `n` with values in `lo..=hi`
pub fn arrays(n: usize, lo: i64, hi: i64) -> Arrays {
    Arrays { n, lo, hi, cur: vec![lo; n], done: n == 0 || lo > hi }
}

impl Iterator for Arrays {
    type Item = Vec<i64>;
    fn next(&mut self) -> Option<Vec<i64>> {
        if self.done {
            return None;
        }
        let out = self.cur.clone();
        self.done = !next_arr(&mut self.cur, self.lo, self.hi);
        Some(out)
    }
}

fn next_arr(a: &mut [i64], lo: i64, hi: i64) -> bool {
    for i in 0..a.len() {
        if a[i] < hi {
            a[i] += 1;
            return true;
        }
        a[i] = lo;
    }
    false
}

pub struct Perms {
    p: Vec<usize>,
    done: bool,
}

/// every permutation of `0..n`
pub fn perms(n: usize, start: usize) -> Perms {
    Perms { p: (start..start + n).collect(), done: false }
}

impl Iterator for Perms {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Vec<usize>> {
        if self.done {
            return None;
        }
        let out = self.p.clone();
        self.done = !next_permutation(&mut self.p);
        Some(out)
    }
}

fn next_permutation(a: &mut [usize]) -> bool {
    let n = a.len();
    if n < 2 {
        return false;
    }
    let mut i = n - 1;
    while i > 0 && a[i - 1] >= a[i] {
        i -= 1;
    }
    if i == 0 {
        return false;
    }
    let mut j = n - 1;
    while a[j] <= a[i - 1] {
        j -= 1;
    }
    a.swap(i - 1, j);
    a[i..].reverse();
    true
}

/// every subset of `0..n` as a bitmask
pub fn masks(n: usize) -> impl Iterator<Item = u32> {
    0u32..(1u32 << n)
}

/// every binary string of length `n`
pub fn bits(n: usize) -> impl Iterator<Item = String> {
    masks(n).map(move |m| (0..n).map(|i| if m >> i & 1 == 1 { '1' } else { '0' }).collect())
}

fn prufer_decode(n: usize, code: &[i64]) -> Vec<(usize, usize)> {
    if n <= 1 {
        return vec![];
    }
    let mut deg = vec![1usize; n];
    for &x in code {
        deg[x as usize] += 1;
    }
    let mut leaves: BinaryHeap<Reverse<usize>> =
        (0..n).filter(|&i| deg[i] == 1).map(Reverse).collect();
    let mut e = Vec::with_capacity(n - 1);
    for &x in code {
        let x = x as usize;
        let Reverse(leaf) = leaves.pop().unwrap();
        e.push((leaf, x));
        deg[x] -= 1;
        if deg[x] == 1 {
            leaves.push(Reverse(x));
        }
    }
    let Reverse(u) = leaves.pop().unwrap();
    let Reverse(v) = leaves.pop().unwrap();
    e.push((u, v));
    e
}

/// every labelled tree on `n` nodes, as an edge list. n^(n-2) of them: keep n <= 7
pub fn trees(n: usize) -> Box<dyn Iterator<Item = Vec<(usize, usize)>>> {
    match n {
        0 | 1 => Box::new(std::iter::once(vec![])),
        2 => Box::new(std::iter::once(vec![(0, 1)])),
        _ => Box::new(arrays(n - 2, 0, (n - 1) as i64).map(move |c| prufer_decode(n, &c))),
    }
}

/// every simple undirected graph on `n` nodes. 2^(n(n-1)/2): keep n <= 5
pub fn graphs(n: usize) -> impl Iterator<Item = Vec<(usize, usize)>> {
    let all: Vec<(usize, usize)> = (0..n).flat_map(|i| ((i + 1)..n).map(move |j| (i, j))).collect();
    let m = all.len();
    (0u32..(1u32 << m))
        .map(move |mask| (0..m).filter(|i| mask >> i & 1 == 1).map(|i| all[i]).collect())
}

// ======================================================== state-space BFS
//
// The most reusable brute there is. Any problem phrased "you may repeatedly
// apply operation X" -> BFS the reachable states. Gives min-ops, reachability,
// and the optimal path in one shot.

pub fn bfs_dist<S, F, I>(start: S, moves: F) -> HashMap<S, usize>
where
    S: Clone + Eq + Hash,
    F: Fn(&S) -> I,
    I: IntoIterator<Item = S>,
{
    let mut d = HashMap::new();
    d.insert(start.clone(), 0usize);
    let mut q = VecDeque::new();
    q.push_back(start);
    while let Some(u) = q.pop_front() {
        let du = d[&u];
        for v in moves(&u) {
            if !d.contains_key(&v) {
                d.insert(v.clone(), du + 1);
                q.push_back(v);
            }
        }
    }
    d
}

/// same, but also records a parent pointer so you can reconstruct the path
pub fn bfs_path<S, F, I>(start: S, moves: F) -> HashMap<S, (usize, Option<S>)>
where
    S: Clone + Eq + Hash,
    F: Fn(&S) -> I,
    I: IntoIterator<Item = S>,
{
    let mut d: HashMap<S, (usize, Option<S>)> = HashMap::new();
    d.insert(start.clone(), (0, None));
    let mut q = VecDeque::new();
    q.push_back(start);
    while let Some(u) = q.pop_front() {
        let du = d[&u].0;
        for v in moves(&u) {
            if !d.contains_key(&v) {
                d.insert(v.clone(), (du + 1, Some(u.clone())));
                q.push_back(v);
            }
        }
    }
    d
}

// ================================================================= tabulate
//
// The staring surface. Collect (label, answer), then view it in whichever
// arrangement makes the pattern pop.

#[derive(Default)]
pub struct Table<A> {
    pub rows: Vec<(String, A)>,
}

impl<A: Ord + Clone + Debug> Table<A> {
    pub fn new() -> Self {
        Table { rows: Vec::new() }
    }

    pub fn add(&mut self, label: impl Into<String>, ans: A) {
        self.rows.push((label.into(), ans));
    }

    /// flat dump, insertion order
    pub fn dump(&self) {
        for (l, a) in &self.rows {
            println!("{}  ->  {:?}", l, a);
        }
    }

    /// GROUPED BY ANSWER. this is the one that finds invariants — look at what
    /// all the inputs sharing an answer have in common.
    pub fn by_answer(&self) {
        let mut g: BTreeMap<&A, Vec<&String>> = BTreeMap::new();
        for (l, a) in &self.rows {
            g.entry(a).or_default().push(l);
        }
        for (a, ls) in g {
            println!("== {:?}   ({})", a, ls.len());
            for l in ls {
                println!("   {}", l);
            }
        }
    }

    /// counts per answer — the shape of the distribution
    pub fn histogram(&self) {
        let mut c: BTreeMap<&A, usize> = BTreeMap::new();
        for (_, a) in &self.rows {
            *c.entry(a).or_insert(0) += 1;
        }
        for (a, n) in c {
            println!("{:?} : {}", a, n);
        }
    }

    /// comma-separated answers, ready to paste into oeis.org
    pub fn oeis(&self) {
        let s: Vec<String> = self.rows.iter().map(|(_, a)| format!("{:?}", a)).collect();
        println!("{}", s.join(", "));
    }
}

/// 2-parameter answers as a grid. diagonals and periodicity are visible here
/// and invisible in a flat list.
pub fn grid<A: Debug>(r0: i64, r1: i64, c0: i64, c1: i64, f: impl Fn(i64, i64) -> A) {
    print!("      ");
    for j in c0..=c1 {
        print!("{:>6}", j);
    }
    println!();
    for i in r0..=r1 {
        print!("{:>5} ", i);
        for j in c0..=c1 {
            print!("{:>6}", format!("{:?}", f(i, j)));
        }
        println!();
    }
}
