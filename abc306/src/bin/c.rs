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
        n:usize,a:[usize;3*n]
    }
    let mut cnt = vec![0; n + 1];
    let mut ans: Vec<usize> = Vec::new();
    for i in 0..3 * n {
        cnt[a[i]] += 1;
        if cnt[a[i]] == 2 {
            ans.push(a[i]);
        }
    }
    println!("{}", ans.iter().join(" "));
}
fn main() {
    solve();
}
