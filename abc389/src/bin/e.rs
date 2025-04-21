#[allow(unused_imports)]
use itertools::*;
use libm::tan;
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
        n:usize,mut m:usize, mut price:[usize;n]
    }
    price.sort();
    let mut ans = 0;
    for i in 0..n - 1 {
        let (mut left, mut right): (usize, usize) = (0, 1e9 as usize);
        while right - left > 1 {
            let mid: usize = left + (right - left) / 2;
            let tanka = (mid * mid * price[i]) as f64 / mid as f64;
            if tanka > price[i + 1] as f64 || mid * mid * price[i] > m {
                right = mid;
            } else {
                left = mid;
            }
        }
        ans += left;
        println!("left:{}", left);
        m -= left * left * price[i];
    }
    let (mut left, mut right): (usize, usize) = (0, 1e9 as usize);
    while right - left > 1 {
        let mid: usize = left + (right - left) / 2;
        if mid * mid * price.last().unwrap() > m {
            right = mid;
        } else {
            left = mid;
        }
        ans += left;
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
