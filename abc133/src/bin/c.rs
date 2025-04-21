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
use rand::*;
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
        left:i64,right:i64
    }
    let MOD = 2019;
    if right - left >= 2019 {
        println!("0");
        return;
    }
    let left = left % MOD;
    let right = right % MOD;
    let mut ans = 3000;
    for l in left..=right {
        for r in l + 1..=right {
            ans = ans.min(l * r % MOD);
        }
    }
    println!("{}", ans);
}
fn local() {
    for i in 0..10 {
        let mut rng = thread_rng();
        let mut left = rng.gen_range(0, 1e4 as i64);
        let mut right = rng.gen_range(0, 1e4 as i64);
        if left > right {
            swap(&mut left, &mut right);
        }
        let mut ans = 1e18 as i64;
        let MOD = 2019;
        for l in left..=right {
            for r in l + 1..=right {
                ans = ans.min(l * r % MOD);
            }
        }
        println!("left:{} right:{}", left, right);
        println!("guchoku:{}", ans);
        let left = left % MOD;
        let right = right % MOD;
        let mut ans = 3000;
        println!("left&MOD:{} right%MOD:{}", left, right);
        for l in left..=right {
            for r in l + 1..=right {
                ans = ans.min(l * r % MOD);
            }
        }
        println!("mod:{}", ans);
    }
}
fn main() {
    solve();
}
