#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use petgraph::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
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
        n:usize,s:Chars,t:Chars
    }
    let mut cnt = 0;
    let mut s_zero = 0;
    let mut t_zero = 0;
    let mut incomp: HashSet<usize> = HashSet::new();
    for i in 0..n {
        if s[i] != t[i] {
            cnt += 1;
            incomp.insert(i);
            if s[i] == '0' {
                s_zero += 1;
            }
            if t[i] == '0' {
                t_zero += 1;
            }
        }
    }
    if cnt % 2 != 0 {
        println!("-1");
        return;
    } else if t_zero == s_zero {
        for _ in 0..n {
            print!("0");
        }
        println!();
        return;
    }
    let mut ans: Vec<char> = Vec::new();
    let mut t_same = 0;
    let mut s_same = 0;
    for i in 0..n {
        if incomp.contains(&i) {
            if s_same < incomp.len() / 2 && t_same < incomp.len() / 2 {
                ans.push('0');
                if ans[i] == s[i] {
                    s_same += 1;
                } else {
                    t_same += 1;
                }
            } else if s_same < incomp.len() / 2 {
                ans.push(s[i]);
            } else {
                ans.push(t[i]);
            }
        } else {
            ans.push('0');
        }
    }
    println!("{}", ans.iter().join(""));
}

fn main() {
    solve();
}
