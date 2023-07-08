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
       board:[Chars;8]
    }
    let tate = vec![8, 7, 6, 5, 4, 3, 2, 1];
    let yoko = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    for i in 0..8 {
        for j in 0..8 {
            if board[i][j] == '*' {
                println!("{}{}", yoko[j], tate[i]);
            }
        }
    }
}
fn main() {
    solve();
}
