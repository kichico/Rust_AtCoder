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
#[fastout]
fn solve() {
    input! {
        n:usize,a:[String;n]
    }
    let mut dict: HashMap<String, i64> = HashMap::new();
    for s in &a {
        if !dict.contains_key(s) {
            println!("{}", s);
            dict.insert(s.to_string(), 1);
        } else {
            let x = dict.get(s).unwrap();
            println!("{}({})", s, x);
            *dict.entry(s.to_string()).or_insert(1) += 1;
        }
    }
}

fn main() {
    solve();
}
