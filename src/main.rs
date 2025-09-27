use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader, BufRead};
use std::path::Path;
use chrono::{DateTime, Local};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if !Path::new("water.log").exists() {
        let mut file :File = File::create("water.log")?;

        let current_local: DateTime<Local> = Local::now();
        let custom_format = current_local.format("%Y-%m-%d %H:%M:%S");

        let entry: String = format!("{} 0\n", custom_format);
        file.write_all(entry.as_bytes())?;
    }

    if args.len() == 1 {
        let reader = BufReader::new(File::open("water.log").expect("Cannot open file!"));
        
        let mut intake: i32 = 0;
        for line in reader.lines() {
            let line: String = line?;
            let words: Vec<&str> = line.trim().split_whitespace().collect();
            let amount: i32 = words[2].trim().parse().expect("Wrong type");
            intake += amount;
        }
        println!("{}", intake)
    } else {
        let mut file: File = OpenOptions::new().write(true).append(true).open("water.log")?;

        let current_local: DateTime<Local> = Local::now();
        let custom_format = current_local.format("%Y-%m-%d %H:%M:%S");

        let amount: i32 = args[1].parse().expect("Wrong type");
        let entry: String = format!("{} {}\n", custom_format, amount);
        file.write_all(entry.as_bytes())?;
    }

    Ok(())
}


// nur amount von aktuellem tag wird ausgegeben bei args 1
//delete entries, und reset file