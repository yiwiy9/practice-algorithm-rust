use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        d: [usize; n-1],
    }

    let mut pos = vec![0; l];

    pos[0] = 1;
    let mut cur = 0;
    for i in 0..n - 1 {
        cur += d[i];
        cur %= l;
        pos[cur] += 1;
    }

    if l % 3 != 0 {
        println!("0");
        return;
    }

    let mut ans = 0_usize;
    for p in 0..l / 3 {
        if pos[p] > 0 && pos[p + l / 3] > 0 && pos[p + 2 * l / 3] > 0 {
            ans += pos[p] * pos[p + l / 3] * pos[p + 2 * l / 3];
        }
    }

    println!("{}", ans);
}
