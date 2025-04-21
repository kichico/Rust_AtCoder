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
    to: usize,
    cost: usize,
}

#[derive(Clone)]
struct Dijkstra {
    graph: Vec<Vec<Edge>>,
    from_record: Vec<usize>,
    nodes: usize,
    depart: Vec<usize>,
}

impl Dijkstra {
    pub fn new(n: usize) -> Self {
        let graph: Vec<Vec<Edge>> = vec![Vec::new(); n];
        let from_record: Vec<usize> = vec![n + 1; n];
        let nodes = n;
        let depart = vec![0; n];
        return Dijkstra {
            graph,
            from_record,
            nodes,
            depart,
        };
    }
    pub fn add_edge(&mut self, from: usize, to: usize, cost: usize, time: usize) {
        assert!(from != to);
        self.graph[from].push(Edge { to, cost });
        self.depart[from] = time;
    }
    pub fn calc_shortest_distance(&mut self, start: usize) -> Vec<usize> {
        let mut dist = vec![1e18 as usize + 1; self.nodes];
        //min-heap
        let mut heap: BinaryHeap<(std::cmp::Reverse<usize>, usize)> = BinaryHeap::new();
        heap.push((Reverse(0), start));
        dist[start] = 0;
        while let Some((Reverse(min_dist), v)) = heap.pop() {
            if dist[v] < min_dist {
                continue;
            }
            for e in &self.graph[v] {
                println!(
                    "from:{} to:{}",
                    (self.depart[v] - dist[v] % self.depart[v]) % self.depart[v] + e.cost,
                    dist[e.to]
                );
                if dist[e.to]
                    > (self.depart[v] - dist[v] % self.depart[v]) % self.depart[v] + e.cost
                {
                    dist[e.to] =
                        (self.depart[v] - dist[v] % self.depart[v]) % self.depart[v] + e.cost;
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
        n:usize,m:usize,x:Usize1,y:Usize1,edge:[(Usize1,Usize1,usize,usize);m]
    }
    let mut g = Dijkstra::new(n);
    for (from, to, cost, time) in edge {
        g.add_edge(from, to, cost, time);
    }
    let dist = g.calc_shortest_distance(x);
    println!("{}", dist[y]);
}
fn main() {
    solve();
}
