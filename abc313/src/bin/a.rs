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
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,mut a:[i64;n]
    }
    let first = a[0].clone();
    a.sort();
    if n == 1 {
        println!("0");
        return;
    }
    if first != a[a.len() - 1] {
        println!("{}", a[a.len() - 1] - first + 1);
    } else if n >= 2 && a[a.len() - 1] != a[a.len() - 2] {
        println!("0");
    } else {
        println!("1");
    }
}
fn main() {
    solve();
}
