use std::{collections::{HashMap}, io};

fn main() {
    let mp = HashMap::from([
        ("ml gin", 45),
        ("ml fresh lemon juice", 30),
        ("ml simple syrup", 10),
    ]);

    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);

    let ip: i32 = buf.trim().parse().expect("not an int");

    buf.clear();

    let mut output: HashMap<i32, &str> = mp.iter().fold(HashMap::new(), |mut acc, x| {
        acc.insert(x.1 * ip, x.0);
        acc
    });
    
    if ip == 1 {
        output.insert(ip, "slice of lemon");
    }else{
        output.insert(ip, "slices of lemon");
    }

    let mut keys : Vec<&i32> = output.keys().into_iter().collect();
    keys.sort_unstable_by(|a,b| b.cmp(a));

    keys.iter().for_each(|x| println!("{} {}", x, output[x]));
}
