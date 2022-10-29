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
        a:usize,b:usize
    }
    let mut ans: HashSet<usize> = HashSet::new();
    if a == 1 {
        ans.insert(1);
    } else if a == 2 {
        ans.insert(2);
    } else if a == 3 {
        ans.insert(1);
        ans.insert(2);
    } else if a == 4 {
        ans.insert(4);
    } else if a == 5 {
        ans.insert(1);
        ans.insert(4);
    } else if a == 6 {
        ans.insert(4);
        ans.insert(2);
    } else if a == 7 {
        ans.insert(1);
        ans.insert(4);
        ans.insert(2);
    }
    if b == 1 {
        ans.insert(1);
    } else if b == 2 {
        ans.insert(2);
    } else if b == 3 {
        ans.insert(1);
        ans.insert(2);
    } else if b == 4 {
        ans.insert(4);
    } else if b == 5 {
        ans.insert(1);
        ans.insert(4);
    } else if b == 6 {
        ans.insert(4);
        ans.insert(2);
    } else if b == 7 {
        ans.insert(1);
        ans.insert(4);
        ans.insert(2);
    }
    let ans: usize = ans.into_iter().sum();
    println!("{}", ans);
}

fn main() {
    solve();
}
