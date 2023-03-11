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
        n:usize,student:[(usize,i64);n],q:usize,query:[(usize,usize);q]
    }
    let mut one = vec![0; n + 1];
    let mut two = one.clone();
    for (i, (class, score)) in enumerate(student) {
        if i == 0 {
            if class == 1 {
                one[i + 1] = score;
            } else {
                two[i + 1] = score;
            }
        } else {
            if class == 1 {
                one[i + 1] = one[i] + score;
                two[i + 1] = two[i];
            } else {
                two[i + 1] = two[i] + score;
                one[i + 1] = one[i];
            }
        }
    }
    for (l, r) in query {
        println!("{} {}", one[r] - one[l - 1], two[r] - two[l - 1]);
    }
}
fn main() {
    solve();
}
