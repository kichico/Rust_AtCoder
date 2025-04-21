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
        n:usize,m:usize,mut a:[usize;n]
    }
    a.sort();
    let sum = a.iter().sum::<usize>();
    if sum <= m {
        println!("infinite");
        return;
    }
    let (mut left, mut right): (usize, usize) = (0, 1e18 as usize);
    let cumsum = a.iter().cumsum().collect::<Vec<usize>>();
    while right - left > 1 {
        let mid: usize = left + (right - left) / 2;
        let p = a.lower_bound(&mid);
        let total = if p == 0 {
            mid * n
        } else {
            cumsum[p - 1] + mid * (n - p)
        };
        if total > m {
            right = mid;
        } else {
            left = mid;
        }
    }
    println!("{}", left);
}
fn main() {
    solve();
}
