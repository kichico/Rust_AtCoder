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
        n:usize,mut a:[usize;n]
    }
    let mut v: Vec<(usize, usize)> = Vec::new();
    v.push((a[0], 1));
    for i in 1..n {
        if a[i] == v[v.len() - 1].0 {
            v.last_mut().unwrap().1 += 1;
        } else {
            v.push((a[i], 1));
        }
        while v[v.len() - 1].1 >= 2 {
            let (val, num) = v.pop().unwrap();
            if num % 2 == 0 {
                if let Some((nv, nn)) = v.last_mut() {
                    if *nv == val + 1 {
                        *nn += num / 2;
                    } else {
                        v.push((val + 1, num / 2));
                    }
                } else {
                    v.push((val + 1, num / 2));
                }
            } else {
                if let Some((nv, nn)) = v.last_mut() {
                    if *nv == val + 1 {
                        *nn += num / 2;
                    } else {
                        v.push((val + 1, num / 2));
                    }
                } else {
                    v.push((val + 1, num / 2));
                }
                v.push((val, 1));
            }
        }
    }
    println!("{}", v.iter().map(|(x, y)| y).sum::<usize>());
}
fn main() {
    solve();
}
