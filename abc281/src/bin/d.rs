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
        n:usize,k:usize,d:usize,a:[usize;n]
    }
    let mut dp = vec![vec![vec![-1i64; d]; k + 2]; n + 1];
    if n == 1 {
        if a[0] as usize % d == 0 {
            println!("{}", a[0]);
        } else {
            println!("-1");
        }
        return;
    }
    dp[0][0][0] = 0;
    for i in 0..n {
        for j in 0..k + 1 {
            for s in 0..d {
                if dp[i][j][s] == -1 {
                    continue;
                }
                dp[i + 1][j][s] = max(dp[i + 1][j][s], dp[i][j][s]);
                dp[i + 1][j + 1][(s + a[i]) % d] =
                    max(dp[i][j][s] + a[i] as i64, dp[i + 1][j + 1][(s + a[i]) % d]);
            }
        }
    }
    println!("{}", dp[n][k][0]);
}
fn main() {
    solve();
}
