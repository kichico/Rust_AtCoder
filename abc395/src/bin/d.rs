#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
use proconio::input_interactive;
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
use std::mem::take;
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
        if self.parent[xpar] > self.parent[ypar] {
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
        n:usize,q:usize
    }
    let mut uf = UnionFind::new(n);
    let mut room: BTreeMap<usize, usize> = BTreeMap::new();
    for i in 0..n {
        room.insert(i, i);
    }
    for query in 0..q {
        input! {o:usize}
        if o == 1 {
            input! {a:Usize1,b:Usize1}
            uf.unite(a, b);
            let e = room.get_mut(&uf.find_root(a)).unwrap();
            *e = b;
        } else if o == 2 {
            input! {a:Usize1,b:Usize1}
            if uf.equiv(a, b) {
                continue;
            }
            let e1 = take(room.get_mut(&uf.find_root(a)).unwrap());
            let e2 = take(room.get_mut(&uf.find_root(b)).unwrap());
            room.insert(uf.find_root(a), e2);
            room.insert(uf.find_root(b), e1);
        } else {
            input! {a:Usize1}
            println!("{}", room[&uf.find_root(a)] + 1);
        }
    }
}
fn main() {
    solve();
}
