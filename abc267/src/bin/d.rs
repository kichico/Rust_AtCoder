#[allow(unused_imports)]
use itertools::join;
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
    let mut dp = vec![vec![(-1e18) as i64; m]; n + 1];
    for i in 0..n {
        dp[i][0] = a[i];
    }
    for j in 0..m - 1 {
        for i in j..n {
            for k in i + 1..n {
                let prev = dp[i][j];
                if prev + a[k] * (j + 2) as i64 > dp[k][j + 1] {
                    dp[k][j + 1] = prev + a[k] * (j + 2) as i64;
                }
            }
        }
    }
    let mut ans = dp[0][m - 1];
    for i in 0..n {
        ans = max(dp[i][m - 1], ans);
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
