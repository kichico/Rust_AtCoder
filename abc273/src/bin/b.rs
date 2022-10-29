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
        mut num:String,K:usize
    }
    let n = num.len();
    if K > n {
        println!("0");
        return;
    }
    for i in 0..K {
        let mut now = num[..num.len() - i - 1].to_string();
        for _ in 0..=i {
            now += &"0".to_string();
        }
        let border = 10i64.pow(i as u32 + 1);
        //println!("{} {}", border, now);
        let rest: i64 = num[num.len() - 1 - i..].parse().unwrap();
        let rest = if rest >= border / 2 { border } else { 0 };
        //println!("rest : {}", rest);
        let now: i64 = now.parse().unwrap();
        let now = now + rest;
        num = now.to_string();
        //println!("num : {}", num);
    }
    println!("{}", num);
}

fn main() {
    solve();
}
