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
        mut s:i64
    }
    let mut ans = 0;
    let n = s.to_string().len() as u32;
    if s < 1000 {
        println!("0");
        return;
    }
    s -= 999;
    let mut i = 3;
    while s > 0 {
        let val = 10.pow(i + 3) - 10.pow(i);
        if s - val <= 0 {
            break;
        }
        println!("{} {} {}", s, val, i);
        ans += val;
        if s - val >= 0 {
            s -= val;
        }
        i += 3;
    }
    println!("{}", ans + s);
}
fn main() {
    solve();
}
