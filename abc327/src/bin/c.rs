#[allow(unused_imports)]
use itertools::*;
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
        field:[[usize;9];9]
    }
    for row in 0..9 {
        let mut check: HashSet<usize> = HashSet::new();
        for col in 0..9 {
            check.insert(field[row][col]);
        }
        if check.len() != 9 {
            println!("No");
            return;
        }
    }

    for col in 0..9 {
        let mut check: HashSet<usize> = HashSet::new();
        for row in 0..9 {
            check.insert(field[row][col]);
        }
        if check.len() != 9 {
            println!("No");
            return;
        }
    }
    for row_head in (0..9).step_by(3) {
        for col_head in (0..9).step_by(3) {
            let mut check: HashSet<usize> = HashSet::new();
            for row in row_head..row_head + 3 {
                for col in col_head..col_head + 3 {
                    check.insert(field[row][col]);
                }
            }
            if check.len() != 9 {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
fn main() {
    solve();
}
