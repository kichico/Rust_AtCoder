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
        n:usize,a:usize,card:[usize;n]
    }
    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; 51 * n]; n + 1]; n + 1];
    dp[0][0][card[0]] = 1;
    for i in 1..n {
        dp[i] = dp[i - 1].clone();
        dp[i][0][card[i]] += 1;
        for current_num in 1..=i {
            for sum in 0..=(51 * n - card[i]) {
                if dp[i - 1][current_num - 1][sum] != 0 {
                    dp[i][current_num][sum + card[i]] += dp[i - 1][current_num - 1][sum];
                }
            }
        }
    }
    /*
    println!(
        "{}",
        (0..=30)
            .into_iter()
            .map(|x| x.to_string().chars().last().unwrap())
            .join(" ")
    );
    for i in 0..n {
        println!(
            "{}",
            dp[n - 1][i]
                .iter()
                .enumerate()
                .filter(|x| x.0 <= 30)
                .map(|x| x.1)
                .join("|")
        );
    }*/
    let mut ans = 0;
    for i in 0..n {
        ans += dp[n - 1][i][a * (i + 1)];
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
