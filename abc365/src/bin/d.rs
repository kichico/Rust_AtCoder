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
        n:usize,aoki:Chars
    }
    let mut dp: Vec<HashMap<char, i64>> = vec![HashMap::new(); n];
    if aoki[0] == 'R' {
        *dp[0].entry('P').or_insert(0) += 1;
        dp[0].insert('R', 0);
        dp[0].insert('S', -1);
    } else if aoki[0] == 'P' {
        *dp[0].entry('S').or_insert(0) += 1;
        dp[0].insert('R', -1);
        dp[0].insert('P', 0);
    } else {
        *dp[0].entry('R').or_insert(0) += 1;
        dp[0].insert('P', -1);
        dp[0].insert('S', 0);
    }
    for i in 1..n {
        let a = aoki[i];
        if a == 'P' {
            let win = *dp[i - 1]
                .get(&'R')
                .unwrap()
                .max(dp[i - 1].get(&'P').unwrap());
            let draw = *dp[i - 1]
                .get(&'R')
                .unwrap()
                .max(dp[i - 1].get(&'S').unwrap());
            dp[i].insert('S', win + 1);
            dp[i].insert('P', draw);
            dp[i].insert('R', -1i64);
        } else if a == 'R' {
            let win = *dp[i - 1]
                .get(&'R')
                .unwrap()
                .max(dp[i - 1].get(&'S').unwrap());
            let draw = *dp[i - 1]
                .get(&'P')
                .unwrap()
                .max(dp[i - 1].get(&'S').unwrap());
            dp[i].insert('P', win + 1);
            dp[i].insert('R', draw);
            dp[i].insert('S', -1i64);
        } else {
            let win = *dp[i - 1]
                .get(&'S')
                .unwrap()
                .max(dp[i - 1].get(&'P').unwrap());
            let draw = *dp[i - 1]
                .get(&'R')
                .unwrap()
                .max(dp[i - 1].get(&'P').unwrap());
            dp[i].insert('R', win + 1);
            dp[i].insert('S', draw);
            dp[i].insert('P', -1i64);
        }
    }
    println!("{}", dp[n - 1].values().max().unwrap());
}
fn main() {
    solve();
}
