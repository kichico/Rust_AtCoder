use proconio::input;

fn solve() {
    input! {
        n: i64,
        m: i64,
        mut a: [i64;n],
    }
    let mut ans: i64 = 0;
    if a[0] > m {
        ans += a[0] - m;
        a[0] = m;
    }
    for i in 1..n as usize {
        if a[i - 1] + a[i] > m {
            ans += a[i] + a[i - 1] - m;
            a[i] = m - a[i - 1];
        }
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
