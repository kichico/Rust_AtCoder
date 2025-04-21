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
        n:i128
    }
    let mut ans = 0;
    for a in 1..=2 {
        let (mut left, mut right): (i128, i128) = (1, n.sqrt());
        while right - left > 1 {
            let b = left + (right - left) / 2;
            if 2i128.pow(a) > 1e9 as i128 && b.pow(2) > 1e9 as i128 {
                right = b;
                continue;
            }
            if 2i128.pow(a) * b.pow(2) > n {
                right = b;
            } else {
                left = b;
            }
        }
        if 2i128.pow(a) * left.pow(2) <= n {
            ans += left;
        } else {
            break;
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
