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
        n:usize,s:Chars
    }
    let aoki = s
        .iter()
        .filter(|x| **x == 'A')
        .collect::<Vec<&char>>()
        .len();
    let takahashi = s
        .iter()
        .filter(|x| **x == 'T')
        .collect::<Vec<&char>>()
        .len();
    let ans = if aoki == takahashi {
        if *s.iter().next_back().unwrap() == 'A' {
            "T"
        } else {
            "A"
        }
    } else if aoki > takahashi {
        "A"
    } else {
        "T"
    };
    println!("{}", ans);
}
fn main() {
    solve();
}
