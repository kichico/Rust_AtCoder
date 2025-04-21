use csr::IndexType;
use glidesort::sort;
#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
use petgraph::*;
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
#[derive(Debug, Clone)]
pub struct TopologicalSort {
    graph: Vec<Vec<usize>>,
    deg: Vec<usize>,
    res: Vec<usize>,
}
impl TopologicalSort {
    pub fn new(v: usize) -> Self {
        let graph: Vec<Vec<usize>> = vec![Vec::new(); v];
        let deg = vec![0usize; v];
        let res: Vec<usize> = Vec::new();
        TopologicalSort {
            graph: graph,
            deg: deg,
            res: res,
        }
    }
    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.graph[from].push(to);
        //入次数を増やす
        self.deg[to] += 1;
    }
    pub fn solve(&mut self) -> bool {
        let mut heap: BinaryHeap<std::cmp::Reverse<usize>> = BinaryHeap::new();
        let v = self.graph.len();
        for i in 0..v {
            if self.deg[i] == 0 {
                heap.push(Reverse(i));
            }
        }
        while let Some(Reverse(u)) = heap.pop() {
            self.res.push(u);
            for neighbor in &self.graph[u] {
                self.deg[*neighbor] -= 1;
                if self.deg[*neighbor] == 0 {
                    heap.push(Reverse(*neighbor));
                }
            }
        }
        for d in &self.deg {
            if *d != 0 {
                return false;
            }
        }
        return true;
    }
    pub fn get_topological_order(self) -> Vec<usize> {
        return self.res;
    }
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,edge:[Usize1;n]
    }
    let mut edges = Vec::new();
    for i in 0..n {
        if i == edge[i] {
            continue;
        }
        edges.push((i, edge[i]));
    }
    let mut g = Graph::<(), ()>::new();
    let nodes: Vec<_> = (0..n).map(|_| g.add_node(())).collect();
    g.extend_with_edges(edges.iter().map(|&x| (nodes[x.0], nodes[x.1])));
    let scc = petgraph::algo::kosaraju_scc(&g);
    let mut number = vec![0; n];
    for i in 0..scc.len() {
        let group = &scc[i];
        // println!(
        //     "group_{}:{}",
        //     i + 1,
        //     &group.iter().map(|x| x.index() + 1).join(" ")
        // );
        for id in group {
            number[id.index()] = i;
        }
    }
    let mut group_edge = vec![HashSet::new(); scc.len()];
    let mut top = TopologicalSort::new(scc.len());
    for i in 0..n {
        if number[i] == number[edge[i]] {
            continue;
        }
        //println!("make_edge:{} -> {}", number[i] + 1, number[edge[i]] + 1);
        group_edge[number[i]].insert(number[edge[i]]);
        top.add_edge(number[i], number[edge[i]]);
    }
    top.solve();
    let mut sorted = top.get_topological_order();
    //println!("top:{}", sorted.iter().map(|x| x + 1).join(" "));
    sorted.reverse();
    let mut ans = scc[sorted[0]].len() * scc[sorted[0]].len();
    let mut visited = vec![1; scc.len()];
    visited[sorted[0]] = scc[sorted[0]].len();
    for i in 1..sorted.len() {
        let group_id = sorted[i];
        visited[group_id] = scc[group_id].len();

        if visited[group_id] == 1 {
            ans += 1;
        }
        if let Some(prev_group_id) = group_edge[group_id].iter().next() {
            ans += visited[*prev_group_id] * visited[group_id];
            visited[group_id] += visited[*prev_group_id];
        }
    }
    //println!("v:{}", visited.iter().join(" "));
    println!("{}", ans);
}
fn main() {
    solve();
}
