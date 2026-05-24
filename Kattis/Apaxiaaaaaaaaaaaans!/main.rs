use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let temp = buf.trim();

    let a: String = temp.to_owned() + " ";

    (0..a.len()).into_iter().for_each(|x| 
        if a.chars().nth(x+1) != a.chars().nth(x){
            print!("{}", a.chars().nth(x).unwrap())
    })
}
