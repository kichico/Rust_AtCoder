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
        n:usize,k:usize,h:[usize;n]
    }
    let mut dp = vec![usize::MAX; n];
    dp[0] = 0;
    for i in 0..n - 1 {
        for j in i + 1..(i + k + 1).min(n) {
            //println!("{} {}", i, j);
            dp[j] = dp[j].min(dp[i] + h[j].abs_diff(h[i]));
        }
    }
    println!("{}", dp[n - 1]);
}
fn main() {
    solve();
}
