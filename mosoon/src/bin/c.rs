#[allow(unused_imports)]
use itertools::*;
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use num::*;
use num_complex::ComplexFloat;
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
use std::mem::take;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize
    }
    let mut v = vec![2, 1, 3, 4, 5, 6];
    let mut cnt = 1;
    while v != vec![1, 2, 3, 4, 5, 6] {
        let id = cnt % 5;
        let idd = id + 1;
        let first: usize = take(&mut v[id]);
        let second = take(&mut v[idd]);
        v[id] = second;
        v[idd] = first;
        cnt += 1;
    }
    let n = n % cnt;
    let mut v = vec![1usize, 2, 3, 4, 5, 6];
    for i in 0..n {
        let id = i % 5;
        let idd = id + 1;
        let first: usize = take(&mut v[id]);
        let second = take(&mut v[idd]);
        v[id] = second;
        v[idd] = first;
    }
    println!("{}", v.iter().join(""));
}
fn main() {
    solve();
}
