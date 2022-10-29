#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use petgraph::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
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
        n:usize,a:[i64;n]
    }
    let mut odd: BTreeSet<i64> = BTreeSet::new();
    let mut even: BTreeSet<i64> = BTreeSet::new();
    let mut zero: BTreeSet<i64> = BTreeSet::new();
    for i in 0..n {
        if a[i] == 0 {
            zero.insert(0);
        } else if a[i] % 2 == 0 {
            even.insert(a[i]);
        } else {
            odd.insert(a[i]);
        }
    }
    let mut ans: BTreeSet<i64> = BTreeSet::new();
    if even.len() >= 1 && zero.len() >= 1 {
        ans.insert(even.iter().next_back().unwrap().clone());
    }
    if even.len() >= 2 {
        let first = even.iter().next_back().unwrap().clone();
        even.remove(&first);
        let second = even.iter().next_back().unwrap().clone();
        ans.insert(first + second);
    }
    if odd.len() >= 2 {
        let first = odd.iter().next_back().unwrap().clone();
        odd.remove(&first);
        let second = odd.iter().next_back().unwrap().clone();
        ans.insert(first + second);
    }
    if ans.is_empty() {
        println!("-1");
    } else {
        println!("{}", ans.iter().next_back().unwrap());
    }
}

fn main() {
    solve();
}
