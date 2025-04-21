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
#[allow(unused_imports)]
use std::ops::Bound::{Excluded, Included, Unbounded};
#[allow(unused_imports)]
use superslice::*;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,s:[Chars;n]
    }
    let m = s.iter().map(|x| x.len()).max().unwrap();
    let mut ans = vec![vec!['*'; n]; m];
    for i in 0..n {
        let col = n - i - 1;
        for j in 0..s[i].len() {
            ans[j][col] = s[i][j];
        }
    }
    for row in 0..m {
        let mut id = n - 1;
        while ans[row][id] == '*' {
            ans[row].pop();
            id -= 1;
            if id == 0 {
                break;
            }
        }
    }
    for i in ans {
        println!("{}", i.iter().join(""));
    }
}
fn main() {
    solve();
}
