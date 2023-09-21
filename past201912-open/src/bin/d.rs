use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize;n],
    };
    a.sort();
    let mut deleted = 0;
    let mut inserted = 0;
    for (i, _) in a.iter().enumerate().skip(1) {
        if a[i - 1] == a[i] {
            inserted = a[i];
            a.remove(i);
            break;
        }
    }
    if inserted != 0 {
        for i in 0..n - 1 {
            if i + 1 != a[i] {
                deleted = i + 1;
                break;
            }
        }
        if deleted == 0 {
            deleted = n;
        }
    }
    if deleted == 0 && inserted == 0 {
        println!("Correct");
    } else {
        println!("{} {}", inserted, deleted);
    }
}
