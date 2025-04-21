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
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        mut n:usize
    }
    if n == 0 {
        println!("1");
        return;
    } else if n == 1 {
        println!("0");
        return;
    }
    let mut ans: Vec<usize> = Vec::new();
    while n >= 2 {
        ans.push(n % 2);
        n /= 2;
    }
    ans.push(n);
    let mut cnt = 0;
    for i in 0..ans.len() {
        if ans[i] == 0 {
            cnt += 1;
        } else {
            break;
        }
    }
    println!("{}", cnt);
}
fn main() {
    solve();
}
