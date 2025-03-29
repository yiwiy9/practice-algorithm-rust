use proconio::input;

fn main() {
    input! {
        a: [usize; 7],
    }

    let mut cnt = vec![0; 14];
    for i in 0..7 {
        cnt[a[i]] += 1;
    }

    let mut more_3 = 0;
    let mut more_2 = 0;
    for i in 1..14 {
        if cnt[i] >= 3 {
            more_3 += 1;
        } else if cnt[i] == 2 {
            more_2 += 1;
        }
    }

    if more_3 == 0 {
        println!("No");
    } else if more_3 >= 2 {
        println!("Yes");
    } else if more_2 >= 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
