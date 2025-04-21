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
struct Edge {
    to: usize,
    cost: i64,
}

#[derive(Clone)]
struct Dijkstra {
    graph: Vec<Vec<Edge>>,
    from_record: Vec<usize>,
    nodes: usize,
}
#[allow(dead_code)]
impl Dijkstra {
    pub fn new(n: usize) -> Self {
        let graph: Vec<Vec<Edge>> = vec![Vec::new(); n];
        let from_record: Vec<usize> = vec![n + 1; n];
        let nodes = n;
        return Dijkstra {
            graph,
            from_record,
            nodes,
        };
    }
    pub fn add_edge(&mut self, from: usize, to: usize, cost: i64) {
        assert!(from != to);
        self.graph[from].push(Edge { to, cost });
    }
    pub fn calc_shortest_distance(&mut self, start: usize) -> Vec<i64> {
        let mut dist = vec![1e18 as i64 + 1; self.nodes];
        //min-heap
        let mut heap: BinaryHeap<(std::cmp::Reverse<i64>, usize)> = BinaryHeap::new();
        heap.push((Reverse(0), start));
        dist[start] = 0;
        while let Some((Reverse(min_dist), v)) = heap.pop() {
            if dist[v] < min_dist {
                continue;
            }
            for e in &self.graph[v] {
                if dist[e.to] > dist[v] + e.cost {
                    dist[e.to] = dist[v] + e.cost;
                    heap.push((Reverse(dist[e.to]), e.to));
                    self.from_record[e.to] = v;
                }
            }
        }
        return dist;
    }
    pub fn get_shortest_path(&mut self, mut goal: usize) -> Vec<usize> {
        let mut path: Vec<usize> = Vec::new();
        while goal != self.nodes {
            path.push(goal);
            goal = self.from_record[goal];
        }
        return path;
    }
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,edge:[(Usize1,Usize1);m]
    }
    let mut g = Dijkstra::new(n);
    let mut neighbor: Vec<usize> = Vec::new();
    for (from, to) in edge {
        g.add_edge(from, to, 1);
        if to == 0 {
            neighbor.push(from);
        }
    }
    if neighbor.is_empty() {
        println!("-1");
        return;
    }
    let dist = g.calc_shortest_distance(0);
    let shortest = neighbor.iter().map(|x| dist[*x]).min().unwrap();
    if shortest == 1e18 as i64 + 1 {
        println!("-1");
    } else {
        println!("{}", shortest + 1);
    }
}
fn main() {
    solve();
}
