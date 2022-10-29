use itertools::enumerate;
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
        s:String
    }
    if s == "atcoder" {
        println!("0");
        return;
    }
    let mut s: Vec<char> = s.chars().collect();
    let mut cnt = 0;
    let n = s.len();
    let refe: Vec<char> = "atcoder".chars().collect();
    for (i, c) in enumerate(refe) {
        let mut pos = 0;
        while s[pos] != c {
            pos += 1;
        }
        while pos != i {
            let fr = s[pos].clone();
            let se = s[pos - 1].clone();
            s[pos] = se;
            s[pos - 1] = fr;
            pos -= 1;
            cnt += 1;
        }
    }
    println!("{}", cnt);
}

fn main() {
    solve();
}
