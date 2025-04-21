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
#[derive(Debug, Clone)]
pub struct Eratosthenes {
    min_factor: Vec<usize>,
}

impl Eratosthenes {
    pub fn new(n: i64) -> Self {
        let n = n as usize;
        let mut min_factor = vec![0usize; n as usize + 1];
        min_factor[1] = 1;
        for i in 2..=n {
            if min_factor[i] != 0 {
                continue;
            }
            min_factor[i] = i;
            for factor in (i * 2..=n).step_by(i) {
                if min_factor[factor] == 0 {
                    min_factor[factor] = i;
                }
            }
        }
        return Eratosthenes { min_factor };
    }
    pub fn prime_factorize(&mut self, n: i64) -> Vec<(i64, i64)> {
        let mut n = n as usize;
        let mut ret: Vec<(i64, i64)> = Vec::new();
        while n > 1 {
            let p = self.min_factor[n] as usize;
            let mut exp = 0;
            while self.min_factor[n] == p {
                n /= p;
                exp += 1;
            }
            ret.push((p as i64, exp));
        }
        return ret;
    }
    pub fn enumerate_divisors(&mut self, n: i64) -> Vec<i64> {
        let mut ret = vec![1i64];
        let pf = self.prime_factorize(n);
        for (prime, exp) in pf {
            let mut divisor = 1;
            for _i in 0..exp {
                divisor *= prime;
                ret.push(divisor);
            }
        }
        ret.sort();
        return ret;
    }
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
        t:usize,cases:[i64;t]
    }
    'outer: for n in cases {
        let primes = prime_factorize(n);
        if primes.len() == 1 {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
fn main() {
    solve();
}
