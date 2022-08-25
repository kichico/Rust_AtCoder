#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use petgraph::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::mem::swap;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

fn bs(x: usize, border: i64, sum: &VecDeque<i64>) -> usize {
    let (mut left, mut right): (usize, usize) = (x, sum.len());
    if x + 1 == sum.len() {
        return sum.len();
    }
    while right - left > 1 {
        let mid: usize = left + (right - left) / 2;
        if sum[mid] - sum[x] > border {
            right = mid;
        } else {
            left = mid;
        }
    }
    return left;
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,p:i64,q:i64,r:i64,a:[i64;n]
    }
    let mut sum: VecDeque<i64> = VecDeque::new();
    sum.push_back(0);
    sum.push_back(a[0].clone());
    for i in 1..n {
        sum.push_back(sum.iter().next_back().unwrap() + a[i]);
    }
    for x in 0..n - 2 {
        let y = bs(x, p, &sum);
        if sum[y] - sum[x] != p {
            continue;
        }
        let z = bs(y, q, &sum);
        if sum[z] - sum[y] != q {
            continue;
        }
        let w = bs(z, r, &sum);
        if w >= sum.len() {
            continue;
        }
        if sum[w] - sum[z] != r {
            continue;
        }
        println!("Yes");
        return;
    }
    println!("No");
}

fn main() {
    solve();
}
