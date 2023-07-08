#[allow(unused_imports)]
use itertools::*;
use maplit::btreeset;
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

fn local(n: usize, s: Vec<char>) {
    let mut ans = 0;
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                if j - i == k - j {
                    continue;
                }
                let st = btreeset! {s[i],s[j],s[k]};
                if st.len() == 3 {
                    ans += 1;
                }
            }
        }
    }
    println!("local {}", ans);
}

fn solve() {
    input! {
        n:usize,s:Chars
    }
    let mut blue: Vec<usize> = Vec::new();
    let mut red = blue.clone();
    let mut green = blue.clone();
    for i in 0..n {
        if s[i] == 'B' {
            blue.push(i);
        } else if s[i] == 'R' {
            red.push(i);
        } else {
            green.push(i);
        }
    }
    let mut ans = 0;
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            let color1 = s[i];
            let color2 = s[j];
            if s[i] == s[j] {
                continue;
            }
            let d = j - i;
            if (color1 == 'R' && color2 == 'G') || (color2 == 'R' && color1 == 'G') {
                let mut v = blue.len() - blue.upper_bound(&j);
                if j + d < n && s[j + d] == 'B' {
                    v -= 1;
                }
                ans += v;
            } else if (color1 == 'R' && color2 == 'B') || (color2 == 'R' && color1 == 'B') {
                let mut v = green.len() - green.upper_bound(&j);
                if j + d < n && s[j + d] == 'G' {
                    v -= 1;
                }
                ans += v;
            } else {
                let mut v = red.len() - red.upper_bound(&j);
                if j + d < n && s[j + d] == 'R' {
                    v -= 1;
                }
                ans += v;
            }
        }
    }
    println!("{}", ans);
    //local(n, s);
}
fn main() {
    solve();
}
