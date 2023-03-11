#[allow(unused_imports)]
use itertools::Itertools;
use maplit::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
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
    input! {
        n:usize,mut s:Chars
    }
    let mut outside = false;
    for x in &mut s {
        if *x == '"' {
            if outside {
                outside = false;
            } else {
                outside = true;
            }
        } else if *x == ',' && !outside {
            *x = '.';
        }
    }
    println!("{}", &s.iter().join(""));
}
fn main() {
    solve();
}
