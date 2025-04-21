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

#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        w:usize,b:usize
    }
    let s = "wbwbwwbwbwbw";
    let mut ans = String::new();
    for _i in 0..100 {
        ans.push_str(s);
    }
    let len = w + b;
    for i in 0..ans.len() - len {
        let t = &ans[i..i + len].chars().collect::<Vec<char>>();
        if t.iter().filter(|c| c == &&'w').count() == w
            && t.iter().filter(|c| c == &&'b').count() == b
        {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
fn main() {
    solve();
}
