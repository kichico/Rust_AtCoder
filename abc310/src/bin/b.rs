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
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize
    }
    let mut fac: Vec<Vec<i64>> = vec![Vec::new(); n];
    let mut ps = vec![0i64; n];
    for i in 0..n {
        input! {p:i64,c:usize,f:[i64;c]}
        ps[i] = p;
        fac[i] = f;
    }
    for i in 0..n {
        'outer: for j in 0..n {
            if ps[i] < ps[j] || i == j {
                continue;
            }
            let fac_a: HashSet<i64> = HashSet::from_iter(fac[i].clone().into_iter());
            let mut fac_b: HashSet<i64> = HashSet::from_iter(fac[j].clone().into_iter());
            if fac_a.len() > fac_b.len() {
                continue;
            }
            for kinou in fac_a {
                if !fac_b.contains(&kinou) {
                    continue 'outer;
                }
                fac_b.remove(&kinou);
            }
            if fac_b.len() > 0 || ps[i] > ps[j] {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
fn main() {
    solve();
}
