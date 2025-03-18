#![allow(unexpected_cfgs, unused_macros, unused_imports)]
use std::{
    cmp::*,
    collections::*,
    io::{stderr, stdin, stdout, BufRead, Write},
    mem::*,
    str::*,
};

static mut DBG_INDENT: usize = 0;
#[rustfmt::skip]
macro_rules! DB { () => { let _debug_block = DBBlock::new(); }; }
macro_rules! eprintln { ($($arg:tt)*) => { if cfg!(LOCAL) { unsafe{std::eprint!("{}", "  ".repeat(DBG_INDENT));} std::eprintln!($($arg)*); } }; }
macro_rules! dbg { ($($arg:tt),*) => { eprintln!(concat!($("[", stringify!($arg), " = {:?}] "),*) $(, $arg)*) }}

fn main() {
    let mut scan = Scan::new();
    // let mut writer = stdout();  // for interactive
    let stdout = stdout().lock();
    #[allow(unused)]
    let mut writer = std::io::BufWriter::new(stdout);

    // let ntest = 1;
    let num_test: usize = scan.next();
    for test_case in 1..=num_test {
        DB!();
        dbg!(test_case);
        let n: usize = scan.next();
        let a: Vec<usize> = (0..n).map(|_| scan.next()).collect();
    }
}

////////////////////////////////////////////////////////////////////////////////
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

pub struct DBBlock;
#[rustfmt::skip]
impl DBBlock {
    pub fn new() -> Self {
        if cfg!(LOCAL) { eprintln!("{{"); unsafe { DBG_INDENT += 1; } }
        Self {}
    }
}
#[rustfmt::skip]
impl Drop for DBBlock {
    fn drop(&mut self) {
        if cfg!(LOCAL) { unsafe { DBG_INDENT -= 1; } eprintln!("}}"); }
    }
} //}}}
