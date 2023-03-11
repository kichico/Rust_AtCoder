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
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,edges:[(Usize1,Usize1);n-1]
    }
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (u, v) in edges {
        g[u].push(v);
        g[v].push(u);
    }
    let order = (0..n).into_iter().collect::<Vec<usize>>();
    let cnt = g.iter().map(|v| v.len()).collect::<Vec<usize>>();
    let mut order = cnt.into_iter().zip(order).collect::<Vec<(_, _)>>();
    order.sort();
    let mut set: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut ans: BTreeSet<usize> = BTreeSet::new();
    for t in order {
        set.insert(t);
    }
    while ans.len() < n / 2 {
        let node = set.iter().next().unwrap().clone();
        ans.insert(node.1 + 1);
        set.remove(&node);
        for p in &g[node.1] {
            set.remove(&(g[*p].len(), *p));
        }
    }
    println!("{}", ans.iter().join(" "));
}
fn main() {
    solve();
}
