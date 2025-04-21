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
#[allow(unused_imports)]
use superslice::*;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,x:usize,foods:[(Usize1,usize,usize);n]
    }
    let mut dp = vec![vec![vec![0; 3]; x + 1]; n];
    if foods[0].1 <= x {
        dp[0][foods[0].1][foods[0].0] += foods[0].2;
    }
    for i in 1..n {
        let (vitamin, am, cal) = foods[i];
        for c in 0..x {
            dp[i][c] = dp[i - 1][c].clone();
            if c + cal > x + 1 || cal > c {
                continue;
            }
            dp[i][c][vitamin] = dp[i][c][vitamin].max(dp[i - 1][c - cal][vitamin] + am);
        }
    }
    println!("{}", dp[n - 1][x].iter().min().unwrap());
    for i in 0..n {
        for j in 0..=x {
            println!(
                "{},{} => 1:{} 2:{} 3:{}",
                i, j, dp[i][j][0], dp[i][j][1], dp[i][j][2]
            );
        }
    }
}
fn main() {
    solve();
}
