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
use std::iter::FromIterator;
#[allow(unused_imports)]
use std::mem::swap;
use superslice::Ext;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
fn isPrime(x: i64) -> bool {
    for i in 2..x.sqrt() as i64 + 1 {
        if x % i == 0 {
            return false;
        }
    }
    return true;
}

fn enumeratePrime(x: i64) -> Vec<i128> {
    let mut prime: Vec<i128> = Vec::new();
    let mut notp = vec![false; 1e6 as usize + 1];
    let upper = std::cmp::min(x as usize + 1, notp.len());
    for i in 2..upper {
        if notp[i] {
            continue;
        }
        if isPrime(i as i64) {
            prime.push(i as i128);
        }
        let mut coe = 1;
        while coe * i < upper {
            notp[coe * i] = true;
            coe += 1;
        }
    }
    return prime;
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:i128
    }
    let v = enumeratePrime(n as i64);
    let prime = v.lower_bound(&300);
    let mut ans = 0;
    for ai in 0..prime {
        let a = v[ai] as i128;
        let rest = n / (a * a);
        let to_c = v.lower_bound(&(rest.sqrt()));
        for ci in ai + 2..=to_c {
            let c = v[ci];
            let bi = v.lower_bound(&(n / (a * a * c * c)));
            let b = v[bi];
            println!("before {} {} {} {}", v[ai], v[bi], v[ci], ci - ai - 1);
            if a * a * b * c * c > n {
                continue;
            }
            if ai < bi && bi < ci {
                println!("{} {} {} {}", v[ai], v[bi], v[ci], ci - ai - 1);
                ans += ci - ai - 1;
            }
        }
    }
    println!("{}", ans);
    local(n);
}

fn local(n: i128) {
    let mut ans = 0;
    for a in 2..=n.sqrt() + 1 {
        if !isPrime(a as i64) {
            continue;
        }
        for b in a + 1..=n {
            if !isPrime(b as i64) {
                continue;
            }
            for c in b + 1..=n.sqrt() + 1 {
                if !isPrime(c as i64) {
                    continue;
                }
                if a * a * b * c * c <= n {
                    println!("local {} {} {}", a, b, c);
                    ans += 1;
                }
            }
        }
    }
    println!("local {}", ans);
}
fn main() {
    solve();
}
