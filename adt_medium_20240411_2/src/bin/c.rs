use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }

    a.sort();
    b.sort();

    let mut i = 0;
    for b_j in b {
        while i < n && a[i] != b_j {
            i += 1;
        }
        if i == n {
            println!("No");
            return;
        }
        i += 1;
    }

    println!("Yes");
}
