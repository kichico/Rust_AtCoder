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
        n:usize,radder:[(Usize1,Usize1);n]
    }
    let mut conv: HashMap<usize, usize> = HashMap::new();
    let mut st: BTreeSet<usize> = BTreeSet::new();
    st.insert(0);
    let mut rev = conv.clone();
    for (a, b) in &radder {
        st.insert(*a);
        st.insert(*b);
    }
    for (i, v) in enumerate(st) {
        conv.insert(v, i);
        rev.insert(i, v);
    }

    let mut uf = UnionFind::new(conv.len() + 1);
    for (a, b) in radder {
        uf.unite(*conv.get(&a).unwrap(), *conv.get(&b).unwrap());
    }
    //println!("{}", uf.parent.iter().join(" "));
    for i in (0..conv.len()).rev() {
        if uf.equiv(i, 0) {
            println!("{}", rev.get(&i).unwrap() + 1);
            return;
        }
    }
}
fn main() {
    solve();
}
