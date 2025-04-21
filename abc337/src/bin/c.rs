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
        n:usize,ord:[i64;n]
    }
    let mut tp = TopologicalSort::new(n);
    for i in 0..n {
        let mut s = ord[i];
        if s == -1 {
            continue;
        } else {
            s -= 1;
        }
        tp.add_edge(i, s as usize);
    }
    tp.solve();
    let mut ans = tp.get_topological_order();
    ans.reverse();
    println!("{}", ans.iter().map(|x| x + 1).join(" "));
}
fn main() {
    solve();
}
