use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 7*n],
    }

    let mut cnt = 0;
    let mut ans = vec![];
    for (i, &a_i) in a.iter().enumerate() {
        cnt += a_i;
        if i % 7 == 6 {
            ans.push(cnt);
            cnt = 0;
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
