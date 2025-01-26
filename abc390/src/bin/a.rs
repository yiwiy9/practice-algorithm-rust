use proconio::input;

fn main() {
    input! {
        a: [i32; 5],
    }

    let mut ok = false;
    for i in 1..5 {
        let mut b = a.clone();
        b.swap(i - 1, i);
        if b == [1, 2, 3, 4, 5] {
            ok = true;
            break;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
