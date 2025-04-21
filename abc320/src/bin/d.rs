#[allow(unused_imports)]
use itertools::*;
use multimap::MultiMap;
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
    diff_x: Vec<i64>,
    diff_y: Vec<i64>,
}

impl PotentializedUnionFind {
    pub fn new(n: usize) -> Self {
        let parent = vec![-1i64; n];
        let diff_x = vec![0i64; n];
        let diff_y = vec![0i64; n];
        return PotentializedUnionFind {
            parent,
            diff_x,
            diff_y,
        };
    }
    pub fn find_root(&mut self, x: usize) -> usize {
        if self.parent[x] < 0 {
            return x;
        }
        let root = self.find_root(self.parent[x] as usize) as i64;
        self.diff_x[x] += self.diff_x[self.parent[x] as usize];
        self.diff_y[x] += self.diff_y[self.parent[x] as usize];
        self.parent[x] = root;
        return self.parent[x] as usize;
    }
    pub fn get_potential(&mut self, x: usize) -> (i64, i64) {
        let _r = self.find_root(x);
        return (self.diff_x[x], self.diff_y[x]);
    }
    pub fn get_diff(&mut self, x: usize, y: usize) -> (i64, i64) {
        let x_val = self.get_potential(x);
        let y_val = self.get_potential(y);
        return (y_val.0 - x_val.0, y_val.1 - x_val.1);
    }
    pub fn unite(&mut self, x: usize, y: usize, mut potential_x: i64, mut potential_y: i64) {
        let potential_xy = self.get_diff(y, x);
        potential_x += potential_xy.0;
        potential_y += potential_xy.1;
        let mut xpar = self.find_root(x);
        let mut ypar = self.find_root(y);
        if xpar == ypar {
            return;
        }
        if self.parent[xpar] > self.parent[ypar] {
            std::mem::swap(&mut xpar, &mut ypar);
            potential_x = -potential_x;
            potential_y = -potential_y;
        }
        let x = xpar;
        let y = ypar;
        self.parent[x] += self.parent[y];
        self.parent[y] = x as i64;
        self.diff_x[y] = potential_x;
        self.diff_y[y] = potential_y;
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
        n:usize,m:usize,mut witness:[(Usize1,Usize1,i64,i64);m]
    }
    let mut pos = vec![(1e18 as i64, 1e18 as i64); n];
    let mut g = PotentializedUnionFind::new(n);
    for (x, y, potential_x, potential_y) in witness {
        g.unite(x, y, potential_x, potential_y);
    }
    for i in 0..n {
        let potential = g.get_diff(0, i);
        if !g.equiv(0, i) {
            println!("undecidable");
        } else {
            println!("{} {}", potential.0, potential.1);
        }
    }
}
fn main() {
    solve();
}
