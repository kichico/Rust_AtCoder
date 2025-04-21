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
fn prime_factorize(mut x: i64) -> Vec<(i64, i64)> {
    let mut factor: Vec<(i64, i64)> = Vec::new();
    let sq = x.sqrt() + 1;
    let mut isSeeked = vec![true; sq as usize + 1];
    for i in 2..=sq {
        if !isSeeked[i as usize] {
            continue;
        }
        let mut exp_num = 0;
        while x % i == 0 {
            exp_num += 1;
            x /= i;
        }
        for n in (i * 2..=sq).step_by(i as usize) {
            isSeeked[n as usize] = false;
        }
        if exp_num != 0 {
            factor.push((i, exp_num));
        }
    }
    if x > 1 {
        factor.push((x, 1));
    }
    return factor;
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        mut n:i64
    }
    if n == 1 {
        println!("0");
        return;
    }
    let mut ans = 0;
    let mut factors = prime_factorize(n);
    while let Some((factor, mut cnt)) = factors.pop() {
        let mut div = 1;
        while cnt >= div {
            n /= factor.pow(div as u32);
            cnt -= div;
            div += 1;
            ans += 1;
            if n == 1 {
                println!("{}", ans);
                return;
            }
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
