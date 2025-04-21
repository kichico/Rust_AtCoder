use indexing::algorithms::binary_search;
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
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,p:usize,shusai:[usize;n],mut hukusai:[usize;m]
    }
    hukusai.sort();
    let cumsum = hukusai.iter().cumsum().collect::<Vec<usize>>();
    let total = cumsum[m - 1];
    let mut ans = 0;
    for i in 0..n {
        let (mut left, mut right): (usize, usize) = (0, m);
        while right - left > 1 {
            let mid: usize = left + (right - left) / 2;
            if hukusai[mid] + shusai[i] > p {
                right = mid;
            } else {
                left = mid;
            }
        }
        let val = if hukusai[0] + shusai[i] > p {
            p * m
        } else {
            (m - right) * p + cumsum[left] + (left + 1) * shusai[i]
        };
        ans += val;
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
