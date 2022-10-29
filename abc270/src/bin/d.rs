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
        mut n:i64,k:usize,a:[i64;k]
    }
    let mut retu: BTreeSet<i64> = BTreeSet::new();
    let mut aoki = 0;
    let mut takahashi = 0;
    for x in a {
        if x <= n as i64 {
            retu.insert(x);
        }
    }
    while !retu.is_empty() {
        let s = retu.iter().next_back().unwrap().clone();
        let mi = retu.iter().next().unwrap();
        if *mi > n as i64 {
            break;
        }
        if s > n {
            retu.remove(&s);
            continue;
        }
        takahashi += s;
        n -= s;
        while !retu.is_empty() {
            let c = retu.iter().next_back().unwrap().clone();
            if c <= n {
                break;
            } else {
                retu.remove(&c);
            }
        }
        if retu.is_empty() {
            break;
        }
        let s = retu.iter().next_back().unwrap().clone();
        let mi = retu.iter().next().unwrap();
        if *mi > n as i64 {
            break;
        }
        aoki += s;
        n -= s;
        println!("n {} takahashi {} aoki {}", n, takahashi, aoki);
    }
    println!("{}", takahashi);
}

fn main() {
    solve();
}
