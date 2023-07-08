#[allow(unused_imports)]
use itertools::*;
use maplit::hashset;
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
use std::iter::FromIterator;
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
        n:usize,card:[Chars;n]
    }
    let kind: HashSet<Vec<char>> = HashSet::from_iter(card.clone().into_iter());
    if kind.len() != n {
        println!("No");
        return;
    }
    let first = "HDCS";
    let second = "A23456789TJQK";
    for s in card {
        if !first.contains(s[0]) || !second.contains(s[1]) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
fn main() {
    solve();
}
