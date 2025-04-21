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
        n:usize,m:usize,s:Chars,c:[Usize1;n]
    }
    let mut rotater: Vec<VecDeque<usize>> = vec![VecDeque::new(); m];
    for i in 0..n {
        rotater[c[i]].push_back(i);
    }
    for i in 0..m {
        rotater[i].rotate_right(1);
    }
    let mut ans = vec!['0'; n];
    for i in 0..n {
        ans[i] = s[rotater[c[i]].pop_front().unwrap()];
    }
    println!("{}", ans.iter().join(""));
}
fn main() {
    solve();
}
