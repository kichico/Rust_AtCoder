use itertools::enumerate;
#[allow(unused_imports)]
use itertools::Itertools;
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
        coord:[(i64,i64);4]
    }
    let neighbor = vec![(3, 1), (0, 2), (1, 3), (2, 0)];
    for (i, (u, v)) in enumerate(neighbor) {
        let va = (coord[u].0 - coord[i].0, coord[u].1 - coord[i].1);
        let vb = (coord[v].0 - coord[i].0, coord[v].1 - coord[i].1);
        if va.0 * vb.1 - vb.0 * va.1 > 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
fn main() {
    solve();
}
