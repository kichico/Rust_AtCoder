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
#[allow(non_snake_case)]
#[derive(Clone)]
struct Edge {
    from: usize,
    to: usize,
    cost: i64,
}

#[derive(Clone)]
struct BellmanFord {
    edges: Vec<Edge>,
    nodes: usize,
}

impl BellmanFord {
    pub fn new(n: usize) -> Self {
        let edges: Vec<Edge> = Vec::new();
        let nodes = n;
        return BellmanFord { edges, nodes };
    }
    pub fn add_edge(&mut self, from: usize, to: usize, cost: i64) {
        let e = Edge { from, to, cost };
        self.edges.push(e);
    }
    pub fn calc_shortest_distance(&mut self, start: usize) -> Option<Vec<i64>> {
        const INF: i64 = 1e18 as i64;
        let mut dist = vec![-INF; self.nodes];
        dist[start] = 0i64;
        let mut cnt = 0;
        while cnt < self.nodes {
            let mut is_finished = true;
            for e in &self.edges {
                if dist[e.from] != INF && dist[e.from] + e.cost < dist[e.to] {
                    dist[e.to] = dist[e.from] + e.cost;
                    is_finished = false;
                }
            }
            if is_finished {
                break;
            }
            cnt += 1;
        }
        if cnt == self.nodes {
            return None;
        }
        return Some(dist);
    }
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,edge:[(Usize1,Usize1,i64);m]
    }
    let mut bf = BellmanFord::new(n);
    for (from, to, cost) in edge {
        bf.add_edge(from, to, -cost);
    }
    let Some(dist) = bf.calc_shortest_distance(0) else {
        println!("inf");
        return;
    };
}
fn main() {
    solve();
}
