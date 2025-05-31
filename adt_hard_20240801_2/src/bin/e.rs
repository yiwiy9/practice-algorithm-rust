use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut back = vec![];
    let mut change_index = n;
    for i in (0..n).rev() {
        if i + 1 < n && p[i] > p[i + 1] {
            change_index = i;
            break;
        }
        back.push(p[i]);
    }

    let mut less_num = n;
    for j in 0..back.len() {
        if back[j] < p[change_index] {
            less_num = back[j];
            back[j] = p[change_index];
            break;
        }
    }

    let mut ans = p[0..change_index].to_vec();
    ans.push(less_num);
    ans.extend(back);

    println!("{}", ans.iter().join(" "));
}
