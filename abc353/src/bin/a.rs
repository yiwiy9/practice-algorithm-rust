use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut ans = -1;
    for i in 1..n {
        if h[i] > h[0] {
            ans = (i + 1) as i32;
            break;
        }
    }

    println!("{}", ans);
}
