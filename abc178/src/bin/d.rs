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
        s:usize
    }
    let mut dp = vec![0; 2001];
    for i in 3..=s {
        dp[i] = 1;
    }
    let MOD = 1e9 as usize + 7;
    for sum in 3..=s {
        for plus in 3..=s - 3 {
            if sum + plus > s {
                break;
            }
            dp[sum + plus] += dp[sum] % MOD;
        }
    }
    println!("{}", dp[s] % MOD);
}
fn main() {
    solve();
}
