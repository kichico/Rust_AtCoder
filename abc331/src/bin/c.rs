#[allow(unused_imports)]
use itertools::*;
use itertools_num::ItertoolsNum;
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
use superslice::Ext;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,a:[usize;n]
    }
    let mut so = a.clone();
    so.sort();
    let so = so.into_iter().collect::<Vec<usize>>();
    let cumsum = so.iter().cumsum().collect::<Vec<usize>>();
    let mut ans: Vec<usize> = Vec::new();
    for i in 0..n {
        let p = so.upper_bound(&a[i]) - 1;
        ans.push(cumsum[cumsum.len() - 1] - cumsum[p]);
    }
    println!("{}", ans.iter().join(" "));
}
fn main() {
    solve();
}
