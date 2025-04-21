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
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:i128
    }
    let mut val = 1i128;
    let st: BTreeSet<i128> = BTreeSet::from_iter((1..20).into_iter().map(|x| x.pow(x as u32)));
    for (i, val) in enumerate(st) {
        if val == n {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
    return;
}
fn main() {
    solve();
}
