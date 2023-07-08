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
        n:isize
    }
    let tgt = Rational::new_raw(4, n);
    for h in 1..=3500 {
        for w in 1..=3500 {
            let other = tgt - Rational::new_raw(1, h) - Rational::new_raw(1, w);
            if other.numer() == &1 {
                println!("{} {} {}", h, other.denom(), w);
                return;
            }
        }
    }
}

fn local() {
    input! {
        n:isize
    }
    let tgt = Rational::new_raw(4, n);
    for h in 1..=3500 {
        for w in 1..=3500 {
            for t in 1..=3500 {
                if tgt == Rational::new_raw(h * t + h * w + w * t, h * w * t) {
                    println!("{} {} {}", h, w, t);
                    return;
                }
            }
        }
    }
    println!("Not found");
}
fn main() {
    solve();
    //local();
}
