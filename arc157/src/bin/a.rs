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
        n:usize,a:usize,mut b:usize,mut c:usize,d:usize
    }
    let mut ans = if a > 0 { vec![1; 1] } else { Vec::new() };
    for _i in 0..a {
        ans.push(1);
    }
    let mut num = 0;
    while b > 0 || c > 0 {
        if num == 0 {
            b -= 1;
        } else {
            c -= 1;
        }
        ans.push(num);
        num = 1 - num;
    }
    if (b == 0 && c == 0) || (b == 1 && d > 0) || (c == 1) {
        println!("Yes");
    }
}
fn main() {
    solve();
}
