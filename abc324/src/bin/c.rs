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
        n:usize,t:Chars,ss:[Chars;n]
    }
    let mut ans: Vec<usize> = Vec::new();
    let le = &t.len();
    'outer: for i in 0..n {
        let s = &ss[i];
        let mut cnt = 0;
        if s.len().abs_diff(*le) > 1 {
            continue;
        }
        if s.len() == *le {
            for j in 0..*le {
                if s[j] != t[j] {
                    cnt += 1;
                }
            }
            if cnt <= 1 {
                ans.push(i + 1);
            }
        } else if s.len() > *le {
            let mut rec = s.len() + 1;
            for j in 0..*le {
                if s[j] != t[j] {
                    rec = j;
                    break;
                }
            }
            if rec == s.len() + 1 {
                ans.push(i + 1);
            } else {
                for j in rec..*le {
                    if s[j + 1] != t[j] {
                        continue 'outer;
                    }
                }
                ans.push(i + 1);
            }
        } else {
            let mut rec = *le + 1;
            for j in 0..s.len() {
                if s[j] != t[j] {
                    rec = j;
                    break;
                }
            }
            if rec == *le + 1 {
                ans.push(i + 1);
            } else {
                for j in rec..s.len() {
                    if s[j] != t[j + 1] {
                        continue 'outer;
                    }
                }
                ans.push(i + 1);
            }
        }
    }
    println!("{}\n{}", ans.len(), ans.iter().join(" "));
}
fn main() {
    solve();
}
