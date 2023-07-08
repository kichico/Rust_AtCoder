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
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,dishes:[(i64,i64);n]
    }
    let mut dp = vec![vec![0; 2]; n + 1];
    for i in 0..n {
        let (doku, val) = dishes[i];
        dp[i + 1] = dp[i].clone();
        if doku == 0 {
            dp[i + 1][0] = dp[i + 1][0].max(dp[i][0].max(dp[i][1]) + val);
        } else {
            dp[i + 1][1] = dp[i + 1][1].max(dp[i][0] + val);
        }
    }

    println!("{}", dp[n][0].max(dp[n][1]));
}
fn main() {
    solve();
}
