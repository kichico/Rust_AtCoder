#[allow(unused_imports)]
use itertools::*;
use maplit::hashset;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use proconio::{
    input,
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
#[allow(unused_imports)]
use std::ops::Bound::{Excluded, Included, Unbounded};
#[allow(unused_imports)]
use superslice::*;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,q:usize,mut s:Chars,query:[(Usize1,char);q]
    }
    let mut ans = 0;
    for i in 0..n - 2 {
        let t = &s[i..i + 3].iter().join("");
        if t == &"ABC" {
            ans += 1;
        }
    }
    let ok = hashset! {'A','B','C'};
    for (x, c) in query {
        if c == s[x] {
            println!("{}", ans);
            continue;
        }
        if ok.contains(&c) {
            if !ok.contains(&s[x]) {
                if c == 'A' {
                    if x >= n - 2 {
                        s[x] = c;
                        println!("{}", ans);
                        continue;
                    }
                    let t = &s[x + 1..x + 3].iter().join("");
                    if t == &"BC" {
                        ans += 1;
                    }
                } else if c == 'B' {
                    if x == n - 1 || x == 0 {
                        s[x] = c;

                        println!("{}", ans);
                        continue;
                    }
                    let t = vec![s[x - 1], s[x + 1]].iter().join("");
                    if t == "AC" {
                        ans += 1;
                    }
                } else {
                    if x <= 1 {
                        s[x] = c;

                        println!("{}", ans);
                        continue;
                    }
                    let t = vec![s[x - 2], s[x - 1]].iter().join("");
                    if t == "AB" {
                        ans += 1;
                    }
                }
            } else {
                if s[x] == 'A' && x <= n - 3 && &s[x..x + 3].iter().join("") == &"ABC" {
                    ans -= 1;
                } else if s[x] == 'B'
                    && x <= n - 2
                    && x >= 1
                    && &s[x - 1..x + 2].iter().join("") == &"ABC"
                {
                    ans -= 1;
                } else if s[x] == 'C' && x >= 2 && &s[x - 2..x + 1].iter().join("") == &"ABC" {
                    ans -= 1;
                }
                if c == 'A' {
                    if x >= n - 2 {
                        s[x] = c;

                        println!("{}", ans);
                        continue;
                    }
                    let t = &s[x + 1..x + 3].iter().join("");
                    if t == &"BC" {
                        ans += 1;
                    }
                } else if c == 'B' {
                    if x == n - 1 || x == 0 {
                        s[x] = c;

                        println!("{}", ans);
                        continue;
                    }
                    let t = vec![s[x - 1], s[x + 1]].iter().join("");
                    if t == "AC" {
                        ans += 1;
                    }
                } else {
                    if x <= 1 {
                        s[x] = c;
                        println!("{}", ans);
                        continue;
                    }
                    let t = vec![s[x - 2], s[x - 1]].iter().join("");
                    if t == "AB" {
                        ans += 1;
                    }
                }
            }
        } else {
            if s[x] == 'A' && x <= n - 3 && &s[x..x + 3].iter().join("") == &"ABC" {
                ans -= 1;
            } else if s[x] == 'B'
                && x <= n - 2
                && x >= 1
                && &s[x - 1..x + 2].iter().join("") == &"ABC"
            {
                ans -= 1;
            } else if s[x] == 'C' && x >= 2 && &s[x - 2..x + 1].iter().join("") == &"ABC" {
                ans -= 1;
            }
        }
        s[x] = c;
        println!("{}", ans);
    }
}
fn main() {
    solve();
}
