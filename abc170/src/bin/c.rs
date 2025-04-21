#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::hash::Hash;
#[allow(unused_imports)]
use std::mem::swap;
use superslice::Ext;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        x:i64,n:usize,mut a:[i64;n]
    }
    a.sort();
    let mut ans = (x, 1e18 as i64);
    for val in 0..=101 {
        if a.contains(&val) {
            continue;
        }
        if abs(val - x) < ans.1 {
            ans = (val, abs(x - val));
        }
    }
    println!("{}", ans.0);
}
fn main() {
    solve();
}
