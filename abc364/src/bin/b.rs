use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut cur_i: Usize1,
        mut cur_j: Usize1,
        c: [Chars; h],
        x:Chars
    }

    for &x_i in &x {
        match x_i {
            'L' => {
                if cur_j > 0 && c[cur_i][cur_j - 1] != '#' {
                    cur_j -= 1;
                }
            }
            'R' => {
                if cur_j < w - 1 && c[cur_i][cur_j + 1] != '#' {
                    cur_j += 1;
                }
            }
            'U' => {
                if cur_i > 0 && c[cur_i - 1][cur_j] != '#' {
                    cur_i -= 1;
                }
            }
            'D' => {
                if cur_i < h - 1 && c[cur_i + 1][cur_j] != '#' {
                    cur_i += 1;
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{} {}", cur_i + 1, cur_j + 1);
}
