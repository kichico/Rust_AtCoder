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
use std::cmp::min;
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
        x:usize,y:usize,z:usize,s:Chars
    }
    let mut dp = vec![vec![1e18 as usize; 2]; s.len() + 1];
    dp[0][0] = 0;
    dp[0][1] = z;
    let n = s.len();
    for i in 1..=n {
        dp[i - 1][0] = dp[i - 1][0].min(dp[i - 1][1] + z);
        dp[i - 1][1] = dp[i - 1][1].min(dp[i - 1][0] + z);
        if s[i - 1] == 'A' {
            dp[i][0] = dp[i][0].min(dp[i - 1][0] + y);
            dp[i][1] = dp[i][1].min(dp[i - 1][1] + x);
        } else {
            dp[i][0] = dp[i][0].min(dp[i - 1][0] + x);
            dp[i][1] = dp[i][1].min(dp[i - 1][1] + y);
        }
    }
    println!("{}", dp[n][0].min(dp[n][1]));
}
fn main() {
    solve();
}
