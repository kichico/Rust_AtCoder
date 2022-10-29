#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use petgraph::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use proconio::{source::line::LineSource, *};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::io::{self, BufReader};
#[allow(unused_imports)]
use std::mem::swap;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
    input! {
        n:usize
    }
    let mut dict: HashMap<(usize, usize, usize, usize), i64> = HashMap::new();
    let (mut rleft, mut rright, mut cleft, mut cright) = (1, n, 1, n);
    while rright - rleft > 0 {
        let mid = rleft + (rright - rleft) / 2;
        println!("? {} {} {} {}", rleft, mid, cleft, cright);
        input! { upper:i64 }
        println!("? {} {} {} {}", mid + 1, rright, cleft, cright);
        input! { lower:i64 }
        if upper == -1 || lower == -1 {
            return;
        }
        dict.insert((rleft, mid, cleft, cright), upper);
        dict.insert((mid + 1, rright, cleft, cright), lower);
        if rright - rleft == 1 {
            if dict.get(&(rleft, mid, cleft, cright)).unwrap()
                < dict.get(&(mid + 1, rright, cleft, cright)).unwrap()
            {
                rleft = mid;
                break;
            } else {
                rleft = rright.clone();
                break;
            }
        }
        if upper < lower {
            rright = mid;
        } else if upper > lower {
            rleft = mid;
        } else if mid - rleft > rright - mid + 1 {
            rleft = mid;
        } else {
            rright = mid;
        }
    }
    let ans = (rleft.clone(), 0);
    let (rleft, rright) = (1, n);
    while cright - cleft > 0 {
        let mid = cleft + (cright - cleft) / 2;
        println!("? {} {} {} {}", rleft, rright, cleft, mid);
        input! { upper:i64 }
        println!("? {} {} {} {}", rleft, rright, mid + 1, cright);
        input! { lower:i64 }
        if upper == -1 || lower == -1 {
            return;
        }
        dict.insert((rleft, rright, cleft, mid), upper);
        dict.insert((rleft, rright, mid + 1, cright), lower);
        if cright - cleft == 1 {
            if dict.get(&(rleft, rright, cleft, mid)).unwrap()
                < dict.get(&(rleft, rright, mid + 1, cright)).unwrap()
            {
                cleft = mid;
                break;
            } else {
                cleft = cright.clone();
                break;
            }
        }
        if upper < lower {
            cright = mid;
        } else if upper > lower {
            cleft = mid;
        } else if mid - cleft > cright - mid + 1 {
            cleft = mid;
        } else {
            cright = mid;
        }
    }
    println!("! {} {}", ans.0, cleft);
}

fn main() {
    solve();
}
