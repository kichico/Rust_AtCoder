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
        q:usize
    }
    let mut pots: Vec<usize> = Vec::new();
    let mut time = 0;
    let mut border = 0;
    let mut ans = Vec::new();
    for i in 0..q {
        input! {t:usize}
        if t == 1 {
            pots.push(time);
        } else if t == 2 {
            input! { spend:usize }
            time += spend;
        } else {
            input! {h:usize}
            let r = pots.lower_bound(&(h + border)) - pots.lower_bound(&border);
            border = time;
            ans.push(r);
        }
    }
    println!("{}", ans.iter().join("\n"));
}
fn main() {
    solve();
}
