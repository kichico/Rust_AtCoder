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
        n:i64,m:i64,k:i64
    }
    let (mut left, mut right): (i64, i64) = (0, 3e18 as i64);
    let lm = lcm(n, m);
    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if mid / n + mid / m - 2 * (mid / lm) >= k {
            right = mid;
        } else {
            left = mid;
        }
    }
    println!("{}", right);
}
fn main() {
    solve();
}
