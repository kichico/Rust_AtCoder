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
pub struct RollbackUnionFind {
    parent: Vec<i64>,
    history: Vec<(i64, i64)>,
    inner_snap: usize,
    roots: BTreeSet<usize>,
}

impl RollbackUnionFind {
    pub fn new(n: usize) -> Self {
        let parent = vec![-1i64; n];
        let history: Vec<(i64, i64)> = Vec::new();
        let inner_snap = 0;
        let roots: BTreeSet<usize> = BTreeSet::from_iter((0..n).into_iter());
        return RollbackUnionFind {
            parent,
            history,
            inner_snap,
            roots,
        };
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
        self.history.push((xpar as i64, self.parent[xpar]));
        self.history.push((ypar as i64, self.parent[ypar]));
        if self.parent[xpar] > self.parent[y] {
            std::mem::swap(&mut xpar, &mut ypar);
        }
        let x = xpar;
        let y = ypar;
        self.parent[x] += self.parent[y];
        self.parent[y] = x as i64;
        self.roots.remove(&y);
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
    pub fn undo(&mut self) {
        if let Some((x, data)) = self.history.pop() {
            self.parent[x as usize] = data;
            self.roots.insert(data as usize);
        }
        if let Some((x, data)) = self.history.pop() {
            self.parent[x as usize] = data;
            self.roots.insert(data as usize);
        }
    }
    pub fn snapshot(&mut self) {
        self.inner_snap = self.history.len() >> 1;
    }
    pub fn recorded_state(&self) -> usize {
        return self.history.len() >> 1;
    }
    pub fn roll_back(&mut self, state: i64) {
        let mut target = if state == -1 {
            self.inner_snap
        } else {
            state as usize
        };
        target <<= 1;
        assert!(target <= self.history.len());
        while target < self.history.len() {
            self.undo();
        }
    }
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,edges:[(Usize1,Usize1);m]
    }
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut ans = vec![-1i64; n];
    for (u, v) in &edges {
        g[*u].push(*v);
        g[*v].push(*u);
    }
    let mut remove_list: BTreeSet<usize> = BTreeSet::new();
    let mut uf: RollbackUnionFind = RollbackUnionFind::new(n);
    for i in 0..n {
        remove_list.remove(&i);
        uf.snapshot();
        for e in &g[i] {
            if uf.equiv(*e, 0) {
                uf.unite(0, i);
                continue;
            }
            if *e > i {
                remove_list.insert(*e);
            } else {
                uf.unite(*e, i);
            }
        }
        if !uf.equiv(i, 0) || uf.size(0) != i as i64 + 1 {
            ans[i] = -1;
        } else {
            ans[i] = remove_list.len() as i64;
            //dbg!(&remove_list);
        }
        for e in &g[i] {
            if *e > i {
                continue;
            }
            uf.unite(*e, i);
        }
    }
    println!("{}", ans.iter().join("\n"));
}
fn main() {
    solve();
}
