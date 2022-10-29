#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
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
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
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
    roots: HashSet<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let parent = vec![-1i64; n];
        let mut roots: HashSet<usize> = HashSet::new();
        for i in 0..n {
            roots.insert(i);
        }
        return UnionFind { parent, roots };
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
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,mut pos:[(i64,i64);n]
    }
    let mut dic: HashMap<(i64, i64), usize> = HashMap::new();
    let mut g: UnionFind = UnionFind::new(n);
    for i in 0..n {
        dic.insert(pos[i].clone(), i);
    }
    let r = vec![-1, -1, 0, 0, 1, 1];
    let c = vec![-1, 0, -1, 1, 0, 1];
    for i in 0..n {
        let (y, x) = pos[i];
        for j in 0..6 {
            let p = (y + r[j], x + c[j]);
            if dic.contains_key(&p) {
                let id = *dic.get(&p).unwrap();
                g.unite(i, id);
            }
        }
    }
    let mut ans: HashSet<usize> = HashSet::new();
    for i in 0..n {
        ans.insert(g.find_root(i));
    }
    println!("{}", ans.len());
}

fn main() {
    solve();
}
