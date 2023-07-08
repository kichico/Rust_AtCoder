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
        h:usize,w:usize,a:[Chars;h],b:[Chars;h]
    }
    for s in 0..h {
        let mut f = a.clone();
        for i in 0..h {
            for j in 0..w {
                f[i][j] = a[(i + s) % h][j];
            }
        }
        for t in 0..w {
            let mut ff = f.clone();
            for i in 0..h {
                for j in 0..w {
                    ff[i][j] = f[i][(j + t) % w];
                }
            }
            let mut isOK = true;
            'loo: for i in 0..h {
                for j in 0..w {
                    if ff[i][j] != b[i][j] {
                        isOK = false;
                        break 'loo;
                    }
                }
            }
            if isOK {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
fn main() {
    solve();
}
