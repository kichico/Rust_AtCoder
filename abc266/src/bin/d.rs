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
use std::f32::consts::E;
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
        n:usize,event:[(i64,i64,i64);n]
    }
    let mut dp = vec![vec![(0, 0); 2]; n + 1];
    let (time, hole, volume) = event[0];
    if time >= hole {
        dp[1][0] = (hole, volume);
    }
    println!("{} {}", dp[1][0].0, dp[1][0].1);
    for i in 2..=n {
        let (time, hole, volume) = event[i - 1];
        let prev_time = event[i - 2].0;
        for j in 0..2 {
            let (prev_pos, current_max) = dp[i - 1][j];
            if time - prev_time >= abs(hole - prev_pos) {
                if dp[i - 1][0].1 > dp[i - 1][1].0 {
                    dp[i][0] = dp[i - 1][0].clone();
                    dp[i][0] = (hole, current_max + volume);
                } else {
                    dp[i][0] = dp[i - 1][1].clone();
                    dp[i][0] = (hole, current_max + volume);
                }
            }
        }
    }
    println!("{}", dp[n][0].1.max(dp[n][1].1));
}

fn main() {
    solve();
}
