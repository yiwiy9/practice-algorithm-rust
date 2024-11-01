use proconio::{input, marker::Usize1};

const DX: [i64; 8] = [2, 1, -1, -2, -2, -1, 1, 2];
const DY: [i64; 8] = [1, 2, 2, 1, -1, -2, -2, -1];

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut set = std::collections::HashSet::new();
    for &(a, b) in &ab {
        set.insert((a, b));
        for i in 0..8 {
            let (x, y) = (a as i64 + DX[i], b as i64 + DY[i]);
            if 0 <= x && x < n as i64 && 0 <= y && y < n as i64 {
                set.insert((x as usize, y as usize));
            }
        }
    }

    println!("{}", n * n - set.len());
}
