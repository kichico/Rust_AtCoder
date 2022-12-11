#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use petgraph::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use rand_core::impls::fill_via_u32_chunks;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
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
    let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    let len = s.len();
    let mut value: Vec<char> = Vec::new();
    for i in 0..len {
        if alpha.contains(s[i]) {
            continue;
        }
        value.push(s[i]);
    }
    if value.len() != len - 2 || value[0] == '0' {
        println!("No");
    } else {
        println!("Yes");
    }
}
fn main() {
    solve();
}
