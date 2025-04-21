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
pub struct PotentializedUnionFind {
    parent: Vec<i64>,
    diff_potential: Vec<i64>,
}

impl PotentializedUnionFind {
    pub fn new(n: usize) -> Self {
        let parent = vec![-1i64; n];
        let diff_potential = vec![0i64; n];
        return PotentializedUnionFind {
            parent,
            diff_potential,
        };
    }
    pub fn find_root(&mut self, x: usize) -> usize {
        if self.parent[x] < 0 {
            return x;
        }
        let root = self.find_root(self.parent[x] as usize) as i64;
        self.diff_potential[x] += self.diff_potential[self.parent[x] as usize];
        self.parent[x] = root;
        return self.parent[x] as usize;
    }
    pub fn get_potential(&mut self, x: usize) -> i64 {
        let _r = self.find_root(x);
        return self.diff_potential[x];
    }
    pub fn get_diff(&mut self, x: usize, y: usize) -> i64 {
        return self.get_potential(y) - self.get_potential(x);
    }
    pub fn unite(&mut self, x: usize, y: usize, mut potential: i64) {
        potential += self.get_potential(x);
        potential -= self.get_potential(y);
        let mut xpar = self.find_root(x);
        let mut ypar = self.find_root(y);
        if xpar == ypar {
            return;
        }
        if self.parent[xpar] > self.parent[ypar] {
            std::mem::swap(&mut xpar, &mut ypar);
            potential = -potential;
        }
        let x = xpar;
        let y = ypar;
        self.parent[x] += self.parent[y];
        self.parent[y] = x as i64;
        self.diff_potential[y] = potential;
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
        n:usize,q:usize,query:[(Usize1,Usize1,i64);q]
    }
    let mut uf = PotentializedUnionFind::new(n);
    let mut ans: Vec<usize> = Vec::new();
    for (i, (x, y, w)) in enumerate(query) {
        if !uf.equiv(x, y) {
            ans.push(i + 1);
            uf.unite(x, y, w);
        } else if uf.get_diff(x, y) == w {
            ans.push(i + 1);
        }
    }
    if !ans.is_empty() {
        println!("{}", ans.iter().join(" "));
    }
}
fn main() {
    solve();
}
