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
        n:usize,s:String
    }
    let s: Vec<char> = s.chars().collect();
    if n == 2 && s[0] != s[1] {
        println!("No");
        return;
    }
    let mut flg = true;
    for i in 0..n {
        let j = n - i - 1;
        if s[i] != s[j] {
            flg = false;
            break;
        }
    }
    if flg {
        println!("Yes");
        return;
    }
    if s[0] == 'A' && *s.iter().next_back().unwrap() == 'B' {
        println!("No");
        return;
    }
    println!("Yes");
}

fn main() {
    solve();
}
