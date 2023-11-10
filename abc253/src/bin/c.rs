use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        q: usize,
    };
    // let mut s=vec![0;10usize.pow(5)+1];
    let mut s=BTreeMap::new();
    for _ in 0..q{
        input! {
            t: usize,
        };
        if t==1{
            input! {
                x: usize,
            };
            // s[x]+=1;
            *s.entry(x).or_insert(0)+=1;
        }else if t==2{
            input! {
                x: usize,
                c: usize,
            };
            // s[x]-=c.min(s[x]);
            let cc=s.entry(x).or_insert(0);
            if *cc<=c{
                s.remove(&x);
            }else{
                *cc-=c;
            }
        }else{
            let mut min=*s.first_entry().unwrap().key();
            let mut max=*s.last_entry().unwrap().key();;
            println!("{}",max-min);
        }
        // let ss=s[0..10].to_vec();
        // println!("{:?}",ss);
    }
}
