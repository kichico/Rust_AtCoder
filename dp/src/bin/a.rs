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
        n:usize,h:[usize;n]
    }
    let mut dp = vec![usize::MAX; n];
    dp[0] = 0;
    for i in 0..n - 1 {
        dp[i + 1] = dp[i + 1].min(dp[i] + h[i + 1].abs_diff(h[i]));
        if i != n - 2 {
            dp[i + 2] = dp[i + 2].min(dp[i] + h[i + 2].abs_diff(h[i]));
        }
    }
    println!("{}", dp[n - 1]);
}
fn main() {
    solve();
}
