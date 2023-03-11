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
        n:usize,mut a:Chars,mut b:Chars
    }
    for i in 0..n {
        if a[i] as u8 > b[i] as u8 {
            swap(&mut a[i], &mut b[i]);
        }
    }
    let a: String = a.iter().collect();
    let b: String = b.iter().collect();
    let a: num::BigInt = a.parse().unwrap();
    let b: num::BigInt = b.parse().unwrap();
    println!("{}", (a * b) % 998244353);
}
fn main() {
    solve();
}
