use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut c = a.clone();
    c.extend_from_slice(&b);
    c.sort();

    for i in 0..c.len() - 1 {
        if a.contains(&c[i]) && a.contains(&c[i + 1]) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
