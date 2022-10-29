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
        mut n:usize
    }
    let mut ans: Vec<char> = Vec::new();
    if n == 0 {
        println!("00");
        return;
    }
    let conv = vec!['A', 'B', 'C', 'D', 'E', 'F'];
    while n != 0 {
        let mut c = n % 16;
        if c < 10 {
            ans.push(to_char(c as i64));
        } else {
            c -= 10;
            ans.push(conv[c]);
        }
        n /= 16;
    }
    ans.reverse();
    if ans.len() == 1 {
        println!("0{}", ans[0]);
    } else {
        println!("{}{}", ans[0], ans[1]);
    }
}

fn main() {
    solve();
}
