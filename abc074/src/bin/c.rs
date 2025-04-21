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
        var:(usize,usize,usize,usize,usize,usize)
    }
    let (a, b, c, d, yokaido, limit) = var;
    let mut dp = vec![vec![false; limit + 1]; limit + 1];
    for i in 0..=30 {
        for j in 0..=30 {
            if i * 100 * a + j * 100 * b <= limit {
                dp[i * 100 * a + j * 100 * b][0] = true;
            }
        }
    }
    for water in 0..=limit {
        if !dp[water][0] {
            continue;
        }
        for sugar in 0..=limit {
            if water + sugar > limit {
                break;
            }
            if sugar >= c && dp[water][sugar - c] && sugar <= water / 100 * yokaido {
                dp[water][sugar] = true;
            }
            if sugar >= d && dp[water][sugar - d] && sugar <= water * yokaido / 100 {
                dp[water][sugar] = true;
            }
        }
    }
    let mut ans = (100 * a, 0);
    for water in 0..=limit {
        for sugar in 0..=limit {
            if !dp[water][sugar] {
                continue;
            }
            if sugar * (ans.0 + ans.1) > ans.1 * (water + sugar) {
                ans = (water, sugar);
            }
        }
    }
    /*
    for i in 0..=limit {
        for j in 0..=limit {
            let p = if dp[i][j] { "True" } else { "False" };
            print!("{} ", p);
        }
        println!();
    }
    */
    println!("{} {}", ans.0 + ans.1, ans.1);
}
fn main() {
    solve();
}
