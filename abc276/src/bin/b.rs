#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
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
#[allow(unused_imports)]
use std::mem::swap;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,edge:[(Usize1,Usize1);m]
    }
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (u, v) in edge {
        g[u].push(v + 1);
        g[v].push(u + 1);
    }
    for i in 0..n {
        if g[i].is_empty() {
            println!("0");
            continue;
        }
        g[i].sort();
        println!("{} {}", g[i].len(), g[i].iter().join(" "));
    }
}
fn main() {
    solve();
}
