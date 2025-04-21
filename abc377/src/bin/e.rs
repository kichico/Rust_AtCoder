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
        n:usize,k:usize,mut p:[Usize1;n]
    }
    let mut mutate = BTreeSet::new();
    for i in 0..n {
        if i != p[i] {
            mutate.insert(i);
        }
    }
    let first = p.clone();
    let t = (k.max(n) as f64).ln() as usize + 1;
    println!("{}", t);
    for i in 0..100000000 {
        let mut tmp = p.clone();
        for m in 0..n {
            if mutate.contains(&m) {
                tmp[m] = p[p[m]];
            }
        }
        p = tmp;
        if first == p {
            println!("hit at {}!", i);
            break;
        }
    }
}
fn main() {
    solve();
}
