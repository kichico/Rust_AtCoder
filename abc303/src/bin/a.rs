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
        n:usize,s:Chars,t:Chars
    }
    for i in 0..n {
        let mut u = s[i];
        let mut v = t[i];
        if u == v {
            continue;
        }
        if (u == '1' && v == 'l') || (u == '0' && v == 'o') {
            continue;
        }
        swap(&mut u, &mut v);
        if (u == '1' && v == 'l') || (u == '0' && v == 'o') {
            continue;
        }
        println!("No");
        return;
    }
    println!("Yes");
}
fn main() {
    solve();
}
