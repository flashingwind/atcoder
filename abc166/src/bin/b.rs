use proconio::input;

fn main() {
    input! {
        mut n_pe: usize,
        mut n_can: usize,
    };
    let mut sunuke: Vec<i32> = vec![0; n_pe + 1];
    for _ in 1..=n_can {
        input! {
            mut d: usize,
            mut a: [usize;d],
        };
        //println!("d={d} a={:?}", a);
        for aa in a.iter() {
            sunuke[*aa] += 1;
        }
    }
    //println!("su={:?}", sunuke);
    let mut cnt = -1;
    for s in sunuke.iter() {
        if *s == 0 {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
