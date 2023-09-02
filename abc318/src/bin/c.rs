use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        p: usize,
        mut f: [usize; n],
    }

    f.sort_by(|&a, &b| b.cmp(&a));

    let mut ans = 0;
    let mut cur = 0;
    let mut days = 0;
    for &f_i in &f {
        cur += f_i;
        days += 1;

        if days == d {
            ans += cur.min(p);
            cur = 0;
            days = 0;
        }
    }

    ans += cur.min(p);

    println!("{}", ans);
}
