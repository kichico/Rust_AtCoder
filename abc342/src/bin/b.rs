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
#[allow(unused_variables)]
fn solve() {
    input! {
        n:usize,person:[usize;n],q:usize,query:[(usize,usize);q]
    }
    for (f, s) in query {
        let fp = person
            .iter()
            .enumerate()
            .find(|(idx, x)| **x == f)
            .map(|(idx, x)| idx + 1)
            .unwrap();
        let sp = person
            .iter()
            .enumerate()
            .find(|(idx, x)| **x == s)
            .map(|(idx, x)| idx + 1)
            .unwrap();
        let ans = if fp < sp { f } else { s };
        println!("{}", ans);
    }
}
fn main() {
    solve();
}
