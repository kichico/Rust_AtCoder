#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
use proconio::input_interactive;
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
    input_interactive! {
        n:usize
    }
    let mut m = 1;
    while 2.pow(m) < n {
        m += 1;
    }
    println!("{}", m);
    let mut drink: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); m as usize];
    let m = m as usize;
    for i in 0..n {
        let mut wine = i.clone();
        let mut binary: Vec<usize> = Vec::new();
        while wine >= 1 {
            binary.push(wine % 2);
            wine /= 2;
        }
        if wine == 1 {
            binary.push(wine);
        }
        while binary.len() < m {
            binary.push(0);
        }
        binary.reverse();
        for p in 0..m {
            if binary[p] == 1 {
                drink[p].insert(i + 1);
            }
        }
    }
    for i in 0..m {
        println!("{} {}", drink[i].len(), drink[i].iter().join(" "));
    }
    input_interactive! {
        mut s:Chars
    }
    let mut ans = 0;
    for i in 0..m {
        if s[i] == '1' {
            ans += 2.pow((m - i - 1) as u32);
        }
    }
    println!("{}", ans + 1);
}
fn main() {
    solve();
}
