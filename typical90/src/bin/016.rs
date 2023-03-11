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
        n:i32,mut pr:[i32;3]
    }
    pr.sort();
    let (a, b, c) = (pr[2], pr[1], pr[0]);
    let mut ans = 1e9 as i32;
    for i in 0..10000 {
        if a * i > n {
            break;
        }
        for j in 0..10000 {
            if a * i + b * j > n {
                break;
            }
            let k = n - (a * i + b * j);
            if k % c != 0 {
                continue;
            }
            let k = k / c;
            ans = min(ans, i + j + k);
            //println!("{} {} {}", i, j, k);
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
