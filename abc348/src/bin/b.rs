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

#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,points:[(i64,i64);n]
    }
    for i in 0..n {
        let mut dic = BTreeMap::new();
        for j in 0..n {
            if i == j {
                continue;
            }
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            let dist = (x1 - x2).pow(2) + (y1 - y2).pow(2);
            let e = dic.entry(dist).or_insert(BTreeSet::new());
            e.insert(j + 1);
        }
        let (k, v) = dic.last_key_value().unzip();
        println!("{}", v.unwrap().first().unwrap());
    }
}
fn main() {
    solve();
}
