#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use proconio::{
    input,
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
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize
    }
    let mut bt: Vec<usize> = Vec::new();
    let mut ans: BTreeSet<usize> = BTreeSet::new();
    bt.push(1);
    for _i in 2..=18 {
        let val = bt[bt.len().clone() - 1];
        bt.push(val * 10 + 1);
    }
    for i in 0..bt.len() {
        for j in 0..bt.len() {
            for k in 0..bt.len() {
                ans.insert(bt[i] + bt[j] + bt[k]);
            }
        }
    }
    println!("{}", ans.iter().take(n).last().unwrap());
}
fn main() {
    solve();
}
