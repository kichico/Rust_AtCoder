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
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,mut bugs:[(usize,usize,i64);n]
    }
    bugs.reverse();
    let mut dp = vec![vec![-1; 5]; 1e5 as usize + 2];
    dp[0][0] = 0;
    for i in 0..=1e5 as usize {
        for x in 0..5 {
            if x == 0 {
                dp[i + 1][x] = dp[i + 1][x].max(dp[i][x]);
                dp[i + 1][x + 1] = dp[i + 1][x + 1].max(dp[i][x]);
            } else if x == 4 {
                dp[i + 1][x] = dp[i + 1][x].max(dp[i][x]);
                dp[i + 1][x - 1] = dp[i + 1][x - 1].max(dp[i][x]);
            } else {
                dp[i + 1][x] = dp[i + 1][x].max(dp[i][x]);
                dp[i + 1][x - 1] = dp[i + 1][x - 1].max(dp[i][x]);
                dp[i + 1][x + 1] = dp[i + 1][x + 1].max(dp[i][x]);
            }
        }
        if !bugs.is_empty() && bugs[bugs.len() - 1].0 == i {
            if dp[i][bugs[bugs.len() - 1].1] != -1 {
                dp[i + 1][bugs[bugs.len() - 1].1] += bugs[bugs.len() - 1].2;
            }
            bugs.pop();
        }
    }
    let mut ans = 0;
    for i in 0..5 {
        ans = ans.max(dp[1e5 as usize + 1][i]);
    }

    println!("{}", ans);
}
fn main() {
    solve();
}
