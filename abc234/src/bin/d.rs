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
#[derive(Debug, Clone)]
struct WaveletMatrix {
    size: usize,
    data: Vec<usize>,
}

impl WaveletMatrix {
    pub fn new() -> Self {
        let data: Vec<usize> = Vec::new();
        WaveletMatrix {
            size: 0,
            data: data,
        }
    }
    pub fn build(&mut self) {
        for 
    }
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,k:usize,a:[usize;n]
    }
}
fn main() {
    solve();
}
