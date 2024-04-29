use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32;n],
    };
    sort(&mut a);
}

fn sort<T: PartialOrd + Clone>(source: &mut [T]) {
    fn q_sort<TInner: PartialOrd + Clone>(
        source: &mut [TInner],
        left: usize,
        right: usize,
        log: &mut Vec<(usize, usize)>,
    ) {
        let pivot = source[(left + right) >> 1].clone();
        let mut l = left;
        let mut r = right;
        while l <= r {
            while pivot < source[r] && r > left {
                r -= 1;
            }
            while source[l] < pivot && l < right {
                l += 1;
            }
            if l <= r {
                source.swap(l, r);
                if l != r {
                    log.push((l + 1, r + 1));
                }

                if r > 0 {
                    r -= 1;
                }
                l += 1;
            }
        }
        if left < r {
            q_sort(source, left, r, log);
        }
        if right > l {
            q_sort(source, l, right, log);
        }
    }

    let size = source.len() - 1;
    let mut log: Vec<(usize, usize)> = vec![];
    q_sort(source, 0, size, &mut log);
    println!("{}", log.len());
    for l in log.iter() {
        println!("{} {}", l.0, l.1);
    }
}
