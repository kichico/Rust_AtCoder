#[allow(unused_imports)]
use itertools::*;
use maplit::btreeset;
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

fn dfs(
    row: usize,
    field: &mut Vec<Vec<char>>,
    row_ans: &Vec<char>,
    col_ans: &Vec<char>,
    n: usize,
    flg: &mut bool,
) {
    if *flg == true {
        return;
    }
    if row == n {
        for c in 0..n {
            let mut checker: Vec<char> = Vec::new();
            for r in 0..n {
                if field[r][c] != '.' {
                    checker.push(field[r][c]);
                }
            }
            checker.sort();
            if checker.len() != 3 || checker[0] != 'A' || checker[1] != 'B' || checker[2] != 'C' {
                return;
            }
            let mut r = 0;
            while r < n && field[r][c] == '.' {
                r += 1;
            }
            if r >= n || col_ans[c] != field[r][c] {
                return;
            }
        }
        *flg = true;
        println!("Yes");
        for i in 0..n {
            println!("{}", field[i].iter().join(""));
        }
        return;
    }
    let mut rest = btreeset! {'A','B','C'};
    for start in 0..3 {
        rest.remove(&row_ans[row]);
        let it = (start + 1..n).into_iter().permutations(2);
        for v in it {
            let mut new_row = vec!['.'; n];
            new_row[start] = row_ans[row];
            let (first, second) = (v[0], v[1]);
            new_row[first] = *rest.first().unwrap();
            new_row[second] = *rest.last().unwrap();
            field.push(new_row.clone());
            dfs(row + 1, field, row_ans, col_ans, n, flg);
            field.pop();
        }
    }
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,r:Chars,c:Chars
    }
    let mut field: &mut Vec<Vec<char>> = &mut Vec::new();
    let mut flg = &mut false;
    dfs(0, field, &r, &c, n, flg);
    if !*flg {
        println!("No");
    }
}
fn main() {
    solve();
}
