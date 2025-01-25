use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }

    let mut ans = vec![];
    ans.push((x * y, x * y));
    let mut i = 0;
    for x_i in 0..x {
        i += 1;
        let mut j = i;
        for _ in 0..y - 1 {
            j += 1;
            ans.push((j - 1, j));
        }
        if x_i < x - 1 {
            ans.push((i, j + 1));
            i = j;
        } else {
            ans.push((i, 1));
        }
    }

    println!(
        "{}",
        ans.iter()
            .map(|(a, b)| format!("{} {}", a, b))
            .collect::<Vec<String>>()
            .join("\n")
    );
}
