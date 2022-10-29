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
        n:usize,mut a:[i64;n]
    }
    let mut set: BTreeSet<i64> = BTreeSet::new();
    let mut dict: HashMap<i64, i64> = HashMap::new();
    for i in 0..n {
        set.insert(a[i]);
        *dict.entry(a[i]).or_insert(0) += 1;
    }
    let mut aa: Vec<i64> = Vec::new();
    for i in set {
        aa.push(i);
    }
    let mut check: HashMap<i64, i64> = HashMap::new();
    for i in 0..aa.len() {
        check.insert((aa.len() - i - 1) as i64, aa[i]);
    }
    for i in 0..n {
        let x = check.get(&(i as i64));
        if x == None {
            println!("0");
        } else {
            let x = *x.unwrap();
            println!("{}", dict.get(&x).unwrap());
        }
    }
}

fn main() {
    solve();
}
