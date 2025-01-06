use proconio::input;

fn main() {
    input! {
        mut p: char,
        mut q: char,
    }

    let line = vec![3, 1, 4, 1, 5, 9];
    let mut s = vec![0; 7];
    for i in 0..6 {
        s[i + 1] = s[i] + line[i];
    }

    if p > q {
        std::mem::swap(&mut p, &mut q);
    }

    println!(
        "{}",
        s[q as usize - 'A' as usize] - s[p as usize - 'A' as usize]
    );
}
