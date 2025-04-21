use ac_library::ModInt1000000007 as mint;
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
        h:usize,w:usize,field:[Chars;h]
    }
    let mut dp = vec![vec![mint::new(0); w]; h];
    dp[0][0] = mint::new(1);
    for i in 0..h {
        for j in 0..w {
            let t = dp[i][j].clone();
            if i < h - 1 && field[i + 1][j] != '#' {
                dp[i + 1][j] += t;
            }
            if j < w - 1 && field[i][j + 1] != '#' {
                dp[i][j + 1] += t;
            }
        }
    }
    println!("{}", dp[h - 1][w - 1]);
}
fn main() {
    solve();
}
