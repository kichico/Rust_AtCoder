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
        n:usize,mut time:i64,a:[i64;n]
    }
    let total: i64 = a.iter().clone().sum();
    let circ = time / total;
    let mut time = time % total;
    let mut current = 0;
    for i in 0..n {
        if time - a[i] > 0 {
            time -= a[i];
            current += a[i];
        } else {
            println!("{} {}", i + 1, time);
            return;
        }
    }
}
fn main() {
    solve();
}
