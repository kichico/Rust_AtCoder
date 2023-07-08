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
        n:usize,m:usize,a:[i64;n],b:[i64;m]
    }
    let mut total: Vec<i64> = a
        .clone()
        .into_iter()
        .chain(b.clone().into_iter())
        .collect::<Vec<i64>>();
    total.sort();
    let mut dict: HashMap<i64, i64> = HashMap::new();
    for (i, v) in total.iter().enumerate() {
        dict.insert(*v, (i + 1) as i64);
    }
    let a = a
        .into_iter()
        .map(|x| dict.get(&x).unwrap().clone())
        .collect::<Vec<i64>>();
    let b = b
        .into_iter()
        .map(|x| dict.get(&x).unwrap().clone())
        .collect::<Vec<i64>>();
    println!("{}", a.iter().join(" "));
    println!("{}", b.iter().join(" "));
}
fn main() {
    solve();
}
