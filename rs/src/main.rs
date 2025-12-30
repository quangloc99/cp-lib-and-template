#![allow(unexpected_cfgs, unused_imports, unused_macros)]
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
macro_rules! dbg { ($($arg:expr),*) => { eprintln!(concat!($("[", stringify!($arg), " = {:?}] "),*) $(, $arg)*) }}

fn main() {
    let mut scan = Scan::new();
    // let mut writer = stdout();  // for interactive
    let stdout = stdout().lock();
    #[allow(unused)]
    let mut writer = std::io::BufWriter::new(stdout);

    // let num_test = 1;
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
    buff: Vec<u8>,
    pos: usize,
}
#[allow(dead_code)]
#[allow(unused_variables)]
impl Scan {
    fn new() -> Self {
        return Self { stdin: std::io::stdin().lock(), buff: vec![], pos: 0 };
    }

    fn next_token(&mut self) -> Option<&[u8]> {
        while self.pos < self.buff.len() && self.buff[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
        if self.pos >= self.buff.len() {
            self.buff.clear();
            self.pos = 0;
            if self.stdin.read_until(b'\n', &mut self.buff).is_err() {
                return None;
            }
            return self.next_token();
        }
        let start = self.pos;
        while self.pos < self.buff.len() && !self.buff[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
        let token = &self.buff[start..self.pos];
        return Some(token);
    }

    fn next<T: FromStr>(&mut self) -> T {
        return self.next_opt().unwrap();
    }

    fn next_opt<T: FromStr>(&mut self) -> Option<T> {
        let token = self.next_token()?;
        let s = unsafe { std::str::from_utf8_unchecked(token) };
        return s.parse::<T>().ok();
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
