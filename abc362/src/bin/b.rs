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
        pa:(i64,i64),pb:(i64,i64),pc:(i64,i64)
    }
    let points = vec![pa, pb, pc];
    let mut edges = Vec::new();
    for v in (0..3).combinations(2) {
        let p1 = points[v[0]];
        let p2 = points[v[1]];
        let edge = (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2);
        edges.push(edge);
    }
    edges.sort();
    let ans = if edges[0] + edges[1] == edges[2] {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
fn main() {
    solve();
}
