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
fn dfs(tree: &Vec<Vec<usize>>, prev: usize, now: usize, dp: &mut Vec<usize>) {
    dp[now] = 1;
    for node in &tree[now] {
        if node != &prev {
            dfs(tree, now, *node, dp);
            dp[now] += dp[*node];
        }
    }
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,edge:[(Usize1,Usize1);n-1]
    }
    let mut tree = vec![Vec::new(); n];
    for (from, to) in &edge {
        tree[*from].push(*to);
        tree[*to].push(*from);
    }
    let dp = &mut vec![0usize; n];
    dfs(&tree, 0, 0, dp);
    let ans = edge
        .iter()
        .map(|(x, y)| dp[*x].min(dp[*y]) * (n - dp[*x].min(dp[*y])))
        .sum::<usize>();
    println!("{}", ans);
}
fn main() {
    solve();
}
