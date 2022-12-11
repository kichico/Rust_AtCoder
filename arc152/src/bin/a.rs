#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use petgraph::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
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
        n:usize,mut isu:i64,mut a:[i64;n]
    }
    let mut pair = 0;
    for i in 0..n {
        if a[i] == 2 {
            pair += 1;
        }
    }
    if pair == 0 || (isu == 2 && pair == 1) {
        println!("Yes");
    }
    let mut current = 1;
    let mut length = isu.clone();
    for i in 0..pair {
        if isu - 2 < 0 || current + 2 > length {
            println!("No");
            return;
        }
        isu -= 2;
        current += 2;
    }
    println!("Yes");
}
fn main() {
    solve();
}
