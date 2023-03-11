#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
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
fn solve() {
    input! {
        n:usize,edge:[(Usize1,Usize1);n-1]
    }
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (u, v) in edge {
        graph[u].push(v);
        graph[v].push(u);
    }
    let mut visited = vec![false; n];
    let mut que: VecDeque<usize> = VecDeque::new();
    let mut dist = vec![0; n];
    que.push_back(0);
    while let Some(node) = que.pop_front() {
        visited[node] = true;
        for u in &graph[node] {
            if !visited[*u] {
                dist[*u] = dist[node] + 1;
                que.push_back(*u);
            }
        }
    }
    let order = (0..n).into_iter().collect::<Vec<usize>>();
    let mut dist_ord = dist.into_iter().zip(order).collect::<Vec<(usize, usize)>>();
    dist_ord.sort_by(|a, b| a.0.cmp(&b.0));
    let farest = dist_ord.iter().next_back().unwrap().1.clone();
    let mut dist = vec![0; n];
    que.push_back(farest);
    let mut visited = vec![false; n];
    while let Some(node) = que.pop_front() {
        visited[node] = true;
        for u in &graph[node] {
            if !visited[*u] {
                dist[*u] = dist[node] + 1;
                que.push_back(*u);
            }
        }
    }
    dist.sort();
    println!("{}", dist.iter().next_back().unwrap() + 1);
}
fn main() {
    solve();
}
