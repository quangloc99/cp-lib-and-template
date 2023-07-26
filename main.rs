#[allow(unused_imports)]
use std::{
    cmp::*,
    collections::BTreeSet,
    io::{stderr, stdin, Write},
    mem::*,
    str::*,
};

fn main() {
    let mut scan = Scan::default();
    let stdout = std::io::stdout();
    let mut writer = std::io::BufWriter::new(stdout.lock());
}

#[derive(Default)]//{{{
struct Scan {
    buff: Vec<String>,
}
impl Scan {
    fn next<T: FromStr>(&mut self) -> T {
        if let Some(token) = self.buff.pop() {
            return token.parse().ok().unwrap();
        }
        let mut line: String = String::new();
        stdin().read_line(&mut line).ok();
        self.buff = line
            .split_ascii_whitespace()
            .map(String::from)
            .rev()
            .collect();
        return self.next();
    }
}

// Helpers
#[derive(Copy, Clone, Eq, PartialEq)]
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
}//}}}
