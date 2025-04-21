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
        n:usize,sushi:[usize;n]
    }
    let mut dp = vec![vec![vec![0f64; n + 2]; n + 2]; n + 2];
    let mut rest = vec![0; 4];
    for i in 0..n {
        rest[sushi[i]] += 1;
    }
    for k in 0..=n {
        for j in 0..=n {
            for i in 0..=n {
                if i + j + k == 0 {
                    continue;
                }
                let sum = (i + j + k) as f64;
                let mut val = n as f64 / sum;
                val += if i > 0 {
                    dp[i - 1][j][k] * (i as f64) / sum
                } else {
                    0.0
                };
                val += if j > 0 {
                    dp[i + 1][j - 1][k] * (j as f64) / sum
                } else {
                    0.0
                };
                val += if k > 0 {
                    dp[i][j + 1][k - 1] * (k as f64) / sum
                } else {
                    0.0
                };
                dp[i][j][k] = val;
            }
        }
    }
    println!("{}", dp[rest[1]][rest[2]][rest[3]]);
}
fn main() {
    solve();
}
