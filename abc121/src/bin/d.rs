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
fn to_Binary(mut x: usize) -> Vec<char> {
    let mut v: Vec<char> = Vec::new();
    if x == 0 {
        v.push('0');
    }
    while x > 0 {
        let c = std::char::from_digit((x % 2) as u32, 10).unwrap();
        v.push(c);
        x /= 2;
    }
    v.reverse();
    return v;
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        a:usize,b:usize
    }
    for i in a..=a + 10 {
        println!("{:0>4}", to_Binary(i).iter().join(""));
    }
}
fn main() {
    solve();
}
