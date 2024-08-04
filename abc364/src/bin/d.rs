use std::usize;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64;n],
        b: [(u64,u64);n],
    };
    let mut l=vec![];
    for i in 0..n{
        l.push(('a',i, a[i]));
    }
    for i in 0..q {
        l.push(('b',i,b[i].1));
    }
    l.sort_by(|a, b| a.2.cmp(&b.2));
    for i in 0..q {
        let mut bi=0;
        for j in 0..=n+q-2{
            //search bi
            if l[j].0 == 'b' &&l[j].1 == i {
                bi=j;
                break;
            }    
        }
        let mut cnt=1;
        for j in bi..=n+q-2{
            if l[j].0 == 'a' {
                cnt+=1;
                if cnt==b[i].1 {
                    println!("{}", b[i].0.abs_diff(a[j]));
                    break;
                }
            }
        }
    }
}