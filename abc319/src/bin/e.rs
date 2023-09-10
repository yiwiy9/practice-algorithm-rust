use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        pt: [(usize,usize); n-1],
        q:usize
    }

    let lcm = (1..=8).fold(1, num::integer::lcm);

    let mut s = (0..lcm).collect::<Vec<usize>>();
    for i in 0..lcm {
        for &(p, t) in &pt {
            let waiting_bus = if s[i] % p == 0 { 0 } else { p - (s[i] % p) };
            s[i] += waiting_bus + t;
        }
    }

    for _ in 0..q {
        input! {
            start_time: usize,
        }
        let start_mod_lcm = (start_time + x) % lcm;
        let rest = start_time + x - start_mod_lcm;
        let bus_time = s[start_mod_lcm];
        println!("{}", rest + bus_time + y);
    }
}
