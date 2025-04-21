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
        mut s:Chars
    }
    let n = s.len().clone();
    let mut ans = Vec::new();
    s.reverse();
    while let Some(x) = s.pop() {
        if ans.len() % 2 == 0 && x == 'o' {
            ans.push('i');
        } else if ans.len() % 2 == 1 && x == 'i' {
            ans.push('o');
        }
        ans.push(x);
    }
    if ans.iter().last().unwrap() == &'i' {
        ans.push('o');
    }
    println!("{}", ans.len() - n);
}
fn main() {
    solve();
}
