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
        n:usize,m:usize,a:[i64;n]
    }
    let LOW = -1e18 as i64;
    let mut dp = vec![vec![LOW as i64; m + 1]; n];
    dp[0][1] = a[0];
    for i in 1..n {
        dp[i][1] = a[i];
        for j in 1..(i + 2).min(m + 1) {
            dp[i][j] = dp[i][j].max(dp[i - 1][j]);
            dp[i][j] = dp[i][j].max(dp[i - 1][j - 1] + a[i] * j as i64);
        }
    }

    println!("{}", dp[n - 1][m]);
}
fn main() {
    solve();
}
