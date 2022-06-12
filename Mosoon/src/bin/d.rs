#[allow(unused_imports)]
use itertools::Itertools;
use libm::floor;
use num_integer::Roots;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
fn to_Binary(mut x: i64) -> Vec<char> {
    let mut v: Vec<char> = Vec::new();
    if x == 0 {
        v.push('0');
    }
    while x > 0 {
        let c = std::char::from_digit((x % 2) as u32, 10).unwrap();
        v.push(c);
        x /= 2;
    }
    return v;
}
#[fastout]
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,
    }
    let mut swit: Vec<Vec<usize>> = vec![Vec::new(); m];
    for i in 0..m {
        input! {k:usize,s:[Usize1;k]}
        swit[i] = s;
    }
    let mut ans = 0;
    input! {p:[i64;m]}
    for i in 0..2i64.pow(10) {
        let mut bi = to_Binary(i);
        let mut light: Vec<i64> = vec![0; n];
        while bi.len() < n {
            bi.push('0');
        }
        bi.reverse();
        let bi = bi;
        if bi.len() > n {
            continue;
        }
        for j in 0..n {
            if bi[j] == '1' {
                light[j] += 1;
            }
        }
        let mut cnt = vec![0; m];
        for j in 0..m {
            for k in 0..swit[j].len() {
                if light[swit[j][k] as usize] >= 1 {
                    cnt[j] += 1;
                }
            }
        }
        let mut check: bool = true;
        for j in 0..m {
            if cnt[j] % 2 != p[j] {
                check = false;
            }
        }
        if check {
            ans += 1;
            for j in 0..m {
                let mut vec: Vec<i64> = Vec::new();
                for k in 0..swit[j].len() {
                    if light[swit[j][k] as usize] >= 1 {
                        vec.push(swit[j][k] as i64 + 1);
                    }
                }
            }
        }
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
