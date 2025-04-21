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

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,q:usize,edge:[(Usize1,Usize1,usize);m]
    }
    let mut query: Vec<(usize, usize, usize)> = vec![(0, usize::MAX, usize::MAX); q];
    let mut banned: HashSet<usize> = HashSet::new();
    for i in 0..q {
        input! {t:usize}
        if t == 1 {
            input! { e:Usize1 }
            banned.insert(e);
            query[i] = (t, e, 0);
        } else {
            input! {x:Usize1,y:Usize1}
            query[i] = (t, x, y);
        }
    }
    let mut dist = vec![vec![usize::MAX; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }
    for i in 0..m {
        if banned.contains(&i) {
            continue;
        }
        let (x, y, w) = edge[i];
        dist[x][y] = dist[x][y].min(w);
        dist[y][x] = dist[y][x].min(w);
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = dist[i][j].min(dist[i][k].saturating_add(dist[k][j]));
            }
        }
    }
    let mut ans = Vec::new();
    query.reverse();
    for (t, x, y) in query {
        if t == 1 {
            let (u, v, w) = edge[x];
            dist[u][v] = dist[u][v].min(w).min(dist[v][u]);
            dist[v][u] = dist[v][u].min(w).min(dist[u][v]);
            for i in 0..n {
                for j in 0..n {
                    dist[i][j] = dist[i][j]
                        .min(
                            dist[i][u]
                                .saturating_add(dist[u][v])
                                .saturating_add(dist[v][j]),
                        )
                        .min(
                            dist[i][v]
                                .saturating_add(dist[v][u])
                                .saturating_add(dist[u][j]),
                        );
                }
            }
            // println!("------------------");
        } else {
            let d = dist[x][y];
            if d == usize::MAX {
                ans.push(-1);
            } else {
                ans.push(d as i32);
            }
        }
    }
    ans.reverse();
    println!("{}", ans.iter().join("\n"));
}
fn main() {
    solve();
}
