use csr::IndexType;
#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
use petgraph::*;
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
        n:usize,edge:[Usize1;n]
    }
    let mut edges = Vec::new();
    for i in 0..n {
        edges.push((i, edge[i]));
    }
    let mut g = Graph::<(), ()>::new();
    let nodes: Vec<_> = (0..n).map(|_| g.add_node(())).collect();
    g.extend_with_edges(edges.iter().map(|&x| (nodes[x.0], nodes[x.1])));
    let ans = petgraph::algo::kosaraju_scc(&g);
    for group in ans {
        println!("now");
        println!("{}", group.iter().map(|x| x.index()).join(" "));
    }
}
fn main() {
    solve();
}
