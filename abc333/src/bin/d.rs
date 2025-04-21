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
fn dfs(
    g: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    cost: &mut Vec<usize>,
    now: usize,
    par: usize,
) {
    visited[now] = true;
    if g[now].len() == 1 {
        cost[par] += cost[now] + 1;
        return;
    }
    for e in &g[now] {
        if visited[*e] {
            continue;
        }
        dfs(g, visited, cost, *e, now);
    }
    if par != 0 {
        cost[par] += cost[now] + 1;
    }
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,edge:[(Usize1,Usize1);n-1]
    }
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (u, v) in edge {
        g[u].push(v);
        g[v].push(u);
    }
    if g[0].len() == 1 {
        println!("1");
        return;
    }
    let visited = &mut vec![false; n];
    let cost = &mut vec![0; n];
    cost[0] = 1;
    dfs(&g, visited, cost, 0, 0);
    let mut costs: Vec<usize> = Vec::new();
    for node in &g[0] {
        costs.push(cost[*node] + 1);
    }
    costs.sort();
    costs.pop();
    println!("{}", costs.iter().sum::<usize>() + 1);
}
fn main() {
    solve();
}
