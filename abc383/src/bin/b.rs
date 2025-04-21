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
        h:usize,w:usize,d:i64,mut field:[Chars;h]
    }
    let mut available: Vec<(usize, usize)> = Vec::new();
    for i in 0..h {
        for j in 0..w {
            if field[i][j] == '#' {
                continue;
            }
            available.push((i, j));
        }
    }
    for v in available.iter().combinations(2) {
        let (fy, fx) = v[0];
        let (sy, sx) = v[1];
    }
}
fn main() {
    solve();
}
