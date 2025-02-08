use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
    }

    let mut j_pos = (1..=n).collect::<Vec<_>>();
    for i in (0..m).rev() {
        let a_i = a[i];
        j_pos.swap(a_i, a_i + 1);
    }

    let mut cur_pos = 0;
    for i in 0..m {
        let a_i = a[i];
        j_pos.swap(a_i, a_i + 1);
        println!("{}", j_pos[cur_pos]);
        if a_i == cur_pos {
            cur_pos += 1;
        } else if a_i + 1 == cur_pos {
            cur_pos -= 1;
        }
    }
}
