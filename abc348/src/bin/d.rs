use indexing::container;
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
use std::thread::Scope;

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
        h:usize,w:usize,field:[Chars;h], n:usize,med:[(Usize1,Usize1,i64);n]
    }
    let mut portion = HashMap::new();
    for i in 0..n {
        let (r, c, val) = med[i];
        if val < 0 {
            continue;
        }
        portion.insert((r, c), val);
    }

    let mut uf = UnionFind::new(h * w);
    let dx = vec![!0, 0, 1, 0];
    let dy = vec![0, !0, 0, 1];
    let mut start = (0, 0);
    let mut goal = (0, 0);
    for i in 0..h {
        for j in 0..w {
            let c = field[i][j];
            if c == '#' {
                continue;
            } else if c == 'S' {
                start = (i, j);
            } else if c == 'T' {
                goal = (i, j);
            }
            for k in 0..4 {
                let nx = j.wrapping_add(dx[k]);
                let ny = i.wrapping_add(dy[k]);
                if nx >= w || ny >= h {
                    continue;
                }
                if field[ny][nx] == '#' {
                    continue;
                }
                uf.unite(i * w + j, ny * w + nx);
            }
        }
    }
    if !uf.equiv(start.0 * w + start.1, goal.0 * w + goal.1) {
        println!("No");
        return;
    }
    let mut que: VecDeque<(usize, usize)> =
        VecDeque::from_iter(vec![(start.0, start.1)].into_iter());
    let mut scores = vec![vec![-1; w]; h];
    for ((r, c), v) in &portion {
        scores[*r][*c] = *v;
    }
    if !portion.contains_key(&(start.0, start.1)) {
        println!("No");
        return;
    }
    let mut visited = vec![vec![false; w]; h];
    visited[start.0][start.1] = true;
    while let Some((i, j)) = que.pop_front() {
        if scores[i][j] <= 0 {
            continue;
        }
        visited[i][j] = true;
        for k in 0..4 {
            let nx = j.wrapping_add(dx[k]);
            let ny = i.wrapping_add(dy[k]);
            if nx >= w || ny >= h {
                continue;
            }
            if field[ny][nx] == '#' {
                continue;
            }
            let score = scores[i][j] - 1;
            if !visited[ny][nx] {
                if let Some(_v) = portion.remove(&(ny, nx)) {
                    scores[ny][nx] = scores[ny][nx].max(score);
                    que.push_back((ny, nx));
                } else if scores[ny][nx] < score {
                    scores[ny][nx] = scores[ny][nx].max(score);
                    que.push_back((ny, nx));
                }
            } else if scores[ny][nx] < score {
                scores[ny][nx] = score;
                que.push_back((ny, nx));
            }
        }
    }
    // for i in 0..h {
    //     println!("{}", scores[i].iter().join(" "));
    // }
    let ans = if scores[goal.0][goal.1] >= 0 {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
fn main() {
    solve();
}
