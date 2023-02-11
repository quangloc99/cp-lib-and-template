#[allow(unused_imports)]
use std::{
    io::{stdin, stderr},
    std::*,
    mem::*,
    cmp::*,
    collections::BTreeSet
};

fn main() {
    let mut scan = Scan::default();
    
}


#[derive(Default)]
struct Scan { buff: Vec<String> }
impl Scan {
    fn next<T: FromStr>(&mut self) -> T {
        if let Some(token) = self.buff.pop() {
            return token.parse().ok().unwrap();
        }
        let mut line: String = String::new();
        stdin().read_line(&mut line).ok();
        self.buff = line.split_ascii_whitespace().map(String::from).rev().collect();
        return self.next();
    }
}

