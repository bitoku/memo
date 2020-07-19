extern crate clap;
use clap::{App};
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> std::io::Result<()>{
    let matches = App::new("memo")
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .args_from_usage("<sentence> 'sentence you want to note'")
        .get_matches();

    let sentence = match matches.value_of("sentence") {
        Some(x) => x,
        None => {
            eprintln!("empty sentence");
            std::process::exit(1);
        },
    };

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("note.txt")?;
    write!(file, "{}\n", sentence)?;
    file.flush()?;
    Ok(())
}
