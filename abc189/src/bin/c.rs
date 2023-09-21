use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize;n],
    };
    a.insert(0, 0);
    let mut max = 0;
    for l in 1..=n {
        let mut min_a = 100001_usize;
        let mut min_a_i = 0;
        for r in l..=n {
            if l <= min_a_i && min_a_i <= r {
                if a[l] < min_a {
                    min_a = a[l];
                    min_a_i = l;
                }
                if a[r] < min_a {
                    min_a = a[r];
                    min_a_i = r;
                }
            } else {
                for i in l..=r {
                    if a[i] < min_a {
                        min_a = a[i];
                        min_a_i = i;
                    }
                }
            }
            if max < min_a * (r - l + 1) {
                max = min_a * (r - l + 1);
            }
        }
    }
    println!("{}", max);
}
