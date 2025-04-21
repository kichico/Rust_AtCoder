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

#[allow(dead_code)]
fn local(n: usize, mut s: Vec<char>) -> i64 {
    s.sort();
    let mut cnt: HashSet<i64> = HashSet::new();
    let mut sieve = Eratosthenes::new(1e6 as i64);
    for v in s.iter().permutations(n) {
        let val = v
            .iter()
            .enumerate()
            .map(|x| (**x.1 as i64 - '0' as i64) * 10.pow(x.0 as u32))
            .sum::<i64>();
        if val == 1 {
            cnt.insert(val);
            continue;
        }
        let p = sieve.prime_factorize(val);
        let num = p.iter().filter(|(_x, y)| y == &2).count();
        if p.len() == num {
            println!("heihou:{}", val);
            cnt.insert(val);
        }
    }
    cnt.len() as i64
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,mut s:Chars
    }
    let mut ans = 0;
    let zeros = s.iter().filter(|x| x == &&'0').count();
    s.sort();
    let _lc = 9; // local(n, s.clone());
    let ss: String = s.clone().iter().collect();
    let maxi = (10i64.pow(n as u32)).sqrt() + 1;
    for val in 0..=maxi {
        let mut t = (val * val).to_string().chars().collect::<Vec<char>>();
        if t.len() < ss.len() {
            for _i in 0..zeros {
                t.push('0');
                if t.len() == ss.len() {
                    break;
                }
            }
        }
        t.sort();
        let t: String = t.iter().collect();
        if t == ss {
            ans += 1;
        }
    }

    println!("{}", ans);
}
fn main() {
    solve();
}
