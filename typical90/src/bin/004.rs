use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a:[[i32;w];h],
    }

    let mut row_sum = vec![0; h];
    let mut line_sum = vec![0; w];

    for (i, r) in row_sum.iter_mut().enumerate().take(h) {
        for (j, l) in line_sum.iter_mut().enumerate().take(w) {
            *r += a[i][j];
            *l += a[i][j];
        }
    }

    for (i, r) in row_sum.iter().enumerate().take(h) {
        for (j, l) in line_sum.iter().enumerate().take(w) {
            print!("{}", r + l - a[i][j]);
            if j != w - 1 {
                print! {" "};
            }
        }
        println!()
    }
}
