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
fn isPrime(x: usize) -> bool {
    for i in 2..x.sqrt() as usize + 1 {
        if x % i == 0 {
            return false;
        }
    }
    return true;
}

fn enumeratePrime(x: usize) -> Vec<usize> {
    let mut prime: Vec<usize> = Vec::new();
    let mut notp = vec![false; 1e6 as usize + 1];
    for i in 2..(x + 1).min(1e6 as usize) {
        if notp[i] {
            continue;
        }
        if isPrime(i) {
            prime.push(i);
        }
        let mut coe = 1;
        while coe * i <= x as usize {
            notp[coe * i] = true;
            coe += 1;
        }
    }
    return prime;
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,s:Chars
    }
    let mut check = vec![0usize; 26];
    check[s[0] as usize - 'a' as usize] = 1;
    let mut current = 1;
    for i in 1..n {
        if s[i] != s[i - 1] {
            check[s[i] as usize - 'a' as usize] = check[s[i] as usize - 'a' as usize].max(1);
            current = 1;
        } else {
            current += 1;
            check[s[i] as usize - 'a' as usize] = check[s[i] as usize - 'a' as usize].max(current);
        }
    }
    println!("{}", check.iter().sum::<usize>());
}
fn main() {
    solve();
}
