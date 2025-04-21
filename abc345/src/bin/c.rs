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
use std::mem::take;
#[allow(unused_imports)]
use std::ops::Bound::{Excluded, Included, Unbounded};

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
    let mut dic: BTreeMap<char, usize> = BTreeMap::new();
    for i in 0..s.len() {
        *dic.entry(s[i]).or_insert(0) += 1;
    }
    let mut ans = 0;
    let dic2 = dic.clone();
    for (k, v) in dic2 {
        if v >= 2 {
            ans = 1;
            break;
        }
    }
    for i in 0..s.len() - 1 {
        for alpha in 0..26 {
            let c = ('a' as u8 + alpha) as char;
            if c == s[i] {
                continue;
            }
            if let Some(x) = dic.get(&c) {
                ans += x;
            }
        }
        let v = dic.get_mut(&s[i]).unwrap();
        *v -= 1;
    }
    println!("{}", ans.max(1));
}

fn local() {
    let s = vec!['a', 'a', 'a', 'b', 'b'];
    let mut ans: BTreeSet<Vec<char>> = BTreeSet::new();
    //ans.insert(s.clone());
    for i in 0..s.len() - 1 {
        for j in i + 1..s.len() {
            let v = &mut s.clone();
            let t = take(&mut v[i]);
            v[i] = v[j];
            v[j] = t;
            ans.insert(v.clone());
        }
    }
    println!("{}", ans.len());
    for v in ans {
        println!("{}", v.iter().join(""));
    }
}
fn main() {
    //local();
    solve();
}
