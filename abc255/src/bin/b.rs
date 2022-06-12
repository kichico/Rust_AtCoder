#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::Roots;
#[allow(unused_imports)]
use petgraph::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

fn dist(a: (i64, i64), b: (i64, i64)) -> f64 {
    let ret = (b.0 - a.0).pow(2) + (b.1 - a.1).pow(2);
    let ret = ret as f64;
    return ret.sqrt();
}
#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        n: usize,k:usize,a:[i64;k],mut pos:[(i64,i64);n]
    }
    let mut notHaving: BTreeSet<i64> = BTreeSet::new();
    for i in 1..n + 1 {
        notHaving.insert(i as i64);
    }
    for i in &a {
        notHaving.remove(i);
    }
    let mut sorter: Vec<(i64, i64)> = Vec::new();
    for i in 0..n {
        if !notHaving.contains(&(i as i64 + 1)) {
            sorter.push(pos[i].clone());
        }
    }
    let mut ans = 0 as f64;
    for i in &notHaving {
        let base = pos[*i as usize - 1];
        sorter.sort_by(|&a, &b| dist(base, a).partial_cmp(&dist(base, b)).unwrap());
        ans = ans.max(dist(base, sorter[0]));
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
