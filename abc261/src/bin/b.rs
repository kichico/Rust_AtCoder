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
use std::f32::consts::E;
#[allow(unused_imports)]
use std::mem::swap;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        n:usize,t:[String;n]
    }
    let mut table: Vec<Vec<char>> = vec![Vec::new(); n];
    for i in 0..n {
        let c: Vec<char> = t[i].chars().collect();
        table[i] = c;
    }
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            if (table[i][j] == 'W' && table[j][i] == 'L')
                || (table[i][j] == 'L' && table[j][i] == 'W')
                || (table[i][j] == 'D' && table[j][i] == 'D')
            {
                continue;
            } else {
                println!("incorrect");
                return;
            }
        }
    }
    println!("correct");
}

fn main() {
    solve();
}
