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
use std::iter::FromIterator;
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
        n:usize,m:usize,e:usize,edge:[(Usize1,Usize1);e],q:usize,mut Query:[Usize1;q]
    }
    let mut uf: UnionFind = UnionFind::new(n + m + 1);
    Query.reverse();
    let que = Query.clone();
    let hs: HashSet<usize> = HashSet::from_iter(Query.into_iter());
    let electron = n + m;
    for i in 0..e {
        if hs.contains(&i) {
            continue;
        }
        let (u, v) = edge[i];
        uf.unite(u, v);
    }
    for i in n..n + m {
        uf.unite(i, electron);
    }
    let mut cnt = 0;
    for i in 0..n {
        if uf.equiv(i, electron) {
            cnt += 1;
        }
    }
    let mut ans = vec![cnt];
    let Query = que;
    for i in Query {
        let (x, y) = edge[i];
        let xton = if uf.equiv(x, electron) { true } else { false };
        let yton = if uf.equiv(y, electron) { true } else { false };
        if uf.equiv(x, y) || xton == yton {
            uf.unite(x, y);
            ans.push(cnt);
            continue;
        }
        if xton && !yton {
            cnt += uf.size(y);
            ans.push(cnt);
        } else if !xton && yton {
            cnt += uf.size(x);
            ans.push(cnt);
        }
        uf.unite(x, y);
    }
    ans.pop();
    ans.reverse();
    for i in ans {
        println!("{}", i);
    }
}

fn main() {
    solve();
}
