#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
use num_iter::Range;
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
use std::{
    convert::{From, Into},
    ops::RangeBounds,
    process::Output,
};
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
/*
struct RangeAdditiveBitIndexedTree<T>
where
    T: Clone + std::ops::AddAssign<T>,
{
    b1: ac_library::FenwickTree<T>,
    b2: ac_library::FenwickTree<T>,
}

impl<T> RangeAdditiveBitIndexedTree<T>
where
    T: Clone
        + std::ops::AddAssign<T>
        + FromPrimitive
        + std::ops::Mul<Output = T>
        + std::ops::Neg<Output = T>
        + std::ops::Sub<Output = T>,
{
    pub fn new(n: usize, intializer: T) -> Self {
        let b1 = ac_library::FenwickTree::new(n, intializer.clone());
        let b2 = ac_library::FenwickTree::new(n, intializer.clone());
        RangeAdditiveBitIndexedTree { b1, b2 }
    }
    //半開区間[l,r)で加算
    pub fn add(&mut self, l: usize, r: usize, val: T) {
        self.b1.add(
            l,
            val.clone() * (T::from_usize(l).unwrap() - T::from_usize(1).unwrap()),
        );
        self.b1.add(r + 1, val.clone());
        self.b2.add(l, val.clone() * T::from_usize(r).unwrap());
        self.b2.add(r + 1, -val);
    }
    pub fn sum<R>(&self, range: R) -> T
    where
        R: RangeBounds<usize>,
    {
    }
    pub fn get(&mut self,i:usize)->T{
        b1.sum(0..(i + 1)) + b2.sum(0..(i + 1)) * (i + 1) as i64
                - (b1.sum(0..i) + b2.sum(0..i) * i as i64)
    }
}
*/
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,a:[i64;n],b:[usize;m]
    }
    let lazy_st=ac_library::LazySegtree<ac_library::Max<usize>>::new(n);
}
fn main() {
    solve();
}
