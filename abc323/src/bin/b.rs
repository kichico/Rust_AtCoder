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
        n:usize,score:[Chars;n]
    }
    let mut ans: BTreeSet<(i64, usize)> = BTreeSet::new();
    for i in 0..n {
        let cnt = score[i].iter().filter(|x| x == &&'o').count() as i64;
        ans.insert((-cnt, i + 1));
    }
    println!("{}", ans.iter().map(|x| x.1).join(" "));
}
fn main() {
    solve();
}
