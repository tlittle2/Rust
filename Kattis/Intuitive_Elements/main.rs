use std::{collections::HashSet, io};

fn main() {
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);

    let ip: i32 = buf.trim().parse().expect("Nothing provided");

    buf.clear();

    let answer: Vec<&str> = (0..ip).into_iter().fold(Vec::new(), |mut acc, _|  {
        let _ = io::stdin().read_line(&mut buf);
        let element: String = buf.trim().parse().expect("Nothing Provided");
        let s1: HashSet<char> = element.chars().into_iter().collect();
        buf.clear();

        let _ = io::stdin().read_line(&mut buf);
        let abbrev: String = buf.trim().parse().expect("Nothing Provided");
        let s2: HashSet<char> = abbrev.chars().into_iter().collect();
        buf.clear();

        let intersect: HashSet<&char> = s1.intersection(&s2).collect();

        if intersect.len() == s2.len(){
            acc.push("YES");
        }else{
            acc.push("NO");
        }
        acc
    });

    let _ = &answer.iter().for_each(|x| println!("{}", x));

}
