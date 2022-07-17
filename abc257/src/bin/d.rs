#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::Roots;
use petgraph::unionfind::UnionFind;
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
#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        n: usize,p:[(i64,i64,i64);n]
    }
    let mut point = vec![((0, 0, 0), 0); n];
    for i in 0..n {
        let inp = (p[i], i);
        point[i] = inp;
    }
    let (mut left, mut right) = (0i64, 4e9 as i64);
    while right - left > 1 {
        let s: i64 = left + (right - left) / 2;
        let mut flg = false;
        for i in 0..n {
            let mut graph: UnionFind<usize> = UnionFind::new(n);
            let mut que: VecDeque<usize> = VecDeque::new();
            que.push_back(i);
            while !que.is_empty() {
                let pos = *que.front().unwrap();
                let (x, y, p) = point[pos].0;
                for j in 0..n {
                    if pos == j || graph.equiv(pos, j) {
                        continue;
                    }
                    let (nx, ny, _) = point[j].0;
                    if p * s >= abs(x - nx) + abs(y - ny) {
                        graph.union(pos, j);
                        que.push_back(j);
                    }
                }
                que.pop_front();
            }
            let par: HashSet<usize> = graph.into_labeling().into_iter().collect();
            if par.len() == 1 {
                flg = true;
                break;
            }
        }
        if flg {
            right = s;
        } else {
            left = s;
        }
    }
    println!("{}", right);
}

fn main() {
    solve();
}
