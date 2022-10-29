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
        n:usize,k:usize,h:[i64;n]
    }
    let mut dp = vec![std::i64::MAX; n];
    dp[0] = 0;
    for i in 0..n - 1 {
        for j in 1..k + 1 {
            let next = i + j;
            if next >= n {
                break;
            }
            dp[next] = min(dp[next], dp[i] + abs(h[next] - h[i]));
        }
    }
    println!("{}", dp[n - 1]);
}

fn main() {
    solve();
}
