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

#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize
    }
    if n <= 1 {
        println!("0");
        return;
    }
    let MOD = 1e9 as usize + 7;
    let mut dp = vec![vec![0; n]; 4];
    dp[0][0] = 8;
    dp[1][0] = 1;
    dp[2][0] = 1;
    for i in 1..n {
        dp[0][i] = dp[0][i - 1] * 8 % MOD; //何もなし
        dp[1][i] = (dp[1][i - 1] * 9 + dp[0][i - 1]) % MOD; //9だけ
        dp[2][i] = (dp[2][i - 1] * 9 + dp[0][i - 1]) % MOD;
        dp[3][i] = (dp[3][i - 1] * 10 + dp[2][i - 1] + dp[1][i - 1]) % MOD;
    }
    println!("{}", dp[3][n - 1] % MOD);
}
fn main() {
    solve();
}
