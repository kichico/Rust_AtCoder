#[allow(unused_imports)]
use itertools::*;
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
        n:usize,tar:i64,col:[i64;n],val:[i64;n]
    }
    let mut color: HashMap<i64, BTreeMap<i64, usize>> = HashMap::new();
    for (i, (c, v)) in enumerate(zip(&col, val)) {
        if !color.contains_key(&c) {
            color.insert(*c, BTreeMap::new());
            let e = color.get_mut(&c).unwrap();
            e.insert(v, i);
        } else {
            let e = color.get_mut(&c).unwrap();
            e.insert(v, i);
        }
    }
    if color.contains_key(&tar) {
        let e = color.get(&tar).unwrap();
        println!("{}", e.iter().next_back().unwrap().1 + 1);
    } else {
        let e = color.get(&col[0]).unwrap();
        println!("{}", e.iter().next_back().unwrap().1 + 1);
    }
}
fn main() {
    solve();
}
