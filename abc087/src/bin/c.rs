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
        n:usize,a:[[i64;n];2]
    }
    if n == 1 {
        println!("{}", a[0][0] + a[1][0]);
        return;
    }
    let mut dp = vec![vec![0; n]; 2];
    dp[0][0] = a[0][0];
    dp[1][0] = a[1][0] + a[0][0];
    for i in 1..n {
        for j in 0..2 {
            dp[j][i] += a[j][i] + dp[j][i - 1];
            if j == 1 {
                dp[j][i] = dp[j][i].max(dp[j - 1][i] + a[j][i]);
            }
        }
    }
    println!("{}", dp[1][n - 1]);
}
fn main() {
    solve();
}
