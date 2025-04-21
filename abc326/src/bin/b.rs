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
        n:usize
    }
    for i in n..1000 {
        let s = i.to_string().chars().collect::<Vec<char>>();
        let h = s[0] as i64 - '0' as i64;
        let j = s[1] as i64 - '0' as i64;
        let ichi = s[2] as i64 - '0' as i64;
        if h * j == ichi {
            println!("{}", i);
            return;
        }
    }
}
fn main() {
    solve();
}
