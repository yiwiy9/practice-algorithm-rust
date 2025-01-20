use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        b: usize,
        h: [usize; n],
    }

    let mut ok = true;
    let mut cur_s = s;
    let mut cur_h = h[0];
    for i in 1..n {
        if cur_h > h[i] {
            continue;
        }

        let diff = h[i] - cur_h;
        if cur_s * b < diff {
            ok = false;
            break;
        }

        cur_s = s;
        cur_h = h[i];
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
