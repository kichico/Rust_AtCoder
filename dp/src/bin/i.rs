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
        n:usize,p:[f64;n]
    }
    let mut dp = vec![vec![0f64; n + 1]; n];
    dp[0][0] = p[0];
    dp[0][1] = 1f64 - p[0];
    for i in 1..n {
        for j in 0..=n {
            let t = dp[i - 1][j] * p[i];
            dp[i][j] += t;
            if j > 0 {
                let t = dp[i - 1][j - 1] * (1f64 - p[i]);
                dp[i][j] += t;
            }
        }
    }
    let sum = dp[n - 1].iter().take(n / 2 + 1).sum::<f64>();
    println!("{}", sum);
}
fn main() {
    solve();
}
