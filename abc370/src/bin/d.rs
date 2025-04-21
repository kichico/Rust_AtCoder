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
        h:usize,w:usize,q:usize,bomb:[(Usize1,Usize1);q]
    }
    let mut rower: Vec<BTreeSet<usize>> = vec![BTreeSet::from_iter((0..w).into_iter()); h];
    let mut columner: Vec<BTreeSet<usize>> = vec![BTreeSet::from_iter((0..h).into_iter()); w];
    for (r, c) in bomb {
        if rower[r].contains(&c) {
            rower[r].remove(&c);
        } else {
            let mut large = rower[r].range(c..).take(1).cloned();
            if let Some(x) = large.next() {
                rower[r].remove(&x);
                columner[x].remove(&r);
            }
            let mut small = rower[r].range(..=c).rev().take(1).cloned();
            if let Some(x) = small.next() {
                rower[r].remove(&x);
                columner[x].remove(&r);
            }
        }
        if columner[c].contains(&r) {
            columner[c].remove(&r);
        } else {
            let mut large = columner[c].range(r..).take(1).cloned();
            if let Some(x) = large.next() {
                columner[c].remove(&x);
                rower[x].remove(&c);
            }
            let mut small = columner[c].range(..=r).rev().take(1).cloned();
            if let Some(x) = small.next() {
                columner[c].remove(&x);
                rower[x].remove(&c);
            }
        }
    }
    let ans = rower.iter().map(|x| x.len()).sum::<usize>();
    println!("{}", ans);
}
fn main() {
    solve();
}
