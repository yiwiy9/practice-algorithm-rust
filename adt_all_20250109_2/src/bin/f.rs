use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        p: usize,
        mut f: [usize; n],
    }

    f.sort_by(|a, b| b.cmp(a));

    let mut ans = 0;
    let mut cur_y = 0;
    let mut cur_d = 0;
    for &f_i in &f {
        cur_y += f_i;
        cur_d += 1;
        if cur_d < d {
            continue;
        }

        ans += cur_y.min(p);
        cur_y = 0;
        cur_d = 0;
    }

    ans += cur_y.min(p);

    println!("{}", ans);
}
