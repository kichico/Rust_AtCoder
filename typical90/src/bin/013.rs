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
        n:usize,m:usize,edges:[(Usize1,Usize1,i64);m]
    }
    let mut g = Dijkstra::new(n);
    for (u, v, cost) in edges {
        g.add_edge(u, v, cost);
        g.add_edge(v, u, cost);
    }
    let d0 = g.calc_shortest_distance(0);
    let dn = g.calc_shortest_distance(n - 1);
    for i in 0..n {
        println!("{}", d0[i] + dn[i]);
    }
}
fn main() {
    solve();
}
