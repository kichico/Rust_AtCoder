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
        x:usize,y:usize
    }
    let a = btreeset! {1,3,5,7,8,10,12};
    let b = btreeset! {4,6,9,11};
    let c = btreeset! {2};
    for st in vec![a, b, c] {
        if st.contains(&x) && st.contains(&y) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
fn main() {
    solve();
}
