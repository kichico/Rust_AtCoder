#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use petgraph::unionfind::UnionFind;
#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        n: usize,
        m: usize,
        mut edges: [(i64,i64,i64);m],
    }
    let mut uf: UnionFind<usize> = UnionFind::new(n);
    let mut ans: i64 = 0;
    edges.sort_by_key(|k| k.2);
    for (mut x, mut y, cost) in edges {
        x -= 1;
        y -= 1;
        if uf.find(x as usize) != uf.find(y as usize) {
            uf.union(x as usize, y as usize);
        } else if cost <= 0 {
            uf.union(x as usize, y as usize);
        } else {
            ans += cost;
        }
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
