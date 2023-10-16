use proconio::input;

fn main() {
    input! {
        mut n: [u32;3],
    };
    let mut cnt = 0;
    loop {
        n.sort();
        // println!("cnt={cnt} {:?}", n);
        n.sort();
        cnt += n[2] - n[1];
        n[0] += n[2] - n[1];
        n[1] += n[2] - n[1];
        // println!("cnt={cnt} {:?}", n);
        cnt += (n[1] - n[0]) / 2;
        n[0] += ((n[1] - n[0]) / 2) * 2;
        // println!("cnt={cnt} {:?}", n);
        if n[1] - n[0] == 1 {
            n[1] += 1;
            n[2] += 1;
            cnt += 1;
        }
        // println!("cnt={cnt} {:?}", n);
        if n[0] == n[1] && n[1] == n[2] {
            break;
        }
    }
    println!("{}", cnt);
}
