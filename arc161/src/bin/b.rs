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
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
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
    v.reverse();
    return v;
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,s:[i64;n]
    }
    let mut ans: Vec<i64> = Vec::new();
    'outer: for i in 0..n {
        let v = s[i];
        if v < 7 {
            ans.push(-1);
            continue;
        }
        let mut b = to_Binary(v);
        //println!("{:?}", b);
        let stand = b.iter().filter(|x| **x == '1').count();
        if stand >= 3 {
            let mut val = 0;
            let mut stand = 0;
            for i in 0..b.len() {
                if b[i] == '1' {
                    stand += 1;
                    val += 2.pow((b.len() - i - 1) as u32);
                }
                if stand == 3 {
                    ans.push(val);
                    //println!("{}", val);
                    continue 'outer;
                }
            }
        } else if stand == 2 {
            let first = b.iter().rev().find_position(|x| **x == '1').unwrap().0;
            //println!("{}", b.len() - first - 1);
            if first < 2 {
                let v = vec!['1'; b.len() - 1];
                let mut val = 0;
                let mut stand = 0;
                for i in 0..v.len() {
                    if v[i] == '1' {
                        stand += 1;
                        val += 2.pow((v.len() - i - 1) as u32);
                    }
                    if stand == 3 {
                        ans.push(val);
                        continue 'outer;
                    }
                }
            } else {
                let p = b.len() - first - 1;
                b[p] = '0';
                b[p + 1] = '1';
                b[p + 2] = '1';
                let mut val = 0;
                let mut stand = 0;
                for i in 0..b.len() {
                    if b[i] == '1' {
                        stand += 1;
                        val += 2.pow((b.len() - i - 1) as u32);
                    }
                    if stand == 3 {
                        ans.push(val);
                        continue 'outer;
                    }
                }
            }
        } else {
            let v = vec!['1'; b.len() - 1];
            let mut val = 0;
            let mut stand = 0;
            for i in 0..v.len() {
                if v[i] == '1' {
                    stand += 1;
                    val += 2.pow((v.len() - i - 1) as u32);
                }
                if stand == 3 {
                    ans.push(val);
                    continue 'outer;
                }
            }
        }
    }
    println!("{}", ans.iter().join("\n"));
}
fn main() {
    solve();
}
