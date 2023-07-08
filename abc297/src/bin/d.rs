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
#[allow(dead_code)]
fn local(mut a: i64, mut b: i64) {
    let mut ans = 0;
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
        //println!("{} {}", a, b);
        ans += 1;
    }
    println!("local {}", ans);
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        mut a:i64,mut b:i64
    }
    let mut ans = 0;
    //local(a, b);
    if a == b {
        println!("0");
        return;
    }
    while a != b && a != 0 && b != 0 {
        if a > b {
            ans += a / b;
            a %= b;
        } else {
            ans += b / a;
            b %= a;
        }
    }
    println!("{}", ans - 1);
}
fn main() {
    solve();
}
