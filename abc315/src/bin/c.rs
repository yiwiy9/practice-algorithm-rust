use proconio::input;

fn main() {
    input! {
        n: usize,
        mut fs: [(usize,usize); n],
    }

    let compare = |&i: &(usize, usize), &j: &(usize, usize)| j.1.cmp(&i.1);
    fs.sort_by(compare);

    let first_f = fs[0].0;
    let first_s = fs[0].1;

    let mut same_f_s = 0;
    let mut another_f_s = 0;
    for &(f, s) in fs.iter().skip(1) {
        if same_f_s != 0 && another_f_s != 0 {
            break;
        }
        if same_f_s == 0 && f == first_f {
            same_f_s = s;
        }
        if another_f_s == 0 && f != first_f {
            another_f_s = s;
        }
    }

    let mut ans = first_s + same_f_s / 2;
    ans = ans.max(first_s + another_f_s);

    println!("{ans}");
}
