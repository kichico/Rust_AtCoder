#[allow(unused_imports)]
use itertools::*;
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
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        point:[(i64,i64);4]
    }
    let points = vec![point.clone(), point.clone(), point.clone()].concat();
    let mut args = vec![0f64; 4];
    for i in 0..4 {
        let p = i + 4;
        let a = points[p - 1];
        let b = points[p];
        let c = points[p + 1];
        let va = (a.0 - b.0, a.1 - b.1);
        let vb = (c.0 - b.0, c.1 - b.1);
        let cos_theta = (va.0 * vb.0 + va.1 * vb.1) as f64
            / (((va.0.pow(2) + va.1.pow(2)) as f64).sqrt()
                * ((vb.0.pow(2) + vb.1.pow(2)) as f64).sqrt());
        let theta = cos_theta.acos();
        args[i] = theta * 180f64 / std::f64::consts::PI;
    }
    for i in 0..1 << 4 {
        let mut current = vec![0f64; 4];
        for j in 0..4 {
            if (i >> j) & 1 == 1 {
                current[j] = args[j];
            } else {
                current[j] = 360f64 - args[j];
            }
        }
        if (360f64 - current.iter().sum::<f64>()).abs() < 1e-5 {
            for j in 0..4 {
                if current[j] - 180f64 + 1e-6 > 0f64 {
                    println!("No");
                    return;
                }
            }
            println!("Yes");
            return;
        }
    }
}
fn main() {
    solve();
}
