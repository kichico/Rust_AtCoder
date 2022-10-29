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
        x:i64,y:i64,z:i64
    }
    if x * y < 0 {
        println!("{}", x.abs());
        return;
    }
    if x * z < 0 {
        if x.abs() < y.abs() {
            println!("{}", x.abs());
            return;
        }
        println!("{}", x.abs() + 2 * z.abs());
        return;
    }
    let (x, y, z) = (x.abs(), y.abs(), z.abs());
    if y < z {
        if x < y {
            println!("{}", x);
        } else {
            println!("{}", -1);
        }
        return;
    } else {
        println!("{}", x);
        return;
    }
}

fn main() {
    solve();
}
