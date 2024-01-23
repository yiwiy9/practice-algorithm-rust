use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize,usize); n],
    }

    let t_points = xy.iter().map(|(x, y)| *x).sum::<usize>();
    let a_points = xy.iter().map(|(x, y)| *y).sum::<usize>();

    println!(
        "{}",
        if t_points > a_points {
            "Takahashi"
        } else if t_points < a_points {
            "Aoki"
        } else {
            "Draw"
        }
    );
}
