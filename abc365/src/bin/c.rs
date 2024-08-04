use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: u128,
        mut a: [u128;n],
    };
    a.sort();

    let mut sum=0;
    
    let a_cnt=a.iter().enumerate().counts_by(|(i,a)|a);
    
    let mut old_ai=0;
    let mut old_cnt=0;
    let mut old_sum=0;
    for (&&ai,&cnt) in a_cnt.iter().sorted_by(|a,b|a.0.cmp(&b.0)){
        let da=ai-old_ai;
        let k=da*(n-old_cnt) as u128;
        sum+=da*k;
        println!("ai={} sum+=k:{n}-{old_cnt} x da:{da} = {}, {sum}-{m}",ai,da*m);
        if m < sum {
            let res=(sum-m)/cnt as u128;
            println!("{}",old_ai);
            println!("+{res}");
            return;
        }else if m==sum {
            println!("{}",ai);
            return;
        }
        old_ai=ai;
        old_cnt=cnt;
        old_sum=sum;
    }
    println!("infinite");
}