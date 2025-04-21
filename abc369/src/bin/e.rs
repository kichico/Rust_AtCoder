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
#[derive(Clone)]
struct FloydWarshall {
    graph: Vec<Vec<usize>>,
}

impl FloydWarshall {
    pub fn new(n: usize) -> Self {
        let mut graph: Vec<Vec<usize>> = vec![vec![usize::MAX / 3; n]; n];
        for i in 0..n {
            graph[i][i] = 0;
        }
        return FloydWarshall { graph };
    }
    pub fn add_edge(&mut self, from: usize, to: usize, cost: usize) {
        self.graph[from][to] = self.graph[from][to].min(cost);
    }
    pub fn calculate(&mut self) -> Vec<Vec<usize>> {
        let n = self.graph.len();
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    self.graph[i][j] = self.graph[i][j].min(self.graph[i][k] + self.graph[k][j]);
                }
            }
        }
        return self.graph.clone();
    }
}

fn dfs(
    g: &Vec<Vec<usize>>,
    edges: &Vec<(usize, usize, usize)>,
    path: &mut Vec<usize>,
    k: &usize,
    mut current_cost: usize,
) -> usize {
    if path.len() == k * 2 {
        return current_cost + g[0][path[0]] + g[path[k * 2 - 1]][g.len() - 1];
    }
    let (from, to, cost) = edges[path.len() / 2];
    if !path.is_empty() {
        current_cost += g[path[path.len() - 1]][from];
    }
    path.push(from);
    path.push(to);
    let a = dfs(g, edges, path, k, current_cost + cost);
    path.pop();
    path.pop();
    if !path.is_empty() {
        current_cost -= g[path[path.len() - 1]][from];
        current_cost += g[*path.iter().last().unwrap()][to];
    }
    path.push(to);
    path.push(from);
    let b = dfs(g, edges, path, k, current_cost + cost);
    path.pop();
    path.pop();
    return a.min(b);
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,edge:[(Usize1,Usize1,usize);m],q:usize
    }
    let mut g = FloydWarshall::new(n);
    for (from, to, cost) in &edge {
        g.add_edge(*from, *to, *cost);
        g.add_edge(*to, *from, *cost);
    }
    let all_dist = g.calculate();
    let mut ans = Vec::new();
    for _ in 0..q {
        input! {k:usize,mut b:[Usize1;k]}
        b.sort();
        let mut dist = usize::MAX;
        for v in b.iter().permutations(k) {
            let mut ed = Vec::new();
            for i in 0..k {
                ed.push(edge[*v[i]]);
            }
            let current_dist = dfs(&all_dist, &ed, &mut Vec::new(), &k, 0);
            dist = dist.min(current_dist);
        }
        ans.push(dist);
    }
    println!("{}", ans.iter().join("\n"));
}
fn main() {
    solve();
}
