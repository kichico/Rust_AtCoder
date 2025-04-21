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
        n:usize,m:usize,mut a:[usize;n]
    }
    a.sort();
    a.reverse();
    let mut db: Vec<usize> = Vec::new();
    let mut ans = 0;
    let mut con: VecDeque<usize> = VecDeque::new();
    for i in 0..n - m {
        if a.len() <= 1 {
            break;
        }
        let one = a.pop().unwrap();
        let second = a.pop().unwrap();
        con.push_back(one);
        con.push_back(second);
    }
    for i in 0..n - m {
        let one = con.pop_front().unwrap();
        let second = con.pop_back().unwrap();
        ans += (one + second).pow(2);
    }
    println!("{}", ans + a.iter().map(|x| x.pow(2)).sum::<usize>());
}
fn main() {
    solve();
}
