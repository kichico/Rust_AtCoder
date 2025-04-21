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
        r:usize
    }
    let mut farest_x = r - 1;
    let mut ans = 0;
    for farest_y in 2..=r {
        while farest_x > 0
            && (farest_x as f64 + 0.5) * (farest_x as f64 + 0.5)
                + (farest_y as f64 - 0.5) * (farest_y as f64 - 0.5)
                > (r * r) as f64
        {
            //println!("x:{}", (farest_x as f64 + 0.5) * (farest_x as f64 + 0.5));
            //println!("y:{}", (farest_y as f64 - 0.5) * (farest_y as f64 - 0.5));
            farest_x -= 1;
        }
        //println!("-------");
        //println!("farest_y:{} farest_x:{}", farest_y, farest_x);
        ans += farest_x;
    }
    ans *= 4;
    ans += r * 4 - 3;
    println!("{}", ans);
}
fn main() {
    solve();
}
