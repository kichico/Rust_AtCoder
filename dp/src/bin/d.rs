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
        n:usize,W:usize,items:[(usize,usize);n]
    }
    let mut dp = vec![vec![0; W + 1 as usize]; n];
    for w in items[0].0..=W {
        dp[0][w] = items[0].1;
    }
    for i in 1..n {
        let mut ndp = dp[i - 1].clone();
        for w in 0..=W {
            if w + items[i].0 > W {
                break;
            }
            ndp[w + items[i].0] = ndp[w + items[i].0].max(dp[i - 1][w] + items[i].1);
        }
        dp[i] = ndp;
    }
    // println!("W:{}", (0..=W).join("|"));
    // for i in 0..n {
    //     println!("{}:{}", i, dp[i].iter().join("|"));
    // }
    println!("{}", dp[n - 1].iter().max().unwrap());
}
fn main() {
    solve();
}
