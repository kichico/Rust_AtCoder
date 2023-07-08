#[allow(unused_imports)]
use itertools::*;
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
        n:usize,ladder:[(Usize1,Usize1);n]
    }
    let mut st: BTreeSet<usize> = BTreeSet::new();
    let mut uf = UnionFind::new(n * 2);
    for (from, to) in &ladder {
        st.insert(*from);
        st.insert(*to);
    }
    if !st.contains(&0) {
        println!("1");
        return;
    }
    let mut mp: HashMap<usize, usize> = HashMap::new();
    let mut rev = mp.clone();
    for (i, v) in enumerate(st) {
        mp.insert(v, i);
        rev.insert(i, v);
    }
    for (from, to) in &ladder {
        uf.unite(*mp.get(from).unwrap(), *mp.get(to).unwrap());
    }
    dbg!(&uf.parent);
    for i in (0..mp.len()).rev() {
        if uf.equiv(0, i) {
            println!("{}", rev.get(&(i)).unwrap() + 1);
            return;
        }
    }
}
fn main() {
    solve();
}
