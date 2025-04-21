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
fn dfs(k: &usize, v: &mut Vec<usize>, keta: usize, cnt: &mut usize) -> usize {
    if v.len() == keta {
        *cnt += 1;
        if cnt == k {
            println!("{}", v.iter().join(""));
            return *cnt;
        }
        return *cnt;
    }
    let lv = v.iter().last().unwrap().clone();
    //println!("v:{}", v.iter().join(" "));
    for i in 0..lv {
        v.push(i);
        dfs(k, v, keta, cnt);
        v.pop();
    }
    return *cnt;
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        mut k:usize
    }
    let cnt = &mut 0;
    for keta in 1..=10 {
        for i in 1..10 {
            let v = &mut vec![i];
            *cnt = dfs(&k, v, keta, cnt);
            if *cnt > k {
                return;
            }
        }
    }
}

fn local(k: usize) {
    if k <= 9 {
        println!("{}", k);
        return;
    }
    let mut cnt = 0;
    'outer: for i in 1..1e9 as usize {
        let s = i
            .to_string()
            .chars()
            .map(|x| x as usize - '0' as usize)
            .collect::<Vec<usize>>();
        for j in 1..s.len() {
            if s[j - 1] <= s[j] {
                continue 'outer;
            }
        }
        println!("{} {}", s.iter().join(""), cnt);
        cnt += 1;
        if cnt == k {
            println!("ans:{}", s.iter().join(""));
            return;
        }
    }
}
fn main() {
    solve();
    //local(10);
}
