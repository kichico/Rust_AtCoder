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
        s:Chars
    }
    if s[0] != '0' {
        println!("No");
        return;
    }
    let mut pin: Vec<bool> = vec![false; 7];
    if s[6] == '0' {
        pin[0] = true;
    }
    if s[3] == '0' {
        pin[1] = true;
    }
    if s[7] == '0' && s[1] == '0' {
        pin[2] = true;
    }
    if s[4] == '0' {
        pin[3] = true;
    }
    if s[2] == '0' && s[8] == '0' {
        pin[4] = true;
    }
    if s[5] == '0' {
        pin[5] = true;
    }
    if s[9] == '0' {
        pin[6] = true;
    }
    let mut ans: Vec<i64> = Vec::new();
    for i in 0..7 {
        if !pin[i] {
            ans.push(i as i64);
        }
    }
    if ans.is_empty() || ans.len() == 1 {
        println!("No");
        return;
    }
    let mut prev = ans[0];
    for i in 1..ans.len() {
        if ans[i] != prev + 1 {
            println!("Yes");
            return;
        }
        prev = ans[i];
    }
    println!("No");
}

fn main() {
    solve();
}
