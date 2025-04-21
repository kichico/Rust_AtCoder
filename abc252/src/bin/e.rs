use itertools::enumerate;
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::Roots;
#[allow(unused_imports)]
use petgraph::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
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
    edge_record: HashMap<(usize, usize), usize>,
    picked_edge: HashSet<usize>,
}

impl Dijkstra {
    pub fn new(n: usize, edge_record: HashMap<(usize, usize), usize>) -> Self {
        let graph: Vec<Vec<Edge>> = vec![Vec::new(); n];
        let from_record: Vec<usize> = vec![n + 1; n];
        let nodes = n;
        let picked_edge: HashSet<usize> = HashSet::new();
        return Dijkstra {
            graph,
            from_record,
            nodes,
            edge_record,
            picked_edge,
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
                    if let Some(prev_edge_num) = self.edge_record.get(&(
                        e.to.min(self.from_record[e.to]),
                        e.to.max(self.from_record[e.to]),
                    )) {
                        self.picked_edge.remove(prev_edge_num);
                    }
                    let edge_num = *self.edge_record.get(&(e.to.min(v), e.to.max(v))).unwrap();
                    self.picked_edge.insert(edge_num);
                    heap.push((Reverse(dist[e.to]), e.to));
                    self.from_record[e.to] = v;
                }
            }
        }
        return dist;
    }
    pub fn get_shortest_path(&mut self, mut goal: usize) -> Vec<usize> {
        let mut path: Vec<usize> = Vec::new();
        while goal != self.nodes + 1 {
            path.push(goal);
            goal = self.from_record[goal];
        }
        return path;
    }
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n: usize,m:usize, edges:[(Usize1,Usize1,i64);m]
    }
    let mut record_edge: HashMap<(usize, usize), usize> = HashMap::new();
    let mut ans: HashSet<usize> = HashSet::new();
    for (i, (u, v, _)) in enumerate(&edges) {
        record_edge.insert((*u.min(v), *u.max(v)), i + 1);
    }
    let edge_record = record_edge.clone();
    let mut g = Dijkstra::new(n, edge_record);
    for (i, (u, v, cost)) in enumerate(edges) {
        g.add_edge(u, v, cost);
        g.add_edge(v, u, cost);
    }
    let _dist = g.calc_shortest_distance(0);
    let ans = g.picked_edge;
    println!("{}", ans.iter().join(" "));
}

fn main() {
    solve();
}
