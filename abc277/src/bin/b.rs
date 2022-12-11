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
        n:usize,s:[Chars;n]
    }
    let mut all: HashSet<Vec<char>> = HashSet::new();
    let first = "HDCS".to_string();
    let second = "A23456789TJQK".to_string();
    for i in 0..n {
        all.insert(s[i].clone());
        if !first.contains(s[i][0]) || !second.contains(s[i][1]) {
            println!("No");
            return;
        }
    }
    if all.len() != n {
        println!("No");
    } else {
        println!("Yes");
    }
}
fn main() {
    solve();
}
