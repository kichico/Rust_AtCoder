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
fn isPrime(x: i64) -> bool {
    for i in 2..x.sqrt() as i64 + 1 {
        if x % i == 0 {
            return false;
        }
    }
    return true;
}

fn enumerate_prime(x: i64) -> Vec<i128> {
    let mut prime: Vec<i128> = Vec::new();
    let mut notp = vec![false; 1e6 as usize + 2];
    for i in 2..1e6 as usize + 2 {
        if notp[i] {
            continue;
        }
        if isPrime(i as i64) {
            prime.push(i as i128);
        }
        let mut coe = 1;
        while coe * i <= x as usize {
            notp[coe * i] = true;
            coe += 1;
        }
    }
    return prime;
}
fn solve() {
    input! {
        n:i128
    }
    let primes = enumerate_prime(1e6 as i64);
    let mut ans = 0;
    for i in 1..primes.len() {
        let q = primes[i];
        let (mut left, mut right): (usize, usize) = (0, i);
        while right - left > 1 {
            let mid: usize = left + (right - left) / 2;
            if primes[mid] * q.pow(3) > n {
                right = mid;
            } else {
                left = mid;
            }
        }
        //println!("max p:{} q:{}", left, q);
        if 2 * q.pow(3) <= n {
            ans += left + 1;
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
