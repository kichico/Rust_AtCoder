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
        mut h:Chars,mut m:Chars
    }
    if h.len() < 2 {
        h.push('0');
        h.reverse();
    }
    if m.len() < 2 {
        m.push('0');
        m.reverse();
    }
    let new_h: i64 = (h[0].to_string() + &m[0].to_string()).parse().unwrap();
    let new_m: i64 = (h[1].to_string() + &m[1].to_string()).parse().unwrap();
    if new_h < 24 && new_m < 60 {
        println!("{} {}", h.iter().join(""), m.iter().join(""));
    }
    if new_m >= 60 {
        h[1] = '0';
        h[0] = if h[0] == '0' { '1' } else { '2' };
        m[0] = '0';
        m[1] = '0';
        println!("{} {}", h.iter().join(""), m.iter().join(""));
        return;
    }
    if new_h >= 24 {
        if h[0] == '2' && h[1] == '3' {
            h[0] = '0';
            h[1] = '0';
        } else {
            h[1] = to_char((h[1] as i64) - 47);
        }
        m[0] = '0';
        m[1] = '0';
        println!("{} {}", h.iter().join(""), m.iter().join(""));
    }
}
fn main() {
    solve();
}
