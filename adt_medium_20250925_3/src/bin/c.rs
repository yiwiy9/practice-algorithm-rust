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

    let mut j = 0;
    for i in 0..n - 1 {
        while j < m && b[j] < a[i] {
            j += 1;
        }

        if j == m || a[i + 1] < b[j] {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
