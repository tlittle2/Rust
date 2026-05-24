use std::io;

fn main() {
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);

    let ip = buf.trim();

    let l = ip.len();

    let mut space: f32 = 0.0;
    let mut lower: f32 = 0.0;
    let mut upper: f32 = 0.0;
    let mut symbols: f32 = 0.0;

    ip.chars().for_each(|c| 
        if c == '_' {
            space = space + 1.0;
        }else if c.is_lowercase(){
            lower = lower + 1.0;
        }else if c.is_uppercase(){
            upper = upper + 1.0;
        }else{
            symbols = symbols + 1.0;
        }
    );

    let collection: [f32; 4] = [space, lower, upper, symbols];


    collection.iter().for_each(|f| 
        println!("{:?}", f / (l as f32))
    );

}
