#[allow(unused_imports)]
use std::hash::Hash;
#[allow(unused_imports)]
use itertools::*;
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
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque,BinaryHeap};
#[allow(unused_imports)]
use std::mem::swap;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(dead_code)]
fn clamp(val: usize, diff: i64, min: usize, max: usize) -> usize {
    let val = val as i64;
    if val + diff < min as i64 {
        return min;
    } else if val + diff > max as i64 {
        return max;
    } else {
        return (val + diff) as usize;
    }
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize
    }
}
fn main() {
    solve();
}
