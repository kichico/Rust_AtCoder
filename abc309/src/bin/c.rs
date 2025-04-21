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
        n:usize,k:i64,mut medicine:[(i64,i64);n]
    }
    medicine.sort();
    let mut current = medicine.iter().map(|x| x.1).sum::<i64>();
    let mut ans = 1;
    if current <= k {
        println!("1");
        return;
    }
    for (day, num) in medicine {
        ans = day;
        current -= num;
        if current <= k {
            println!("{}", ans + 1);
            return;
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
