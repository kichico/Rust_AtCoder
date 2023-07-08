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
use std::collections::binary_heap;
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
        n:i64,q:usize,
    }
    let mut first: BTreeSet<i64> = BTreeSet::new();
    let mut second: BTreeSet<i64> = BTreeSet::new();
    let mut ans: Vec<i64> = Vec::new();
    for i in 0..n {
        first.insert(i + 1);
    }
    for _i in 0..q {
        input! { x:usize }
        if x == 1 {
            let v = *first.iter().next().unwrap();
            first.remove(&(v.clone()));
            second.insert(v);
        } else if x == 2 {
            input! { called:i64 }
            if second.contains(&called) {
                second.remove(&called);
            }
        } else {
            let v = *second.iter().next().unwrap();
            ans.push(v);
        }
    }
    for s in ans {
        println!("{}", s);
    }
}
fn main() {
    solve();
}
