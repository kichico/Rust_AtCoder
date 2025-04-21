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
// 0==kuro 1==shiro
fn dfs(g: &Vec<Vec<usize>>, prev: usize, now: usize, dp: &mut Vec<Vec<usize>>) {
    let MOD = 1e9 as usize + 7;
    dp[now][0] = 1;
    dp[now][1] = 1;
    for i in &g[now] {
        let node = *i;
        if *i == prev {
            continue;
        }
        dfs(g, now, node, dp);
        dp[now][0] *= dp[node][1];
        dp[now][0] %= MOD;
        dp[now][1] *= dp[node][0] + dp[node][1];
        dp[now][1] %= MOD;
    }
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,edge:[(Usize1,Usize1);n-1]
    }
    let mut g = vec![Vec::new(); n];
    for (from, to) in &edge {
        g[*from].push(*to);
        g[*to].push(*from);
    }
    let dp = &mut vec![vec![0; 2]; n];
    dfs(&g, 0, 0, dp);
    println!("{}", (dp[0][0] + dp[0][1]) % (1e9 as usize + 7));
}
fn main() {
    solve();
}
