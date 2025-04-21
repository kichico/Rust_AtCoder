#[allow(unused_imports)]
use itertools::*;
use maplit::hashset;
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
        h:usize,w:usize,mut field:[Chars;h]
    }
    let mut dx = vec![1, 0, -1, 0];
    let mut dy = vec![0, 1, 0, -1];
    let mut g = Dijkstra::new(h * w);
    let mut start = 0;
    let mut goal = 0;
    for r in 0..h {
        for c in 0..w {
            if field[r][c] == '^' {
                let nx = c.clone();
                let mut ny = r.clone();
                while let Some(ny_) = ny.checked_sub(1) {
                    if field[ny_][nx] != '.' && field[ny_][nx] != 'x' {
                        break;
                    }
                    field[ny_][nx] = 'x';
                    ny = ny_;
                }
            }
            if field[r][c] == '>' {
                let mut nx = c.clone() + 1;
                let ny = r.clone();
                while nx < w && (field[ny][nx] == '.' || field[ny][nx] == 'x') {
                    field[ny][nx] = 'x';
                    nx += 1;
                }
            }
            if field[r][c] == '<' {
                let mut nx = c.clone();
                let ny = r.clone();
                while let Some(nx_) = nx.checked_sub(1) {
                    if field[ny][nx_] != '.' && field[ny][nx_] != 'x' {
                        break;
                    }
                    field[ny][nx_] = 'x';
                    nx = nx_;
                }
            }
            if field[r][c] == 'v' {
                let nx = c.clone();
                let mut ny = r.clone() + 1;
                while ny < h {
                    if field[ny][nx] != '.' && field[ny][nx] != 'x' {
                        break;
                    }
                    field[ny][nx] = 'x';
                    ny += 1;
                }
            }
            if field[r][c] == 'S' {
                start = r * w + c;
            }
            if field[r][c] == 'G' {
                goal = r * w + c;
            }
        }
    }
    for r in 0..h {
        for c in 0..w {
            for k in 0..4 {
                let ny = r.clone() as i64 + dy[k];
                let nx = c.clone() as i64 + dx[k];
                if ny < 0 || nx < 0 || nx >= w as i64 || ny >= h as i64 {
                    continue;
                }
                let banned = hashset! {'>','<','^','v','x','#'};
                if banned.contains(&field[r][c]) {
                    continue;
                }
                if field[ny as usize][nx as usize] == '.' {
                    g.add_edge(r * w + c, (ny as usize) * w + nx as usize, 1);
                    g.add_edge((ny as usize) * w + nx as usize, r * w + c, 1);
                }
            }
        }
    }
    let dist = g.calc_shortest_distance(start);
    let ans = if dist[goal] == 1e18 as i64 + 1 {
        -1
    } else {
        dist[goal]
    };
    println!("{}", ans);
}
fn main() {
    solve();
}
