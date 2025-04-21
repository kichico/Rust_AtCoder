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
use superslice::Ext;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        h:usize,w:usize,k:usize,field:[Chars;h]
    }
    let mut saisyo = 1e18 as usize;
    for row in 0..h {
        let mut cnt = 0;
        let mut sousa = 0;
        let mut right = 0;
        for left in 0..w {
            if field[row][left] == 'x' {
                continue;
            }
            if field[row][left] == '.' {
                sousa += 1;
            }
            if left == right {
                right += 1;
            }
            while right + 1 < w && field[row][right + 1] != 'x' {
                right += 1;
                if field[row][right] == '.' {
                    sousa += 1;
                }
            }
        }
    }
}
fn main() {
    solve();
}
