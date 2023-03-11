#[allow(unused_imports)]
use itertools::Itertools;
use maplit::{btreeset, hashset};
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
        n:usize,x:usize,coin:[(usize,usize);n]
    }
    let mut payable = btreeset! {0usize};
    for (price, amount) in coin {
        let mut tmp: BTreeSet<usize> = BTreeSet::new();
        for val in &payable {
            for i in 0..=amount {
                if val + price * i > x {
                    break;
                }
                tmp.insert(val + price * i);
            }
        }
        payable = tmp;
    }
    let ans = if payable.contains(&x) { "Yes" } else { "No" };
    println!("{}", ans);
}
fn main() {
    solve();
}
