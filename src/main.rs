extern crate clap;
use indicatif::{ProgressBar, ProgressStyle};


use clap::{Arg, App};

fn main() {
    let matches = App::new("rget")
        .version("0.1.0")
        .author("Sudip Roy")
        .about("Wget clone written in Rust")
        .arg(Arg::with_name("URL")
            .required(true)
            .index(1)
            .help("The URL to download"))
        .get_matches();
    let url = matches.value_of("URL").unwrap();
    println!("{}", url);
}

fn create_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
    let bar = match quiet_mode {
        true => ProgressBar::hidden(),
        false => {
            match length {
                Some(len) => ProgressBar::new(len),
                None => ProgressBar::new_spinner(),
            }
        }
    };
    bar.set_message(msg);
    match length.is_some() {
        true => bar
        .set_style(ProgressStyle::default_bar()
        .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}")
        .unwrap()
        .progress_chars("=> ")),
        false => bar.set_style(ProgressStyle::default_spinner()),
    };
    bar
}