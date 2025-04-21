#[allow(unused_imports)]
use itertools::*;
use maplit::btreeset;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
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
    input! {
        e1:String,e2:String
    }
    let short: BTreeSet<&str> = btreeset! {"AB","BC","CD","DE","EA","BA","AE","ED","DC","CB"};
    let e1: &str = &e1;
    let e2: &str = &e2;
    if (short.contains(e1) && short.contains(&e2)) || (!short.contains(e1) && !short.contains(e2)) {
        println!("Yes");
    } else {
        println!("No");
    }
}
fn main() {
    solve();
}
