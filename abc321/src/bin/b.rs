#[allow(unused_imports)]
use itertools::*;
use multimap::*;
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
        n:usize,x:i64,mut a:[i64;n-1]
    }
    a.sort();
    //let mut ans: BTreeSet<i64> = BTreeSet::new();
    for i in 0..=100 {
        a.push(i);
        a.sort();
        let s = a.iter().sum::<i64>() - a[0] - a[a.len() - 1];
        if s >= x {
            println!("{}", i);
            return;
        }
        let mut pos = 0;
        for j in 0..n {
            if a[j] == i {
                pos = j;
                break;
            }
        }
        a.remove(pos);
    }
    println!("-1");
}
fn main() {
    solve();
}
