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

#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,beans:[(usize,usize);n]
    }
    let mut dict = HashMap::new();
    for (aji, colour) in beans {
        let e = dict.entry(colour).or_insert(BTreeSet::new());
        e.insert(aji);
    }
    let mut ans = BTreeSet::new();
    for (k, v) in &dict {
        ans.insert(v.first().unwrap());
    }
    println!("{}", ans.last().unwrap());
}
fn main() {
    solve();
}
