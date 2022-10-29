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
        n:usize,q:usize
    }
    let mut suretu: Vec<Vec<i64>> = vec![Vec::new(); n];
    for i in 0..n {
        input! { l:usize, a:[i64;l] }
        suretu[i] = a;
    }
    input! { query:[(Usize1,Usize1);q]}
    for (s, t) in query {
        println!("{}", suretu[s][t]);
    }
}

fn main() {
    solve();
}
