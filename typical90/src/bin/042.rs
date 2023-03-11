use itertools::iproduct;
#[allow(unused_imports)]
use itertools::Itertools;
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
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        K:usize
    }
    let mut dp = vec![vec![vec![0; 10]; 2]; K + 1];
    dp[0][0][0] = 1;
    for i in 0..K {
        for j in 0..10 {
            for k in 0..10 {
                dp[i + 1][1][(j + k) % 10] += dp[i][1][j];
            }
        }
    }
    for (i, j, k) in iproduct!(0..K, 0..10, 0..10) {}
}
fn main() {
    solve();
}
