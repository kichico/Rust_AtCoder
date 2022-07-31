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
#[fastout]
fn solve() {
    input! {
        n:usize,k:usize,x:[i64;n],gain:[(usize,i64);k]
    }
    let mut dp: Vec<Vec<i64>> = vec![vec![-1; n + 1]; n];
    let mut g = vec![0; n + 1];
    for i in 0..k {
        g[gain[i].0] = gain[i].1;
    }
    let gain = g;
    let mut ans = 0;
    dp[0][0] = 0;
    dp[0][1] = x[0] + gain[1];
    for i in 1..n {
        for j in 0..n + 1 {
            dp[i][0] = max(dp[i][0], dp[i - 1][j]);
        }
        for j in 1..i + 2 {
            dp[i][j] = max(dp[i][j], dp[i - 1][j - 1] + x[i] + gain[j]);
        }
    }
    for i in 0..n + 1 {
        ans = max(ans, dp[n - 1][i]);
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
