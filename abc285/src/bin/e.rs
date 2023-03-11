#[allow(unused_imports)]
use itertools::Itertools;
use itertools_num::ItertoolsNum;
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
        n:usize,mut a:[i64;n]
    }
    let mut youbi = vec![(0, 0); n];
    for i in 0..n {
        youbi[i] = (a[i], i);
    }
    youbi.sort();
    let mut left = youbi[n - 2].1;
    let mut right = youbi[n - 1].1;
    if left > right {
        swap(&mut left, &mut right);
    }
    let mut ans = 0;
    for i in 0..n {
        if i == left || i == right {
            continue;
        } else {
            if left < i && i < right {
                let dist_left = i - left + 1;
                let dist_right = right - i + 1;
                println!("{} {} {}", 1, dist_left, dist_right);
                if dist_left > dist_right {
                    a[i] = a[right];
                } else {
                    a[i] = a[left];
                }
            } else if i < left {
                let dist_left = min(left - i + 1, i + n - left + 1);
                let dist_right = min(right - i + 1, i + n - right + 1);
                println!("{} {} {}", 2, dist_left, dist_right);
                if dist_left > dist_right {
                    a[i] = a[right];
                } else {
                    a[i] = a[left];
                }
            } else {
                let dist_left = min(i - left + 1, left + n - i + 1);
                let dist_right = min(right + n - i + 1, i - right + 1);
                println!("{} {} {}", 3, dist_left, dist_right);
                if dist_left > dist_right {
                    a[i] = a[right];
                } else {
                    a[i] = a[left];
                }
            }
        }
    }
    a[left] = 0;
    a[right] = 0;
    println!("a : {}", a.iter().join(" "));
    for x in 0..n {
        if x != left && x != right {
            ans += a[x];
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
