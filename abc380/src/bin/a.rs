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
        mut s:Chars
    }
    // let ans = if s.iter().filter(|c| c == &&'1').count() == 1
    //     && s.iter().filter(|c| c == &&'2').count() == 2
    //     && s.iter().filter(|c| c == &&'3').count() == 3
    // {
    //     "Yes"
    // } else {
    //     "No"
    // };
    // println!("{}", ans);
    s.sort();
    let ans = if s.iter().collect::<String>() == "122333".to_string() {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
fn main() {
    solve();
}
