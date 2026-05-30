use std::{collections::{HashMap}, io};

fn main() {
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);


    let cases: i32 = buf.trim().parse().expect("No cases");
    buf.clear();


    let mp: HashMap<String, String> = (0..cases).into_iter().fold(HashMap::new(), |mut acc, _| {
        let _ = io::stdin().read_line(&mut buf);
        let ip: String = buf.trim().parse().expect("Nothing provided");
        buf.clear();

        let idx = ip.char_indices()
            .filter(|c| c.1.is_uppercase())
            .min_by_key(|c| c.0)
            .unwrap()
            .0 ;

        let substr = &ip[idx..ip.len()];

        acc.insert(String::from(substr), ip);

        acc
    });

    let mut keys: Vec<&String> = mp.keys().collect();

    keys.sort();

    for k in keys{
        println!("{}", mp[k])
    }

}
