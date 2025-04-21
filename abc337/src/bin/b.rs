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
        mut s:Chars
    }
    s.reverse();
    while s.last() == Some(&'A') {
        s.pop();
    }
    while s.last() == Some(&'B') {
        s.pop();
    }
    while s.last() == Some(&'C') {
        s.pop();
    }
    let ans = if s.is_empty() { "Yes" } else { "No" };
    println!("{}", ans);
}
fn main() {
    solve();
}
