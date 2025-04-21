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
        n:Chars,k:usize
    }
    let mut num = vec![0u32, 0u32, 0u32]
        .into_iter()
        .chain(n.iter().map(|c| *c as u32 - 48))
        .collect::<Vec<_>>();
    if k > n.len() {
        println!("0");
        return;
    }
    num.reverse();
    for i in 0..k {
        if num[i] >= 5 {
            num[i + 1] += 1;
        }
        num[i] = 0;
    }
    for i in k..n.len() {
        if num[i] >= 10 {
            num[i + 1] += 1;
        }
        num[i] %= 10;
    }
    while num.iter().next_back() == Some(&0u32) {
        num.pop();
    }
    num.reverse();
    if num.is_empty() {
        println!("0");
        return;
    }
    println!("{}", num.iter().join(""));
}
fn main() {
    solve();
}
