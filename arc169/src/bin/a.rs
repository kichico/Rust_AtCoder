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
        n:usize,mut a:[i64;n],p:[Usize1;n-1]
    }
    let mut g = TopologicalSort::new(n);
    for i in 0..n - 1 {
        g.add_edge(i + 1, p[i]);
    }
    let flg = g.solve();
    if !flg {
        panic!();
    }
    let v = g.get_topological_order();
    for i in 0..n - 1 {
        a[v[i + 1]] += a[v[i]];
    }
    //println!("{:?}", v);
    let mut ans = "+";
    if a[v[n - 1]] - a[v[n - 2]] == 0 {
        if a[0] < 0 {
            ans = "-";
        } else if a[0] == 0 {
            ans = "0";
        }
    } else if a[v[n - 2]] < 0 {
        ans = "-";
    }
    //println!("{:?}", a);
    println!("{}", ans);
}
fn main() {
    solve();
}
