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
fn dfs(g: &Vec<Vec<usize>>, visited: &mut Vec<bool>, path: &mut Vec<usize>, now: usize) {
    if visited[now] {
        return;
    }
    visited[now] = true;
    for to in &g[now] {
        dfs(g, visited, path, *to);
    }
    path.push(now + 1);
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize
    }
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        input! { c:usize,books:[Usize1;c] }
        for book in books {
            g[i].push(book);
        }
    }
    let mut path: Vec<usize> = Vec::new();
    let mut visited = vec![false; n];
    dfs(&g, &mut visited, &mut path, 0);
    path.pop();
    println!("{}", path.iter().join(" "));
}
fn main() {
    solve();
}
