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
        n:usize,cards:[(i64,i64);n]
    }
    let mut dp = vec![vec![0; 2]; n];
    dp[0][0] = 1;
    dp[0][1] = 1;
    let MOD = 998244353;
    for i in 1..n {
        let (omote, ura) = cards[i];
        if omote != cards[i - 1].0 {
            dp[i][0] += dp[i - 1][0];
        }
        if omote != cards[i - 1].1 {
            dp[i][0] += dp[i - 1][1];
        }
        if ura != cards[i - 1].0 {
            dp[i][1] += dp[i - 1][0];
        }
        if ura != cards[i - 1].1 {
            dp[i][1] += dp[i - 1][1];
        }
        dp[i][0] %= MOD;
        dp[i][1] %= MOD;
    }
    println!("{}", (dp[n - 1][0] + dp[n - 1][1]) % MOD);
}
fn main() {
    solve();
}
