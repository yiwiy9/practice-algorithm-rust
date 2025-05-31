use proconio::input;

const DX: [i64; 8] = [1, 2, 2, 1, -1, -2, -2, -1];
const DY: [i64; 8] = [2, 1, -1, -2, -2, -1, 1, 2];

fn main() {
    input! {
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
    }

    let mut set = std::collections::HashSet::new();
    for i in 0..8 {
        let nx = x1 + DX[i];
        let ny = y1 + DY[i];
        set.insert((nx, ny));
    }

    for i in 0..8 {
        let nx = x2 + DX[i];
        let ny = y2 + DY[i];
        if set.contains(&(nx, ny)) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
