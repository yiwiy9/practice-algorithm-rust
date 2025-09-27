use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }

    let mut front = 0;
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    c: usize,
                }
                front += c;
                front %= n;
            }
            2 => {
                input! {
                    l: usize,
                    r: usize,
                }
                let l = (l + front - 1) % n;
                let r = (r + front - 1) % n;
                if l <= r {
                    println!("{}", s[r + 1] - s[l]);
                } else {
                    println!("{}", s[n] - s[l] + s[r + 1]);
                }
            }
            _ => unreachable!(),
        }
    }
}
