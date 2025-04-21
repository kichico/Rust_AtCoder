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
use std::process::exit;

#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

fn dfs(
    field: &mut Vec<Vec<usize>>,
    blocks: &Vec<&(usize, usize)>,
    idx: usize,
    r: usize,
    c: usize,
    rot: bool,
) {
    let h = field.len();
    let w = field[0].len();
    let (mut bw, mut bh) = blocks[idx].clone();
    if rot {
        swap(&mut bw, &mut bh);
    }
    for row in r..r + bh {
        for col in c..c + bw {
            if row >= h || col >= w {
                println!("r:{} c:{} idx:{}", r, c, idx);
                println!("bw:{} bh:{}", bw, bh);
                println!("{:?}", blocks);
                panic!();
            }
            field[row][col] = 1;
        }
    }
    if idx + 1 == blocks.len() {
        let mut is_completed = true;
        'outer: for i in 0..field.len() {
            for j in 0..field[i].len() {
                if field[i][j] == 0 {
                    is_completed = false;
                    break 'outer;
                }
            }
        }
        if is_completed {
            println!("Yes");
            exit(0);
        } else {
            for row in r..r + bh {
                for col in c..c + bw {
                    field[row][col] = 0;
                }
            }
            return;
        }
    }

    for i in 0..h {
        'position: for j in 0..w {
            let (mut bw, mut bh) = blocks[idx + 1].clone();
            if field[i][j] == 1 {
                continue;
            }
            let mut is_ok = true;
            'outer: for row in i..i + bh {
                if row >= h {
                    is_ok = false;
                    break 'outer;
                }
                for col in j..j + bw {
                    if col >= w {
                        is_ok = false;
                        break 'outer;
                    }
                    if field[row][col] == 1 {
                        is_ok = false;
                        break 'outer;
                    }
                }
            }
            if is_ok {
                dfs(field, blocks, idx + 1, i, j, false);
            }
            swap(&mut bw, &mut bh);
            for row in i..i + bh {
                if row >= h {
                    continue 'position;
                }
                for col in j..j + bw {
                    if col >= w {
                        continue 'position;
                    }
                    if field[row][col] == 1 {
                        continue 'position;
                    }
                }
            }
            dfs(field, blocks, idx + 1, i, j, true);
        }
    }
    let (mut bw, mut bh) = blocks[idx].clone();
    if rot {
        swap(&mut bw, &mut bh);
    }
    for row in r..r + bh {
        for col in c..c + bw {
            field[row][col] = 0;
        }
    }
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,h:usize,w:usize,blocks:[(usize,usize);n]
    }
    let field = &mut vec![vec![0; w]; h];

    for k in 1..=n {
        for bb in blocks.iter().combinations(k) {
            for i in 0..h {
                for j in 0..w {
                    let (mut bw, mut bh) = bb[0].clone();
                    let mut is_ok = true;
                    'outer: for row in i..i + bh {
                        if row >= h {
                            is_ok = false;
                            break 'outer;
                        }
                        for col in j..j + bw {
                            if col >= w {
                                is_ok = false;
                                break 'outer;
                            }
                            if field[row][col] == 1 {
                                is_ok = false;
                                break 'outer;
                            }
                        }
                    }
                    if is_ok {
                        dfs(field, &bb, 0, i, j, false);
                    }
                    swap(&mut bw, &mut bh);

                    let mut is_ok = true;
                    'outer: for row in i..i + bh {
                        if row >= h {
                            is_ok = false;
                            break 'outer;
                        }
                        for col in j..j + bw {
                            if col >= w {
                                is_ok = false;
                                break 'outer;
                            }
                            if field[row][col] == 1 {
                                is_ok = false;
                                break 'outer;
                            }
                        }
                    }
                    if is_ok {
                        dfs(field, &bb, 0, i, j, true);
                    }
                }
            }
        }
    }

    println!("No");
}
fn main() {
    solve();
}
