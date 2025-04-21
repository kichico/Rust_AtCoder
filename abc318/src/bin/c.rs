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
        n:usize,maisu:usize,p:i64,mut fee:[i64;n]
    }
    fee.sort();
    let mut ans = 0;
    while !fee.is_empty() && fee.len() >= maisu {
        let mut current = 0;
        for _i in (n - maisu..n).rev() {
            current += fee[fee.len() - 1];
            fee.pop();
        }
        ans += if current > p { p } else { current };
    }
    let mut current = 0;
    for i in 0..fee.len() {
        current += fee[i];
    }

    ans += if current > p { p } else { current };
    println!("{}", ans);
}
fn main() {
    solve();
}
