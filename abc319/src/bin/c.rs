use proconio::input;

fn main() {
    input! {
        c: [usize; 9],
    }

    let check_line = [
        (0, 1, 2),
        (3, 4, 5),
        (6, 7, 8),
        (0, 3, 6),
        (1, 4, 7),
        (2, 5, 8),
        (0, 4, 8),
        (2, 4, 6),
    ];

    let mut all_cnt = 0;
    let mut cnt = 0;

    let mut order = (0..9).collect::<Vec<_>>();

    // next_permutation()
    permutohedron::heap_recursive(&mut order, |order| {
        all_cnt += 1;

        let mut ok = true;
        for &(x, y, z) in &check_line {
            if c[x] == c[y] && order[x] < order[z] && order[y] < order[z] {
                ok = false;
                break;
            }
            if c[x] == c[z] && order[x] < order[y] && order[z] < order[y] {
                ok = false;
                break;
            }
            if c[z] == c[y] && order[z] < order[x] && order[y] < order[x] {
                ok = false;
                break;
            }
        }

        if ok {
            cnt += 1;
        }
    });

    println!("{:.10}", cnt as f64 / all_cnt as f64);
}
