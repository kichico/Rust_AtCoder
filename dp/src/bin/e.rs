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
        N:usize,W:usize,mut goods:[(i64,i64);N]
    }
    let mut value = vec![0; N];
    let mut weight = vec![0; N];
    goods.sort_by(|x, y| x.0.cmp(&y.0));
    for i in 0..N {
        weight[i] = goods[i].0;
        value[i] = goods[i].1;
    }
    let mut V = 0;
    for i in 0..N {
        V += value[i];
    }
    V += 1;
    let mut dp = vec![vec![10000000000 as i64; V as usize + 1]; N + 1];
    dp[0][0] = 0;
    for i in 0..N {
        for val in 0..=V as i64 {
            let ww = val as usize;
            dp[i + 1][ww] = dp[i][ww];
            if val >= value[i] {
                dp[i + 1][ww] = min(dp[i + 1][ww], dp[i][(val - value[i]) as usize] + weight[i]);
            }
        }
    }
    let mut ans = 0;
    for i in 0..=V as usize {
        if dp[N][i] <= W as i64 {
            ans = i;
        }
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
