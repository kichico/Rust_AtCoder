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
fn solve() {
    input! {
        s:Chars
    }
    let keta = s.len() - 1;
    let mut ans = 0;
    for bit in 0..(1 << keta) {
        let mut plus: Vec<usize> = Vec::new();
        for k in 0..s.len() {
            if (bit >> k) & 1 == 1 {
                plus.push(k);
            }
        }
        plus.reverse();
        let mut dbgger: Vec<usize> = Vec::new();
        let mut tmp: Vec<char> = Vec::new();
        for i in 0..s.len() {
            tmp.push(s[i]);
            if !plus.is_empty() && plus[plus.len() - 1] == i {
                let mut val = 0;
                for c in 0..tmp.len() {
                    val += (tmp[c] as usize - 48) * 10.pow((tmp.len() - c - 1) as u32);
                }
                ans += val;
                dbgger.push(val);
                tmp.clear();
                plus.pop();
            }
        }
        if !tmp.is_empty() {
            let val = tmp
                .iter()
                .enumerate()
                .map(|x| (*x.1 as usize - 48) * 10.pow((tmp.len() - x.0 - 1) as u32))
                .sum::<usize>();
            ans += val;

            dbgger.push(val);
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
