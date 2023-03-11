use itertools::enumerate;
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
        n:usize,name:[(String,String);n]
    }
    let mut checker: BTreeSet<String> = BTreeSet::new();
    let mut indexer: BTreeMap<String, usize> = BTreeMap::new();
    for (a, b) in &name {
        checker.insert(a.to_string());
        checker.insert(b.to_string());
    }
    for (i, s) in enumerate(checker) {
        indexer.insert(s, i);
    }
    let mut edges: Vec<(usize, usize)> = Vec::new();
    for (a, b) in &name {
        edges.push((*indexer.get(a).unwrap(), *indexer.get(b).unwrap()));
    }
    let mut topo = TopologicalSort::new(indexer.len());
    for (u, v) in edges {
        topo.add_edge(u, v);
    }
    if topo.solve() {
        println!("Yes");
    } else {
        println!("No");
    }
}
fn main() {
    solve();
}
