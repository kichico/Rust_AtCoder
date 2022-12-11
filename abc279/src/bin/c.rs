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
        h:usize,w:usize,s:[Chars;h],t:[Chars;h]
    }
    let mut ss: Vec<Vec<char>> = Vec::new();
    let mut tt = ss.clone();
    for j in 0..w {
        ss.push(Vec::new());
        for i in 0..h {
            ss[j].push(s[i][j]);
        }
    }
    for j in 0..w {
        tt.push(Vec::new());
        for i in 0..h {
            tt[j].push(t[i][j]);
        }
    }
    ss.sort();
    tt.sort();
    for i in 0..w {
        for j in 0..h {
            if ss[i][j] != tt[i][j] {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
fn main() {
    solve();
}
