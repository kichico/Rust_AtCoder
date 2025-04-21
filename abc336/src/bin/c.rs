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
        n:i128
    }
    if n <= 5 {
        let mut base = 0;
        for _i in 1..n {
            base += 2;
        }
        println!("{}", base);
        return;
    }
    let mut keta = 2;
    let mut cnt = 5;
    while cnt < n {
        cnt += 5.pow(keta - 1) * 4;
        keta += 1;
    }
    keta -= 1;
    let mut current = 0;
    let mut ans = vec![2; keta as usize];
    for i in (2..12).step_by(2) {
        if i * 5.pow(keta - 1) / 2 >= n {
            ans[0] = i - 2;
            current += (i - 2) * 5.pow(keta - 1) / 2;
            break;
        }
    }

    for i in 1..keta {
        for val in (0..12).step_by(2) {
            let _db = current + val * 5.pow(keta - i - 1) / 2;
            if current + val * 5.pow(keta - i - 1) / 2 >= n {
                ans[i as usize] = val - 2;
                current += (val - 2) * 5.pow(keta - i - 1) / 2;
                break;
            }
        }
    }
    println!("{}", ans.iter().join(""));
}
fn main() {
    solve();
}
