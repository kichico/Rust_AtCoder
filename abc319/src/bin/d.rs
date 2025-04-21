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
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,words:[usize;n]
    }
    let (mut left, mut right): (usize, usize) = (
        *words.iter().max().unwrap() - 1,
        words.iter().sum::<usize>() + n,
    );
    while right - left > 1 {
        let w: usize = left + (right - left) / 2;
        let mut cnt = 1;
        let mut current = 0;
        for i in 0..n {
            if current + words[i] > w {
                current = words[i] + 1;
                cnt += 1;
            } else {
                current += words[i] + 1;
            }
        }
        if cnt > m {
            left = w;
        } else {
            right = w;
        }
    }
    let mut current = 0;
    let mut check = right.clone();
    for k in 0..50 {
        check -= 1;
        if check == 0 {
            break;
        }
        for i in 0..n {
            if current + words[i] > left {
                current = words[i] + 1;
            } else {
                current += words[i] + 1;
            }
        }
    }

    println!("{}", right);
}
fn main() {
    solve();
}
