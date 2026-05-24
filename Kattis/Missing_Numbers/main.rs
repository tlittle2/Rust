use std::{collections::HashSet, io};

fn main() {
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
    let x: i32 = buf.trim().parse().unwrap();
    buf.clear();

    let stg = (0..x).into_iter().fold(HashSet::new(), |mut acc, _|{
        let _ = io::stdin().read_line(&mut buf);
        let v: i32 = buf.trim().parse().unwrap();
        acc.insert(v);
        buf.clear();
        acc
    });

    let mx = stg.iter().max().unwrap();

    let mstr = (1..*mx).into_iter().fold(HashSet::new(), |mut acc, x|{
        acc.insert(x);
        acc
    });

    let diff: HashSet<&i32> = mstr.difference(&stg).into_iter().collect();

    if diff.len() == 0{
        println!("Good job");
    }else{
        let mut lst: Vec<&i32> = mstr.difference(&stg).into_iter().collect();
        lst.sort();
        lst.iter().for_each(|x| println!("{:?}", x));
    }
}
