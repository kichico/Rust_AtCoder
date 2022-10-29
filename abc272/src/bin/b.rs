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
        n:usize,m:usize
    }
    let mut person: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for _ in 0..m {
        input! {
            k:usize,par:[Usize1;k]
        }
        for j in 0..k {
            for p in 0..k {
                person[par[j]].insert(par[p]);
            }
        }
    }
    for i in 0..n {
        //println!("{}", person[i].iter().join(" "));
        if person[i].len() != n {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

fn main() {
    solve();
}
