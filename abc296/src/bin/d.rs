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
fn isPrime(x: i64) -> bool {
    for i in 2..x.sqrt() as i64 + 1 {
        if x % i == 0 {
            return false;
        }
    }
    return true;
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:i64,m:i64
    }
    let limit = 1e7 as i64 + 1;
    let mut miniv = 1e18 as i64;
    for i in 1..limit {
        let (mut left, mut right): (i64, i64) = (1, n);
        while right - left > 1 {
            let mid = left + (right - left) / 2;
            if mid * i < m {
                left = mid;
            } else {
                right = mid;
            }
        }
        if right * i >= m && right <= n && i <= n {
            miniv = miniv.min(right * i);
        }
    }
    if miniv == 1e18 as i64 {
        println!("-1");
    } else {
        println!("{}", miniv)
    };
}

fn main() {
    solve();
}
