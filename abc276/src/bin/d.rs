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
        n:usize,mut a:[i64;n]
    }
    let mut all: HashSet<i64> = HashSet::new();
    for i in 0..n {
        all.insert(a[i]);
    }
    let mut ans = 0;
    if all.len() == 1 {
        println!("0");
        return;
    }
    all.clear();
    let mut gcd_value = a[0];
    for i in 1..n {
        gcd_value = gcd(gcd_value, a[i]);
    }
    for i in 0..n {
        a[i] /= gcd_value;
    }
    for i in 0..n {
        while a[i] % 2 == 0 {
            a[i] /= 2;
            ans += 1;
        }
        while a[i] % 3 == 0 {
            a[i] /= 3;
            ans += 1;
        }
        all.insert(a[i]);
    }
    if all.len() != 1 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
fn main() {
    solve();
}
