use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let cnt = (n - 1) / 2;
    let front = vec!['-'; cnt];
    let mid = vec!['='; if n % 2 == 0 { 2 } else { 1 }];
    let back = vec!['-'; cnt];
    let ans = front
        .iter()
        .chain(mid.iter())
        .chain(back.iter())
        .collect::<String>();

    println!("{}", ans);
}
