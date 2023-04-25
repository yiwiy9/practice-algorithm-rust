use proconio::input;

fn main() {
    input! {
        n: i32,
        y: i32,
    }

    for i in 0..=n {
        for j in 0..=n {
            if i + j > n {
                break;
            }

            let k = n - (i + j);
            if 10000 * i + 5000 * j + 1000 * k == y {
                println!("{} {} {}", i, j, k);
                return;
            }
        }
    }

    println!("-1 -1 -1");
}
