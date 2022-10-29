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
        h1:usize,w1:usize,a:[[i64;w1];h1],h2:usize,w2:usize,b:[[i64;w2];h2]
    }
    if h1 < h2 || w1 < w2 {
        println!("No");
        return;
    }
    let h_iter = (0..h1).combinations(h2);
    for x in h_iter {
        let w_iter = (0..w1).combinations(w2);
        for y in w_iter {
            let mut flg = true;
            for i in 0..x.len() {
                for j in 0..y.len() {
                    if a[x[i]][y[j]] != b[i][j] {
                        flg = false;
                        break;
                    }
                    if !flg {
                        break;
                    }
                }
            }
            if flg {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

fn main() {
    solve();
}
