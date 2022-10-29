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
        A:Chars,B:Chars
    }
    for i in 0..min(A.len(), B.len()) {
        let a = A[A.len() - i - 1] as i32 - 48;
        let b = B[B.len() - i - 1] as i32 - 48;
        if a + b >= 10 {
            println!("Hard");
            return;
        }
    }
    println!("Easy");
}

fn main() {
    solve();
}
