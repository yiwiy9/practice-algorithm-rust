use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize,usize); n],
    }

    let takahashi = xy.iter().map(|&(x, _)| x).sum::<usize>();
    let aoki = xy.iter().map(|&(_, y)| y).sum::<usize>();

    println!(
        "{}",
        if takahashi > aoki {
            "Takahashi"
        } else if takahashi < aoki {
            "Aoki"
        } else {
            "Draw"
        }
    );
}
