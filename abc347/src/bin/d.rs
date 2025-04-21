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
#[allow(unused_imports)]
use std::ops::Bound::{Excluded, Included, Unbounded};

#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn to_binary<T>(x: T, keta: usize) -> Vec<char>
where
    T: std::fmt::Display + std::fmt::Binary,
{
    return format!("{:0width$b}", x, width = keta)
        .chars()
        .collect::<Vec<char>>();
}

fn from_binary<T>(x: &Vec<char>) -> T
where
    T: std::ops::AddAssign + num_traits::FromPrimitive,
{
    let mut ret = T::from_usize(0usize).unwrap();
    for i in 0..x.len() {
        if x[i] == '1' {
            let val = 2usize.pow((x.len() - i - 1) as u32);
            ret += T::from_usize(val).unwrap();
        }
    }
    return ret;
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        a:u32,b:u32,c:i128
    }
    let c_ones = c.count_ones();
    let cb = to_binary(c, 60);
    for a_ones in 0..=c_ones {
        let b_ones = c_ones - a_ones;
        if a < a_ones || b < b_ones {
            continue;
        }
        if a - a_ones == b - b_ones {
            let standing = cb
                .iter()
                .enumerate()
                .filter(|(x, y)| **y == '1')
                .map(|(x, y)| cb.len() - x - 1)
                .collect::<Vec<_>>();
            let mut x = vec!['0'; 60];
            let mut y = vec!['0'; 60];
            for i in 0..a_ones {
                x[standing[i as usize]] = '1';
            }
            for i in a_ones..c_ones {
                y[standing[i as usize]] = '1';
            }
            let st: HashSet<usize> = HashSet::from_iter(standing.into_iter());
            let mut rest = a - a_ones;
            for i in 0..60 {
                if st.contains(&i) {
                    continue;
                }
                if rest == 0 {
                    break;
                }
                x[i] = '1';
                y[i] = '1';
                rest -= 1;
            }
            x.reverse();
            y.reverse();
            let x: usize = from_binary(&x);
            let y: usize = from_binary(&y);
            if x.count_ones() == a && y.count_ones() == b && x ^ y == c as usize {
                println!("{} {}", x, y);
                return;
            }
        }
    }
    println!("-1");
}
fn main() {
    solve();
}
