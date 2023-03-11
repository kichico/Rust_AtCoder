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
        n:usize,p:i64,a:[i64;n]
    }
    let mut current: HashSet<i64> = HashSet::new();
    current.insert(a[0]);
    if n == 1 {
        if p == 0 {
            if a[0] % 2 == 0 {
                println!("2");
            } else {
                println!("1");
            }
        } else {
            if a[0] % 2 == 1 {
                println!("1");
            } else {
                println!("0");
            }
        }
        return;
    }
    for i in 1..n {
        let mut another: HashSet<i64> = HashSet::new();
        for x in current {
            if i == n - 1 && (x + a[i]) % 2 == p {
                another.insert(x + a[i]);
            } else {
                println!("{}", x + a[i]);
                another.insert(x + a[i]);
            }
        }
        current = another;
    }
    let ans = if p == 0 {
        1 + current.len()
    } else {
        current.len()
    };
    println!("{}", ans);
}
fn main() {
    solve();
}
