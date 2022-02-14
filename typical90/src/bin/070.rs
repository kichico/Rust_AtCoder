#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        n: usize,
        pos: [(i64,i64);n],
    }
    let (mut x, mut y): (Vec<i64>, Vec<i64>) = (Vec::new(), Vec::new());
    for p in &pos {
        x.push(p.0);
        y.push(p.1);
    }
    x.sort();
    y.sort();
    let xmid = x[n / 2];
    let ymid = y[n / 2];
    let xans: i64 = x.iter().map(|&v| (xmid - v).abs()).sum();
    let yans: i64 = y.iter().map(|&v| (ymid - v).abs()).sum();
    println!("{}", xans + yans);
}

fn main() {
    solve();
}
