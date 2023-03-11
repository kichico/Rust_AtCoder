#[allow(unused_imports)]
use itertools::Itertools;
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
fn isPrime(x: i64) -> bool {
    for i in 2..x.sqrt() as i64 + 1 {
        if x % i == 0 {
            return false;
        }
    }
    return true;
}
#[allow(non_snake_case)]
fn enumeratePrime(x: i64) -> Vec<i64> {
    let mut prime: Vec<i64> = Vec::new();
    let mut notp = vec![false; 1e7 as usize + 1];
    for i in 2..min(x as usize + 1, 1e7 as usize) {
        if notp[i] {
            continue;
        }
        if isPrime(i as i64) {
            prime.push(i as i64);
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
        n:usize,a:[i64;n]
    }
    let mut an = a.clone();
    an.sort();
    let primes = enumeratePrime(an[n - 1].cbrt() + 1);
    'first: for x in a {
        for p in &primes {
            if x % (p * p) != 0 {
                continue;
            }
            let q = x / (p * p);
            println!("{} {}", p, q);
            continue 'first;
        }
        for q in &primes {
            if x % q != 0 {
                continue;
            }
            let p2 = x / q;
            let p = p2.sqrt();
            if p * p != p2 {
                continue;
            }
            println!("{} {}", p, q);
        }
    }
}

fn main() {
    solve();
}
