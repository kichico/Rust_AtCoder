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
        a:char,b:char,c:char
    }
    let s = vec![a, b, c];
    let mut g = TopologicalSort::new(3);
    if s[0] == '>' {
        g.add_edge(0, 1);
    } else {
        g.add_edge(1, 0);
    }
    if s[1] == '>' {
        g.add_edge(0, 2);
    } else {
        g.add_edge(2, 0);
    }
    if s[2] == '>' {
        g.add_edge(1, 2);
    } else {
        g.add_edge(2, 1);
    }
    g.solve();
    let ans = g.get_topological_order();
    let v = vec!['A', 'B', 'C'];
    println!("{}", v[ans[1]]);
}
fn main() {
    solve();
}
