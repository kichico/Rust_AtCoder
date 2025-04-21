use ac_library::{ModInt, ModInt998244353};
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
fn to_Binary(mut x: i128, keta: usize) -> Vec<char> {
    let mut v: Vec<char> = Vec::new();
    if x == 0 {
        v.push('0');
    }
    while x > 0 {
        let c = std::char::from_digit((x % 2) as u32, 10).unwrap();
        v.push(c);
        x /= 2;
    }
    while v.len() <= keta {
        v.push('0');
    }
    v.reverse();
    return v;
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize
    }
    let mut ans = ModInt998244353::new(0);
    for i in 0..=n {
        let v = (i & m).count_ones();
        println!("{}:{}", i, v);
        ans += v;
        println!("i:{}", to_Binary(i as i128, 10).iter().join(""));
        println!("m:{}", to_Binary(m as i128, 10).iter().join(""));
        println!("&:{}", to_Binary((i & m) as i128, 10).iter().join(""));
        println!("----------");
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
