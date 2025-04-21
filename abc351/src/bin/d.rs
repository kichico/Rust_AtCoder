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
        h:usize,w:usize,field:[Chars;h]
    }
    let mut uf = ac_library::Dsu::new(h * w);
    let mut jisyaku = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            if field[i][j] == '#' {
                jisyaku.insert((i, j));
            }
        }
    }
    let dx = vec![0, 1, 0, !0];
    let dy = vec![1, 0, !0, 0];
    let mut not_connect = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            if jisyaku.contains(&(i, j)) {
                continue;
            }
            let mut pass = true;
            for k in 0..4 {
                let nx = j.wrapping_add(dx[k]);
                let ny = i.wrapping_add(dy[k]);
                if nx >= w || ny >= h {
                    continue;
                }
                if field[ny][nx] == '#' {
                    pass = false;
                    break;
                }
            }
            if !pass {
                not_connect.insert((i, j));
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            if jisyaku.contains(&(i, j)) || not_connect.contains(&(i, j)) {
                continue;
            }
            for k in 0..4 {
                let nx = j.wrapping_add(dx[k]);
                let ny = i.wrapping_add(dy[k]);
                if nx >= w || ny >= h {
                    continue;
                }
                if not_connect.contains(&(ny, nx)) {
                    continue;
                }
                uf.merge(i * w + j, ny * w + nx);
            }
        }
    }
    let mut ans = HashMap::new();
    let mut biggest_roots = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            if jisyaku.contains(&(i, j)) || not_connect.contains(&(i, j)) {
                continue;
            }
            let root = uf.leader(i * w + j);
            biggest_roots.insert(root);

            for k in 0..4 {
                let nx = j.wrapping_add(dx[k]);
                let ny = i.wrapping_add(dy[k]);
                if nx >= w || ny >= h {
                    continue;
                }
                if not_connect.contains(&(ny, nx)) {
                    ans.entry(uf.leader(i * w + j))
                        .or_insert(HashSet::new())
                        .insert((ny, nx));
                }
            }
        }
    }
    let mut sol = 1;
    if biggest_roots.is_empty() {
        println!("1");
        return;
    }
    // for a in &ans {
    //     print!("root:({},{}) other", a.0 / w, a.0 % w);
    //     for (i, j) in a.1 {
    //         print!(" ({},{})", i, j);
    //     }
    //     println!();
    // }
    for root in &biggest_roots {
        if let Some(other) = ans.get(&root) {
            sol = sol.max(other.len() + uf.size(*root));
        } else {
            sol = sol.max(uf.size(*root));
        }
    }
    println!("{}", sol);
}
fn main() {
    solve();
}
