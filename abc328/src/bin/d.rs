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
        s:Chars
    }
    let mut v: Vec<char> = Vec::new();
    for i in 0..s.len() {
        v.push(s[i]);
        while v.len() >= 3
            && v[v.len() - 3] == 'A'
            && v[v.len() - 2] == 'B'
            && v[v.len() - 1] == 'C'
        {
            for _j in 0..3 {
                v.pop();
            }
        }
    }
    if !v.is_empty() {
        println!("{}", v.iter().join(""));
    }
}
fn main() {
    solve();
}
