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
        n:usize,muji:usize,s:Chars
    }
    let mut logo = 0;
    let mut logo_stock = 0;
    let mut muji_stock = muji.clone();
    for i in 0..n {
        if s[i] == '1' {
            if muji_stock == 0 {
                if logo_stock == 0 {
                    logo += 1;
                } else {
                    logo_stock -= 1;
                }
            } else {
                muji_stock -= 1;
            }
        } else if s[i] == '2' {
            if logo_stock == 0 {
                logo += 1;
            } else {
                logo_stock -= 1;
            }
        } else {
            logo_stock = logo.clone();
            muji_stock = muji.clone();
        }
    }
    println!("{}", logo);
}
fn main() {
    solve();
}
