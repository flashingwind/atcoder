use proconio::input;

fn main() {
    input! {
        n: usize,
        t: u32,
        p: usize,
        mut l: [u32;n],
    };
    let mut day=0;
    let mut cnt_long=l.iter().filter(|&&ll|t<=ll).count();
    while cnt_long<p {
        day+=1;
        for v in l.iter_mut() {
            *v+=1;
        }
        cnt_long=l.iter().filter(|&&ll|t<=ll).count();
        // println!("{day}: {:?} cnt={cnt_long}",l);
    }
    println!("{}",day);
}