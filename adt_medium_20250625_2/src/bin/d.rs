use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
    }

    for _ in 0..q {
        input! {
            op: u8,
        }
        match op {
            1 => {
                input! {
                    k: Usize1,
                    x: usize,
                }
                a[k] = x;
            }
            2 => {
                input! {
                    k: Usize1
                }
                println!("{}", a[k]);
            }
            _ => unreachable!(),
        }
    }
}
