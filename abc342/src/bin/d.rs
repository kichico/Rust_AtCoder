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
#[allow(unused_imports)]
use std::ops::Bound::{Excluded, Included, Unbounded};

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
fn solve() {
    input! {
        n:usize,a:[i64;n]
    }
    let mut sieve = Eratosthenes::new(2e5 as i64);
    let mut cnt = Vec::new();
    for i in 0..n {
        cnt.push(sieve.prime_factorize(a[i]));
    }
    let mut odd: HashMap<i64, i64> = HashMap::new();
    let mut even = 0;
    let mut mp = HashMap::new();
    for i in 0..n {
        let mut odd_val = 1;
        for (v, k) in &cnt[i] {
            if k % 2 == 1 {
                odd_val *= v;
            }
        }
        if odd_val == 1 {
            even += 1;
        } else {
            *odd.entry(odd_val).or_insert(0) += 1;
        }
        mp.insert(i, odd_val);
    }
    let mut ans = 0;
    for i in 0..n {
        if a[i] == 0 {
            ans += (n - i - 1) as i64;
            even -= 1;
            continue;
        }
        let val = mp.get(&i).unwrap();
        if *val == 1 && even > 1 {
            ans += even - 1;
            even -= 1;
        }
        if let Some(num) = odd.get_mut(val) {
            if *num > 1 {
                ans += num.clone() - 1;
                *num -= 1;
            }
        }
        //println!("{}:{}", i, ans);
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
