#[allow(unused_imports)]
use itertools::*;
use maplit::btreeset;
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
fn solve() {
    input! {
        s:Chars
    }
    let mut stack = Vec::new();
    let tgt = btreeset! {"<>".to_string(),"()".to_string(),"[]".to_string()};
    for c in s {
        stack.push(c);
        while stack.len() >= 2 {
            let ss = stack[stack.len() - 2].to_string() + &stack[stack.len() - 1].to_string();
            if tgt.contains(&ss) {
                stack.pop();
                stack.pop();
            } else {
                break;
            }
        }
    }
    let ans = if stack.is_empty() { "Yes" } else { "No" };
    println!("{}", ans);
}
fn main() {
    solve();
}
