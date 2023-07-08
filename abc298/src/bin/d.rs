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
fn modpow(mut x: i128, mut n: i128, MOD: i128) -> i128 {
    let mut ans = 1;
    while n > 0 {
        if n & 1 == 1 {
            ans = (ans * x) % MOD;
        }
        x = (x * x) % MOD;
        n >>= 1;
    }
    return ans;
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,
    }
    let mut base: VecDeque<i128> = VecDeque::new();
    base.push_back(1);
    let mut amari = 1;
    const MOD: i128 = 998244353;
    for i in 0..n {
        input! {x:i64}
        if x == 1 {
            input! {v:i128}
            base.push_back(v);
            amari = (amari * 10 + v) % MOD;
        } else if x == 2 {
            let suffix = base.pop_front().unwrap();
            amari = (amari - suffix * modpow(10, base.len() as i128, MOD)) % MOD;
            if amari < 0 {
                amari += MOD;
            }
        } else {
            println!("{}", amari % MOD);
        }
    }
}
fn main() {
    solve();
}
