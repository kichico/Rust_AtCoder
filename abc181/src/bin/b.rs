#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

fn solve() {
    input! {
        n: usize,
        a: [(i64,i64);n],
    }
    let mut s: Vec<i64> = Vec::new();
    for i in 0..=1e6 as i64 + 1 {
        s.push(i);
        if s.len() > 1 {
            s[i as usize] += s[i as usize - 1];
        }
    }
    let mut current: i64 = 0;
    for (from, to) in a {
        current += s[to as usize] - s[from as usize - 1];
        //dbg!(s[to as usize] - s[from as usize - 1]);
    }
    println!("{}", current);
}

fn main() {
    solve();
}
