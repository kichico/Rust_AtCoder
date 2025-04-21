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
        n:usize,mut a:[i64;n]
    }
    a.sort();
    let sum = a.iter().sum::<i64>() as usize;
    let p = (sum / n) as i64;
    let q = sum % n;
    let mut target: Vec<i64> = Vec::new();
    for _i in 0..n - q {
        target.push(p);
    }
    for _i in 0..q {
        target.push(p + 1);
    }
    let mut ans = 0;
    for (an, bn) in zip(a, target) {
        ans += abs(an - bn);
    }
    println!("{}", ans / 2);
}
fn main() {
    solve();
}
