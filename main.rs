use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;

fn main() -> Result<()> {
    let f = File::open("data/DerivedName-14.0.0.txt")?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line_string = match line {
            Ok(value) => value,
            Err(_error) => String::from("could not unwrap line"),
        };
        let line_str = &line_string.as_str();
        let v: Vec<&str> = line_str.split(';').collect();
        let i = v.iter().filter(|s| !s.starts_with('#')).map(|s| s.trim());
        for x in i {
            println!("{}", &x);
        }
    }

    Ok(())
}
