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
        h:usize,w:usize,table:[Chars;h]
    }
    let mut ans = table.clone();
    for i in 0..h {
        let mut j = 0;
        while j < w - 1 {
            if table[i][j] == 'T' && table[i][j + 1] == 'T' {
                ans[i][j] = 'P';
                ans[i][j + 1] = 'C';
                j += 2;
            } else {
                j += 1;
            }
        }
    }
    for i in 0..h {
        println!("{}", ans[i].iter().join(""));
    }
}
fn main() {
    solve();
}
