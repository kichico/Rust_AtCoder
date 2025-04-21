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
            std::mem::swap(&mut xpar, &mut ypar);
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
        n:usize,q:usize,query:[(usize,usize,usize);q]
    }
    let mut g = UnionFind::new(n);
    let mut roots = vec![HashSet::new(); n];
    for i in 0..n {
        roots[i].insert(i);
    }
    for (x, mut u, mut v) in query {
        if x == 1 {
            u -= 1;
            v -= 1;
            let root_u = g.find_root(u);
            let root_v = g.find_root(v);
            g.unite(u, v);
            let new_root = g.find_root(u);
            if roots[root_u].len() >= roots[root_v].len() {
                let vec = std::mem::take(&mut roots[root_v]);
                for elem in vec {
                    roots[new_root].insert(elem);
                }
            } else {
                let vec = std::mem::take(&mut roots[root_v]);
                for elem in vec {
                    roots[new_root].insert(elem);
                }
            }
        } else {
            u -= 1;
            let root = g.find_root(u);
            if let Some(elem) = roots[root].iter().take(v).last() {
                println!("{}", elem);
            } else {
                println!("-1");
            }
        }
    }
}
fn main() {
    solve();
}
