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
        n:usize,q:usize,field:[Chars;n],query:[(usize,usize,usize,usize);q]
    }
    let mut row_cumsum = vec![vec![0; n]; n];
    let mut col_cumsum = row_cumsum.clone();
    for i in 0..n {
        let mut cnt = 0;
        for j in 0..n {
            if field[i][j] == 'B' {
                cnt += 1;
            }
            row_cumsum[i][j] = cnt;
        }
    }
    for j in 0..n {
        let mut cnt = 0;
        for i in 0..n {
            if field[i][j] == 'B' {
                cnt += 1;
            }
            col_cumsum[j][i] = cnt;
        }
    }
    let mut total = 0;
    for i in 0..n {
        total += field[i].iter().filter(|x| x == &&'B').count();
    }
    for (mut x0, mut y0, mut x1, mut y1) in query {
        let mut ans = 0;
        let xd = x1 - x0 + 1;
        let yd = y1 - y0 + 1;
        if xd > 2 * n && yd > 2 * n {
            ans += total * (xd / n + yd / n);
        }
        x0 %= n;
        y0 %= n;
        if x1 > n {
            let upper = if y1 > n { n - 1 } else { y1 };
            for row in y0..=upper {
                ans += row_cumsum[row][n - 1] - row_cumsum[row][x0];
            }
            let mut xn = 1;
            while (xn - 1) * n < x1 {
                xn += 1;
            }
            for row in y0..=upper {
                ans += (row_cumsum[row][n - 1] - row_cumsum[row][0]) * xn;
            }
            x1 %= n;
            for row in y0..=upper {
                ans += (row_cumsum[row][n - 1] - row_cumsum[row][0]) * xn;
            }
        }
        println!("{}", ans);
    }
}
fn main() {
    solve();
}
