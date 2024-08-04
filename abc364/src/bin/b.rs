use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: i64,
        w: i64,
        mut si: i64,
        mut sj: i64,
        c:[Chars;h],
        xs: Chars
    };
    si-=1;
    sj-=1;
    let d = HashMap::from(
        [
            ('L',(0, -1)),
            ('U',(-1, 0)),
            ('D',(1, 0)),
            ('R',(0, 1)),
        ]
    );
    for x in xs.iter(){
        let dir=d[x];
        // println!("{}: {} {} + {} {}", x, si+1, sj+1, dir.0, dir.1);
        if si+dir.0!=-1 && dir.1+sj!=-1 && si+dir.0!=h && sj+dir.1!=w  && c[(dir.0+si) as usize][(dir.1+sj)as usize]=='.'{
            si+=dir.0;
            sj+=dir.1;
        }
    }
    println!("{} {}",si+1,sj+1);

}
