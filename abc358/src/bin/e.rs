use ac_library::ModInt998244353;
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
        k:usize,c:[usize;26]
    }
    let mut dp = vec![vec![vec![ModInt998244353::new(0); 1010]; 26]; k];
    for i in 0..26 {
        if c[i] != 0 {
            dp[0][i][1] += 1;
        }
    }
    for i in 1..k {
        for alpha1 in 0..26 {
            for r in 1..1000 {
                for alpha2 in 0..26 {
                    if alpha1 == alpha2 {
                        let e = dp[i - 1][alpha1][r - 1];
                        dp[i][alpha1][r] += e;
                    } else {
                        let e = dp[i - 1][alpha2][r];
                        dp[i][alpha1][r] += e;
                    }
                }
            }
        }
    }
    let mut ans = ModInt998244353::new(0);
    'outer: for i in 0..k {
        let mut t = ModInt998244353::new(0);
        for alpha in 0..26 {
            let e = dp[i][alpha][c[alpha]];
            t += e;
        }
        ans += t;
    }
    println!("{}", dp[1][0].iter().take(5).join(" "));
    println!("{}", ans);
}
fn main() {
    solve();
}
