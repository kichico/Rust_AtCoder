#[allow(unused_imports)]
use itertools::Itertools;
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
        n:usize,pos:[Usize1;4],mut a:[i64;n]
    }
    let len = pos[1] - pos[0] + 1;
    let mut now = pos[0];
    let mut other = pos[2];
    for _i in 0..len {
        let tmp = a[now].clone();
        a[now] = a[other].clone();
        a[other] = tmp;
        now += 1;
        other += 1;
    }
    println!("{}", a.iter().join(" "));
}
fn main() {
    solve();
}
