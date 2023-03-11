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
fn dfs(
    g: &Vec<Vec<usize>>,
    colors: &mut Vec<i64>,
    visited: &mut Vec<bool>,
    now: usize,
    current_color: i64,
) -> bool {
    if colors[now] == -1 {
        colors[now] = current_color;
    }
    if visited[now] {
        return true;
    }
    visited[now] = true;
    for next in &g[now] {
        if colors[now] == colors[*next] && visited[*next] {
            return false;
        }
        dfs(g, colors, visited, *next, 1 - current_color);
    }
    return true;
}
#[derive(Debug, Clone)]
pub struct UnionFind {
    parent: Vec<i64>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let parent = vec![-1i64; n];
        return UnionFind { parent };
    }
    pub fn find_root(&mut self, x: usize) -> usize {
        if self.parent[x] < 0 {
            return x;
        }
        self.parent[x] = self.find_root(self.parent[x] as usize) as i64;
        return self.parent[x] as usize;
    }
    pub fn unite(&mut self, x: usize, y: usize) {
        let mut xpar = self.find_root(x);
        let mut ypar = self.find_root(y);
        if xpar == ypar {
            return;
        }
        if self.parent[xpar] > self.parent[y] {
            swap(&mut xpar, &mut ypar);
        }
        let x = xpar;
        let y = ypar;
        self.parent[x] += self.parent[y];
        self.parent[y] = x as i64;
    }
    pub fn size(&mut self, x: usize) -> i64 {
        let x = self.find_root(x);
        return -self.parent[x];
    }
    pub fn equiv(&mut self, x: usize, y: usize) -> bool {
        let x = self.find_root(x);
        let y = self.find_root(y);
        return x == y;
    }
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,edge:[(Usize1,Usize1);m]
    }
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut uf = UnionFind::new(n);
    for (u, v) in &edge {
        g[*u].push(*v);
        g[*v].push(*u);
        uf.unite(*u, *v);
    }
    let mut color = vec![-1i64; n];
    let mut visited = vec![false; n];
    for i in 0..n {
        if !dfs(&g, &mut color, &mut visited, i, 0) {
            println!("0");
            return;
        }
    }
    let ans = n * (n - 1) / 2;
    let mut dict: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..n {
        let e = dict.entry(uf.find_root(i)).or_insert(vec![0; 2]);
        e[color[i] as usize] += 1;
    }
    let mut cnt = 0;
    for (_r, v) in dict {
        if v[0] != 0 {
            cnt += v[0] * (v[0] - 1) / 2;
        }
        if v[1] != 0 {
            cnt += v[1] * (v[1] - 1) / 2;
        }
    }
    println!("{}", ans - cnt - m);
}
fn main() {
    solve();
}
