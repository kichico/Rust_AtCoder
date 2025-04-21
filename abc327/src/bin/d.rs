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
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,a:[Usize1;m],b:[Usize1;m]
    }
    let mut val = vec![-1; n];
    let mut edge: HashSet<(usize, usize)> = HashSet::new();
    for (ap, bp) in a.iter().zip(b) {
        edge.insert((*ap.min(&bp), *ap.max(&bp)));
    }
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (u, v) in edge {
        g[u].push(v);
        g[v].push(u);
    }
    let mut que: VecDeque<usize> = VecDeque::new();
    for i in 0..n {
        if val[i] == -1 {
            que.push_back(i);
            val[i] = 0;
        }
        while let Some(node) = que.pop_front() {
            for neighbor in &g[node] {
                if val[*neighbor] == -1 {
                    que.push_back(*neighbor);
                    val[*neighbor] = 1 - val[node];
                } else if val[*neighbor] == val[node] {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}
fn main() {
    solve();
}
