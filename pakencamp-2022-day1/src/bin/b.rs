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

fn local() {
    for n in 1..10 {
        for m in 1..10 {
            if m == 1 {
                if n != 1 {
                    println!("-1")
                } else {
                    println!("1");
                }
            } else if n == 1 {
                println!("1");
            }
            println!("{} {}", n, m);
            let n = n.saturating_sub(m);
            println!("{}", (n + m - 2) / (m - 1) + 1);
        }
    }
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        mut n:usize,m:usize
    }
    if m == 1 {
        if n != 1 {
            println!("-1")
        } else {
            println!("0");
        }
        return;
    } else if n == 1 {
        println!("0");
        return;
    }
    let n = n.saturating_sub(m);
    println!("{}", (n + m - 2) / (m - 1) + 1);
}
fn main() {
    //local();
    solve();
}
