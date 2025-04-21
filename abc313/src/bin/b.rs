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

fn dfs(g: &Vec<Vec<usize>>, now: usize, leaf: &mut HashSet<usize>, visited: &mut Vec<bool>) {
    if g[now].is_empty() {
        leaf.insert(now);
        return;
    }
    if visited[now] {
        return;
    }
    visited[now] = true;
    for u in &g[now] {
        if *u == now || visited[*u] {
            continue;
        }
        dfs(g, *u, leaf, visited);
    }
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,edge:[(Usize1,Usize1);m]
    }
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (u, v) in edge {
        g[v].push(u);
    }
    let leaf: &mut HashSet<usize> = &mut HashSet::new();
    let mut visited = vec![false; n];
    for i in 0..n {
        dfs(&g, i, leaf, &mut visited);
    }
    let ans = if leaf.len() == 1 {
        *leaf.iter().next().unwrap() as i32 + 1
    } else {
        -1
    };
    println!("{}", ans);
}
fn main() {
    solve();
}
