#[allow(unused_imports)]
use itertools::Itertools;
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

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,k:usize
    }
    if k == 1 {
        println!("{}", n - 1);
        return;
    }
    let mut ans = 0;
    let mut factors = vec![0; n as usize + 1];
    for x in 2..=n {
        if factors[x] != 0 {
            continue;
        }
        for v in (x..=n).step_by(x) {
            factors[v] += 1;
        }
    }
    println!("{}", factors.iter().filter(|x| **x >= k).count());
}
fn main() {
    solve();
}
