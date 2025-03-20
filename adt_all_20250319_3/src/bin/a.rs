use proconio::input;

fn main() {
    input! {
        a: [usize; 5],
    }

    let mut b = a.clone();
    b.sort();

    let mut cnt = 0;
    let mut i = 0;
    while i < 4 {
        if a[i] != b[i] {
            if a[i + 1] == b[i] {
                cnt += 1;
                i += 1;
            } else {
                println!("No");
                return;
            }
        }
        i += 1;
    }

    println!("{}", if cnt == 1 { "Yes" } else { "No" });
}
