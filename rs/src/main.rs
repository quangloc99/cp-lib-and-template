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
macro_rules! wrln { ($writer: expr, $($arg:expr),*) => {std::writeln!($writer, $($arg,)*).unwrap()}}
macro_rules! wr { ($writer: expr, $($arg:expr),*) => {std::write!($writer, $($arg,)*).unwrap()}}

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

        wrln!(writer, "{}", a.iter().sum::<usize>());
    }
}

////////////////////////////////////////////////////////////////////////////////
//{{{
struct Scan(std::io::StdinLock<'static>, Vec<u8>, usize);

#[allow(dead_code)]
#[allow(unused_variables)]
impl Scan {
    fn new() -> Self {
        Self(std::io::stdin().lock(), Vec::with_capacity(32), 0)
    }

    fn next_token(&mut self) -> Option<&[u8]> {
        let not_ws = |c: &u8| !c.is_ascii_whitespace();
        let Some(skip) = self.1[self.2..].iter().position(not_ws) else {
            self.1.clear();
            self.0.read_until(b'\n', &mut self.1).ok()?;
            self.2 = 0;
            return self.next_token();
        };
        let start = self.2 + skip;
        self.2 += self.1[start..].iter().take_while(|&c| not_ws(c)).count();
        Some(&self.1[start..self.2])
    }

    fn next<T: FromStr>(&mut self) -> T {
        let token = self.next_token().unwrap();
        unsafe { std::str::from_utf8_unchecked(token).parse::<T>().ok() }.unwrap()
    }

    fn read_line(&mut self) -> Option<String> {
        let mut line = String::new();
        self.0.read_line(&mut line).map(|_| line).ok()
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
