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
        bingo:[[i64;3];3]
    }
    let order = (0..9).into_iter().permutations(9);
    let mut cnt = 0;
    let total = 362880 as f64;
    'prop: for v in order {
        let mut sheet = vec![vec![(0, -1); 3]; 3];
        'outer: for i in 0..9 {
            let r = v[i] / 3;
            let c = v[i] % 3;
            sheet[r][c] = (i, bingo[r][c]);
            let mut row: Vec<(usize, i64)> = Vec::new();
            for j in 0..3 {
                if sheet[r][j].1 == -1 {
                    break;
                }
                row.push(sheet[r][j]);
            }
            row.sort();
            if row.len() == 3 && row[0].1 == row[1].1 {
                //println!("first");
                continue 'prop;
            }
            row.clear();
            for j in 0..3 {
                if sheet[j][c].1 == -1 {
                    break;
                }
                row.push(sheet[j][c]);
            }
            row.sort();
            if row.len() == 3 && row[0].1 == row[1].1 {
                //println!("second");
                continue 'prop;
            }
            if r == c {
                row.clear();
                for j in 0..3 {
                    if sheet[j][j].1 == -1 {
                        break;
                    }
                    row.push(sheet[j][j]);
                }
                row.sort();
                if row.len() == 3 && row[0].1 == row[1].1 {
                    continue 'prop;
                }
            }
            if (r == 0 && c == 2) || (r == 2 && c == 0) || (r == 1 && c == 1) {
                row.clear();
                for j in 0..3 {
                    if sheet[j][2 - j].1 == -1 {
                        continue 'outer;
                    }
                    row.push(sheet[j][2 - j]);
                }
                row.sort();
                if row[0].1 == row[1].1 {
                    continue 'prop;
                }
            }
        }
        cnt += 1;
    }
    println!("{}", cnt as f64 / total);
}
fn main() {
    solve();
}
