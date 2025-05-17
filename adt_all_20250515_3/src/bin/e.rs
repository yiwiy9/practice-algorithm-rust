use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut a: [Usize1; n],
    }

    let mut pos = vec![0; n];
    for i in 0..n {
        pos[a[i]] = i;
    }

    let mut ans = vec![];
    for i in 0..n {
        if i == pos[i] {
            continue;
        }
        let pos_i = pos[i];
        let pos_cur = i;
        ans.push((pos_cur + 1, pos_i + 1));

        let pos_cur_num = a[pos_cur];
        pos.swap(i, pos_cur_num);
        a.swap(pos_i, pos_cur);
    }

    println!("{}", ans.len());
    for (i, j) in ans {
        println!("{} {}", i, j);
    }
}
