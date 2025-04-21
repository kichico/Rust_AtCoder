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
#[allow(unused_imports)]
use std::ops::Bound::{Excluded, Included, Unbounded};
#[allow(unused_imports)]
use superslice::*;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,a:[i64;n]
    }
    // dp[i番目][倒す or 逃がす][倒した回数]
    let mut dp = vec![vec![vec![0; 2]; 2]; n];
    dp[0][1][0] = a[0];
    dp[0][0][1] = 0;
    for i in 1..n {
        dp[i] = dp[i - 1].clone();
        for taosu in 0..2 {
            if taosu == 0 {
                for kill_num in 0..2 {
                    dp[i][taosu][kill_num] = dp[i][taosu][kill_num].max(dp[i - 1][taosu][kill_num]);
                }
            } else {
                dp[i][taosu][0] = dp[i][taosu][0].max(dp[i - 1][taosu][1] + a[i]);
                dp[i][taosu][1] = dp[i][taosu][1].max(dp[i - 1][taosu][0] + a[i] * 2);
            }
        }
    }
    let mut ans = 0;
    for taosu in 0..2 {
        for kill_num in 0..2 {
            ans = ans.max(dp[n - 1][taosu][kill_num]);
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
