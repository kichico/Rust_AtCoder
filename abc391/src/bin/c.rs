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
        n:usize,q:usize
    }
    let mut over2 = BTreeSet::new();
    let mut num2box: BTreeMap<usize, usize> = BTreeMap::new();
    let mut boxes: Vec<usize> = vec![1; n];
    for i in 0..n {
        num2box.insert(i, i);
    }
    for i in 0..q {
        input! {t:usize}
        if t == 1 {
            input! {p:Usize1,b:Usize1}
            let prev = *num2box.get(&p).unwrap();
            boxes[prev] -= 1;
            if over2.contains(&prev) && boxes[prev] == 1 {
                over2.remove(&prev);
            }
            let next = num2box.get_mut(&p).unwrap();
            *next = b;
            boxes[*next] += 1;
            if boxes[*next] > 1 {
                over2.insert(*next);
            }
        } else {
            println!("{}", over2.len());
        }
    }
}
fn main() {
    solve();
}
