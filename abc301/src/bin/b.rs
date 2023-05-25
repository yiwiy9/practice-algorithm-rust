use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut ans = vec![];
    for i in 0..n {
        if i == n - 1 {
            ans.push(a[i]);
            continue;
        }
        if (a[i] - a[i + 1]).abs() == 1 {
            ans.push(a[i]);
            continue;
        }

        let x = if a[i] > a[i + 1] { -1 } else { 1 };
        let mut y = a[i];
        for _ in 0..(a[i] - a[i + 1]).abs() {
            ans.push(y);
            y += x;
        }
    }

    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
