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
#[allow(unused_imports)]
use superslice::*;
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
    let mut cnt = BTreeMap::new();
    if s.len() % 2 == 1 {
        println!("No");
        return;
    }
    for i in 0..s.len() / 2 {
        if s[2 * i] != s[2 * i + 1] {
            println!("No");
            return;
        }
        *cnt.entry(s[2 * i]).or_insert(0) += 2;
    }
    if cnt.values().all(|x| x == &2 || x == &0) {
        println!("Yes");
    } else {
        println!("No");
    }
}
fn main() {
    solve();
}
