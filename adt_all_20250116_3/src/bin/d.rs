use proconio::input;

fn main() {
    input! {
        p: [u8; 26],
    }

    println!(
        "{}",
        p.iter()
            .map(|&x| (b'a' + x - 1) as char)
            .collect::<String>()
    );
}
