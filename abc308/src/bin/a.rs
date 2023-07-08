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
        a:[i64;8]
    }
    if 100 > a[0] || 675 < a[0] || a[0] % 25 != 0 {
        println!("No");
        return;
    }
    for i in 1..8 {
        if a[i] < a[i - 1] || 100 > a[i] || 675 < a[i] || a[i] % 25 != 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
fn main() {
    solve();
}
