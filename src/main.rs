use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if !Path::new("water.log").exists() {
        let mut file :File = File::create("water.log")?;
        file.write_all(b"0\n")?;
    }

    if args.len() == 1 {
        let mut file: File = OpenOptions::new().read(true).open("water.log")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        println!("{}", contents);
    } else {
        let mut file: File = OpenOptions::new().write(true).append(true).open("water.log")?;
        let entry = format!("{}\n", args[1]);
        file.write_all(entry.as_bytes())?;
    }
    
    Ok(())
}