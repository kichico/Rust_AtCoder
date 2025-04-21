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
        s:Chars
    }
    let n = s.len();
    let mut ans = 1;
    for i in 0..n {
        let mut text: Vec<char> = Vec::new();
        text.push(s[i]);
        'outer: for j in i + 1..n {
            text.push(s[j]);
            for k in 0..text.len() / 2 {
                if text[k] != text[text.len() - 1 - k] {
                    continue 'outer;
                }
            }
            ans = ans.max(text.len());
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
