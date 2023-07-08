#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
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
use superslice::Ext;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        w:usize,h:usize,n:usize,cake:[(usize,usize);n],a:usize,ver_line:[usize;a],b:usize,mut hori_line:[usize;b]
    }
    let mut xpos = cake.iter().cloned().map(|x| x.0).collect::<Vec<usize>>();
    let mut ypos = cake.iter().cloned().map(|x| x.1).collect::<Vec<usize>>();
    xpos.sort();
    ypos.sort();
    let mut v = vec![0];
    for i in 0..a {
        v.push(ver_line[i]);
    }
    v.push(w);
    let ver_line = v;
    let mut xnum: Vec<usize> = vec![0; a + 1];
    for i in 1..a + 1 {
        let lower = xpos.lower_bound(&ver_line[i - 1]);
        let upper = xpos.upper_bound(&ver_line[i]);
        xnum[i] = upper - lower;
    }
    ///////////////////////////////
    let mut maxi = 0;
    let mut maxi_pos = 0;
    let mut mini = n + 1;
    let mut mini_pos = n + 1;
    for i in 0..xnum.len() {
        if xnum[i] > maxi {
            maxi = xnum[i];
            maxi_pos = i;
        }
        if xnum[i] < mini {
            mini = xnum[i];
            mini_pos = i;
        }
    }
    let mut hit = cake
        .iter()
        .cloned()
        .filter(|x| ver_line[maxi_pos] <= x.0 && x.0 <= ver_line[maxi_pos + 1])
        .map(|x| x.1)
        .collect::<Vec<_>>();
    hit.sort();
    let mut v = vec![0];
    for i in 0..b {
        v.push(hori_line[i]);
    }
    v.push(h);
    let hori_line = v;
    let mut maxi = 0;
    for i in 1..b + 1 {
        let lower = hit.lower_bound(&hori_line[i - 1]);
        let upper = hit.upper_bound(&hori_line[i]);
        maxi = maxi.max(upper - lower);
    }
    ////////////////
    let mut hit = cake
        .iter()
        .cloned()
        .filter(|x| ver_line[mini_pos] <= x.0 && x.0 <= ver_line[mini_pos + 1])
        .map(|x| x.1)
        .collect::<Vec<_>>();
    hit.sort();
    let mut mini = n + 1;
    for i in 1..b + 1 {
        let lower = hit.lower_bound(&hori_line[i - 1]);
        let upper = hit.upper_bound(&hori_line[i]);
        mini = mini.min(upper - lower);
    }
    println!("{} {}", mini, maxi);
}
fn main() {
    solve();
}
