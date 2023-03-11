#[allow(unused_imports)]
use itertools::*;
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
use std::iter::FromIterator;
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
        n:usize,m:usize,
    }
    let mut shugo: Vec<HashSet<i64>> = Vec::new();
    for _ in 0..m {
        input! {
            c:usize,a:[i64;c]
        }
        let set: HashSet<i64> = HashSet::from_iter(a.into_iter());
        shugo.push(set);
    }
    let mut ans: HashSet<Vec<usize>> = HashSet::new();
    let mut keta = 1;
    let mut beki = 1;
    for i in 0..2.pow(m as u32) {
        let mut picked: Vec<usize> = Vec::new();
        if i == 2.pow(beki) {
            keta += 1;
            beki += 1;
        }
        for j in 0..keta as usize {
            if i & (1 << j) != 0 {
                picked.push(j);
            }
        }
        if picked.len() == 0 {
            continue;
        }
        let mut sets = shugo[picked[0]].clone();
        for j in 1..picked.len() {
            sets = sets
                .union(&shugo[picked[j]])
                .map(|x| *x)
                .into_iter()
                .collect();
        }
        if sets.len() == n {
            ans.insert(picked);
        }
    }
    println!("{}", ans.len());
}
fn main() {
    solve();
}
