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
        n:usize,k:usize,mut s:Chars
    }
    s.sort();
    let mut ans = HashSet::new();
    'notpass: for v in s.iter().permutations(n) {
        'outer: for start in 0..=n - k {
            for i in 0..=k / 2 {
                let left = i + start;
                let right = k + start - i - 1;
                if v[left] != v[right] {
                    //println!("{}", v.iter().join(""));
                    continue 'outer;
                }
            }
            continue 'notpass;
        }
        ans.insert(v);
    }
    println!("{}", ans.len());
}
fn main() {
    solve();
}
