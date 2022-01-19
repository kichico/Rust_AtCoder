#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        n: usize,
        mut a: [i64;n],
        mut b: [i64;n],
        mut c: [i64;n],
    }
    for i in 0..n {
        a[i] %= 46;
        b[i] %= 46;
        c[i] %= 46;
    }
    let (mut mpa, mut mpb, mut mpc): (HashMap<i64, i64>, HashMap<i64, i64>, HashMap<i64, i64>) =
        (HashMap::new(), HashMap::new(), HashMap::new());
    for i in 0..47 {
        mpa.insert(i, 0);
        mpb.insert(i, 0);
        mpc.insert(i, 0);
    }
    for i in 0..n {
        let pt = mpa.entry(a[i]).or_insert(0);
        *pt += 1;
        let pt = mpb.entry(b[i]).or_insert(0);
        *pt += 1;
        let pt = mpc.entry(c[i]).or_insert(0);
        *pt += 1;
    }
    let mut ans = 0;
    for i in 0..47 {
        for j in 0..47 {
            for k in 0..47 {
                if (i + j + k) % 46 != 0 {
                    continue;
                }
                let an = mpa.get(&i).unwrap();
                let bn = mpb.get(&j).unwrap();
                let cn = mpc.get(&k).unwrap();
                if an != &0 && bn != &0 && cn != &0 {
                    ans += an * bn * cn;
                }
            }
        }
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
