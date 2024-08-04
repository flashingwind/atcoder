use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32;n],
    };
    for (cnt,(i,v))in a.iter().enumerate().sorted_by(|a,b|a.1.cmp(&b.1)).rev().enumerate(){
        // println!("cnt={cnt} i={i} v={v}");
        if cnt==1{
            println!("{}",i+1);
            return;
        }
    }
}