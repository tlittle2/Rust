use std::io::stdin;

fn process_input(mut ip: String) -> String {
    stdin().read_line(&mut ip).expect("can't find value from stdin");

    let x: String = ip.trim().parse().expect("Can't find associated type");

    return x;

}

fn main() {
    let ip: Vec<f32> = process_input(String::new())
                        .split_whitespace()
                        .map(|x| x.parse::<f32>()
                        .unwrap())
                        .collect();

    let (a,b,c) = (ip[0],ip[1],ip[2]);

    println!("{}", a*c + (0.5 * b * (c * c)));

}
