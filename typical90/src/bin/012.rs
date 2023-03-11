#[allow(unused_imports)]
use itertools::Itertools;
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
#[allow(dead_code)]
fn clamp(val: usize, diff: i64, min: usize, max: usize) -> usize {
    let val = val as i64;
    if val + diff < min as i64 {
        return min;
    } else if val + diff > max as i64 {
        return max;
    } else {
        return (val + diff) as usize;
    }
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
        h:usize,w:usize,q:usize,
    }
    let mut uf = UnionFind::new(2 * h * w);
    let mut ans: Vec<String> = Vec::new();
    let mut painted = vec![vec![false; w]; h];
    let dx = vec![0, 1, 0, -1];
    let dy = vec![1, 0, -1, 0];
    for _ in 0..q {
        input! { c:i64 }
        if c == 1 {
            input! { y:Usize1,x:Usize1 }
            painted[y][x] = true;
            for i in 0..4 {
                let nx = clamp(x, dx[i], 0, w - 1);
                let ny = clamp(y, dy[i], 0, h - 1);
                if painted[ny][nx] {
                    uf.unite(y * w + x, ny * w + nx);
                }
            }
        } else {
            input! { inp:(Usize1,Usize1,Usize1,Usize1) }
            let (y1, x1, y2, x2) = inp;
            let s = if painted[y1][x1] && painted[y2][x2] && uf.equiv(y1 * w + x1, y2 * w + x2) {
                "Yes".to_string()
            } else {
                "No".to_string()
            };
            ans.push(s);
        }
    }
    println!("{}", ans.iter().join("\n"));
}

fn main() {
    solve();
}
