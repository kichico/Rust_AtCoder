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
use std::iter::FromIterator;
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
        n:usize,k:Usize1,mut a:[i64;n]
    }
    let mut heap: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    heap.push(Reverse(0));
    let mut ans: Vec<i64> = Vec::new();
    while ans.len() < 2e6 as usize {
        let base = heap.pop().unwrap().0.clone();
        while !heap.is_empty() && heap.peek().unwrap().0 == base {
            heap.pop();
        }
        for i in 0..n {
            ans.push(base + a[i]);
            heap.push(Reverse(base + a[i]));
        }
    }
    ans.sort();
    ans.dedup();
    println!("{}", ans[k]);
}
fn main() {
    solve();
}
