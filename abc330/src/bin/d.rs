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
use std::ops::Bound::{Excluded, Included, Unbounded};

#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

fn local(n: usize, field: Vec<Vec<char>>) -> usize {
    let mut cnt = 0;
    for first in 0..n * n {
        for second in first + 1..n * n {
            let fr = first / n;
            let fc = first % n;
            let sr = second / n;
            let sc = second % n;
            if field[fr][fc] != 'o' || field[sr][sc] != 'o' {
                continue;
            }
            if fr != sr && fc != sc {
                if field[fr][sc] == 'o' {
                    cnt += 1;
                }
                if field[sr][fc] == 'o' {
                    cnt += 1;
                }
            }
        }
    }
    return cnt;
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,field:[Chars;n]
    }
    let mut row_maru: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); n];
    let mut col_maru: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); n];
    for row in 0..n {
        for col in 0..n {
            if field[row][col] == 'o' {
                row_maru[row].insert(col);
                col_maru[col].insert(row);
            }
        }
    }
    let mut ans = 0;
    for first in 0..n * n {
        let row = first / n;
        let col = first % n;
        if field[row][col] != 'o' {
            continue;
        }
        //println!("row:{} col:{}", row, col);
        //println!("{}", row_maru[row].len());
        //println!("{}", col_maru[col].len());
        let r = row_maru[row].len() - 1;
        let c = col_maru[col].len() - 1;
        ans += r * c;
    }
    println!("{}", ans);
    //println!("local:{}", local(n, field));
}
fn main() {
    solve();
}
