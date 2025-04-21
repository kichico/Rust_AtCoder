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
        n:usize,k:usize,num:[(usize,usize);n]
    }
    let mut vals: BTreeMap<usize, usize> = BTreeMap::new();
    for (val, c) in num {
        let e = vals.entry(val).or_insert(0);
        *e += c;
    }
    let mut cnt = 0;
    for (val, c) in vals {
        cnt += c;
        if cnt >= k {
            println!("{}", val);
            return;
        }
    }
}
fn main() {
    solve();
}
