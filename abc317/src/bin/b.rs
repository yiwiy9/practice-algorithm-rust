use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();

    let mut ans = 0;
    for (i, a_i) in a.iter().enumerate() {
        if i + 1 < a.len() && a[i + 1] != a_i + 1 {
            ans = a_i + 1;
            break;
        }
    }

    println!("{}", ans);
}
