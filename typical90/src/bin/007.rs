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
use superslice::*;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,mut a:[i64;n],q:usize,query:[i64;q]
    }
    a.sort();
    for v in query {
        let upper = a.upper_bound(&v);
        let lower = if upper > 0 { upper - 1 } else { 0 };
        //println!("{} {}", lower, upper);
        let mut dist = if lower < n {
            abs(v - a[lower])
        } else {
            abs(v - a.iter().next_back().unwrap())
        };
        if upper < n {
            dist = min(dist, abs(v - a[upper]));
        }
        println!("{}", dist);
    }
}
fn main() {
    solve();
}
