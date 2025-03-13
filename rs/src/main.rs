#![allow(unexpected_cfgs, unused_macros, unused_imports)]
use std::{
    cmp::*,
    collections::*,
    io::{stderr, stdin, stdout, BufRead, Write},
    mem::*,
    str::*,
};

macro_rules! dbg { ($($arg:tt)*) => { if cfg!(LOCAL) { std::dbg!($($arg)*); } }; }
macro_rules! eprintln { ($($arg:tt)*) => { if cfg!(LOCAL) { std::eprintln!($($arg)*); } }; }

fn main() {
    let mut scan = Scan::new();
    // let mut writer = stdout();  // for interactive
    let stdout = stdout().lock();
    let mut writer = std::io::BufWriter::new(stdout);

    // let ntest = 1;
    let ntest: usize = scan.next();
    for testcase in 1..=ntest {
        eprintln!("==== testcase {testcase} ====");
        let n: usize = scan.next();
        let a: Vec<usize> = (0..n).map(|_| scan.next()).collect();
    }
}
//{{{
struct Scan {
    stdin: std::io::StdinLock<'static>,
    buff: Vec<String>,
}
#[allow(dead_code)]
#[allow(unused_variables)]
impl Scan {
    fn new() -> Self {
        return Self { stdin: std::io::stdin().lock(), buff: vec![] };
    }

    fn next<T: FromStr>(&mut self) -> T {
        return self.next_opt().unwrap();
    }

    fn next_opt<T: FromStr>(&mut self) -> Option<T> {
        if let Some(token) = self.buff.pop() {
            return token.parse().ok();
        }
        if let Some(line) = self.read_line() {
            self.buff = line.split_ascii_whitespace().map(String::from).rev().collect();
            return self.next_opt();
        } else {
            return None;
        }
    }

    fn read_line(&mut self) -> Option<String> {
        let mut line = String::new();
        return self.stdin.read_line(&mut line).map(|_| line).ok();
    }

    // empty line will be consumed too
    fn read_line_till_empty(&mut self) -> Option<String> {
        self.read_line().filter(|line| !line.is_empty())
    }
}

// Helpers
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(dead_code)]
struct IOrd<T>(T);
impl<T: Ord> Ord for IOrd<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl<T: PartialOrd> PartialOrd for IOrd<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
} //}}}
