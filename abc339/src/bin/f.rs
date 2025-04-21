#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
use num_bigint::BigUint;
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
use std::str::FromStr;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,a:[BigUint;n]
    }
    /*
    let p = BigUint::from_str("6903360926176653850629667").unwrap();
    let q = BigUint::from_str("1975509243156370398211883").unwrap();
    let r = BigUint::from_str("4492262937390364429208381").unwrap();
    for m in vec![p, q, r] {
        let aa = a
            .iter()
            .map(|s| BigUint::from_str(s).unwrap() % &m)
            .collect::<Vec<_>>();
        let mut dict: HashMap<&BigUint, HashSet<usize>> = HashMap::new();
        for i in 0..n {
            let e = dict.entry(&aa[i]).or_insert(HashSet::new());
            e.insert(i);
        }
    }
    */
    let p = BigUint::from_str("3142571746368941").unwrap();
    let a = a
        .iter()
        .map(|x| (x % &p).to_u128().unwrap())
        .collect::<Vec<u128>>();
    let p = p.to_u128().unwrap();
    let mut cnt = HashMap::new();
    for i in 0..n {
        let e = cnt.entry(a[i]).or_insert(0usize);
        *e += 1;
    }
    let mut ans = 0usize;
    for i in 0..n {
        for j in 0..n {
            let c = if i == j { 1 } else { 2 };
            if let Some(x) = cnt.get(&((a[i] * a[j]) % p)) {
                ans += x;
            }
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
