extern crate clap;
use clap::{App};
use std::fs::OpenOptions;
use std::io::Write;
use std::env;
use chrono::Local;

fn main() -> std::io::Result<()>{
    let memofile = match env::var("MEMO_FILE") {
        Ok(x) => x,
        Err(_) => "note.txt".to_string(),
    };
    let date_format = match env::var("MEMO_DATE_FORMAT") {
        Ok(x) => x,
        Err(_) => "%Y/%m/%d %H:%M:%S".to_string(),
    };
    let matches = App::new("memo")
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .args_from_usage("<sentence> 'sentence you want to note'")
        .get_matches();
    let sentence = match matches.value_of("sentence") {
        Some(x) => x.to_string(),
        None => {
            eprintln!("empty sentence");
            std::process::exit(1);
        },
    };
    let datetime = Local::now().format(&date_format).to_string();
    let sentence = join_sentence(datetime, sentence);
    memo(memofile, sentence)?;
    Ok(())
}

fn memo(memofile: String, sentence: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(memofile)?;
    write!(file, "{}\n", sentence)?;
    file.flush()?;
    return Ok(());
}

fn join_sentence(sentence1: String, sentence2: String) -> String {
    format!("{} {}", sentence1, sentence2)
}
