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
        n:usize,a:[usize;n]
    }
    let mut cnt = BTreeMap::new();
    for i in 0..n {
        cnt.entry(a[i]).or_insert(Vec::new()).push(i);
    }
    let mut v = Vec::new();
    for val in cnt.values() {
        if val.len() == 1 {
            v.push(val[0]);
        }
    }
    let maxx = v
        .iter()
        .map(|idx| (a[*idx], *idx))
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .collect_vec();
    println!("{}", maxx.last().unwrap().1 + 1);
    let Some((_v,idx)) = maxx.last() else{ println!("-1"); return; };
    println!("{}", idx);
}
fn main() {
    solve();
}
