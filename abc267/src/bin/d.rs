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
        n:usize,m:usize,a:[i64;n]
    }
    let mut dp = vec![vec![-1e18 as i64; m]; n];
    dp[0][0] = a[0];
    for i in 1..n {
        for j in 0..m {
            if j == 0 {
                dp[i][j] = a[i];
            } else {
                dp[i][j] = max(dp[i - 1][j], dp[i - 1][j - 1] + (j as i64 + 1) * a[i]);
            }
        }
    }
    for i in 0..n {
        println!("{}", dp[i].iter().join(" "));
    }
    let mut ans = -1e18 as i64;
    for i in 0..m {
        ans = max(ans, dp[n - 1][i]);
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
