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

#[allow(non_snake_case)]
fn solve() {
    input! {
        a:i64,b:i64,mut w:i64
    }
    let mut mini = 1e18 as i64;
    let mut maxi = 0;
    w *= 1000;
    for i in 1..=1000000 {
        if a * i <= w && b * i >= w {
            mini = mini.min(i);
            maxi = maxi.max(i);
        }
    }
    if maxi == 0 {
        println!("UNSATISFIABLE");
    } else {
        println!("{} {}", mini, maxi);
    }
}
fn main() {
    solve();
}
