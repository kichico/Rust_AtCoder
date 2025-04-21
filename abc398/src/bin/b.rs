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
        cards:[usize;7]
    }
    let mut cnt = HashMap::new();
    for i in 0..7 {
        *cnt.entry(cards[i]).or_insert(0) += 1usize;
    }
    let mut two = 0;
    let mut three_or_high = 0;
    for (_k, v) in cnt {
        if v == 2 {
            two += 1;
        } else if v >= 3 {
            three_or_high += 1;
        }
    }
    if (two >= 1 && three_or_high >= 1) || three_or_high >= 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
fn main() {
    solve();
}
