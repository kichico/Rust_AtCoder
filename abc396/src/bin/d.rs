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

fn dfs(
    g: &Vec<Vec<usize>>,
    visited: &mut BTreeSet<usize>,
    mut xor: usize,
    now: usize,
    mut ans: usize,
) -> (usize, usize) {
    if now == g.len() - 1 {
        ans = ans.min(xor);
        return (xor, ans);
    }
    for to in 0..g.len() {
        if to != now && g[now][to] != 2e18 as usize && !visited.contains(&to) {
            xor ^= g[now][to];
            visited.insert(to);
            (xor, ans) = dfs(g, visited, xor, to, ans);
            xor ^= g[now][to];
            visited.remove(&to);
        }
    }
    return (xor, ans);
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,edges:[(Usize1,Usize1,usize);m]
    }
    let mut g = vec![vec![2e18 as usize; n]; n];
    for (u, v, w) in edges {
        g[u][v] = w;
        g[v][u] = w;
    }
    let xor = 0;
    let mut ans = 2e18 as usize;
    let visited: &mut BTreeSet<usize> = &mut BTreeSet::new();
    visited.insert(0);
    (_, ans) = dfs(&g, visited, xor, 0, ans);
    println!("{}", ans);
}
fn main() {
    solve();
}
