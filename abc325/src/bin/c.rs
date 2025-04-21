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
        h:usize,w:usize,field:[Chars;h]
    }
    let mut uf = UnionFind::new(h * w);
    let dx = vec![1, 1, 0, -1, -1, -1, 0, 1];
    let dy = vec![0, 1, 1, 1, 0, -1, -1, -1];
    for i in 0..h {
        for j in 0..w {
            for k in 0..4 {
                let ny = i as i64 + dy[k];
                let nx = j as i64 + dx[k];
                if ny < 0 || nx < 0 || ny >= h as i64 || nx >= w as i64 {
                    continue;
                }
                let ny = ny as usize;
                let nx = nx as usize;
                if field[i][j] == '#' && field[ny][nx] == '#' {
                    uf.unite(i * w + j, ny * w + nx);
                }
            }
        }
    }
    let mut ans: HashSet<usize> = HashSet::new();
    for i in 0..h * w {
        let xy = uf.find_root(i);
        let y = xy / w;
        let x = xy % w;
        if field[y][x] == '#' {
            ans.insert(xy);
        }
    }
    println!("{}", ans.len());
}
fn main() {
    solve();
}
