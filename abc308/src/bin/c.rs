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
#[allow(unused_imports)]
use std::mem::swap;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
/*
fn solve() {
    input! {
        n:usize,val:[(isize,isize);n]
    }
    let mut kakuritu = vec![(Rational::new(1, 1), 0usize); n];
    for (i, (a, b)) in enumerate(val) {
        kakuritu[i] = (Rational::new(a, a + b), i + 1);
    }
    let mut dict: BTreeMap<Rational, Vec<usize>> = BTreeMap::new();
    for (r, order) in kakuritu {
        let e = dict.entry(r).or_insert(Vec::new());
        e.push(order);
    }
    let mut ans: Vec<usize> = Vec::new();
    for (_i, mut v) in rev(dict) {
        v.sort();
        for c in v {
            ans.push(c);
        }
    }
    println!("{}", ans.iter().join(" "));
}
*/

//fn comp(first: (i64, i64), second: (i64, i64)) -> std::cmp::Ordering {}
fn solve() {
    input! {
        n:usize,mut val:[(i64,i64);n]
    }
    
}
fn main() {
    solve();
}
