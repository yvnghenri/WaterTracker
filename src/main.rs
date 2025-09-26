use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader, BufRead};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if !Path::new("water.log").exists() {
        let mut file :File = File::create("water.log")?;
        file.write_all(b"0\n")?;
    }

    if args.len() == 1 {
        let reader = BufReader::new(File::open("water.log").expect("Cannot open file!"));

        let mut intake: i32 = 0;
        for line in reader.lines() {
            let line = line.expect("Malformation error");
            intake += line.trim().parse::<i32>().expect("Invalid");
        }
        println!("{}", intake)
    } else {
        let mut file: File = OpenOptions::new().write(true).append(true).open("water.log")?;
        let entry = format!("{}\n", args[1]);
        file.write_all(entry.as_bytes())?;
    }

    Ok(())
}