use bitvec::ptr::read_unaligned;
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
        cards:[usize;4]
    }
    let mut checker: BTreeMap<usize, usize> = BTreeMap::new();
    for c in cards {
        *checker.entry(c).or_insert(0) += 1;
    }
    if checker.len() != 2 {
        println!("No");
        return;
    }
    let mut vals = checker.values().collect_vec();
    vals.sort();
    let ans = if (*vals[0] == 1 && *vals[1] == 3) || (*vals[0] == 2 && *vals[1] == 2) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
fn main() {
    solve();
}
