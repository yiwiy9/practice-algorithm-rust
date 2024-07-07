use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
        x: [[usize; m]; n],
    }

    let mut sum = vec![0; m];
    for i in 0..n {
        for j in 0..m {
            sum[j] += x[i][j];
        }
    }

    let mut ok = true;
    for j in 0..m {
        if sum[j] < a[j] {
            ok = false;
            break;
        }
    }

    println!("{}", if ok { "Yes" } else { "No"});
}
