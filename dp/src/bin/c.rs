#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use petgraph::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
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
        n:usize,act:[(i64,i64,i64);n]
    }
    let mut dp = vec![vec![0; 3]; n];
    dp[0][0] = act[0].0;
    dp[0][1] = act[0].1;
    dp[0][2] = act[0].2;
    for i in 1..n {
        dp[i][0] = max(dp[i - 1][1], dp[i - 1][2]) + act[i].0;
        dp[i][1] = max(dp[i - 1][0], dp[i - 1][2]) + act[i].1;
        dp[i][2] = max(dp[i - 1][1], dp[i - 1][0]) + act[i].2;
    }
    println!("{}", max(dp[n - 1][0], max(dp[n - 1][1], dp[n - 1][2])));
}

fn main() {
    solve();
}
