#[macro_use]
extern crate failure;

type Result<T> = std::result::Result<T, failure::Error>;

#[macro_use]
extern crate structopt;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Arguments {
    #[structopt(help = "regex and color pair(s)")]
    regexcolors: Vec<String>,
}

extern crate regex;
use regex::Regex;
use std::io;
extern crate colored;
use colored::{Color, Colorize};
use std::str::FromStr;

fn colorize(line: &str, regex: &Regex, color: &str) -> Result<String> {
    let mut line = line;
    let mut cline = String::new();
    loop {
        if let Some(mat) = regex.find(line) {
            cline += &line[..mat.start()];
            cline += format!("{}", line[mat.start()..mat.end()].color(color)).as_str();
            line = &line[mat.end()..];
        } else {
            cline += line;
            break;
        }
    }

    Ok(cline)
}

#[test]
fn test_colorize() {
    let line = "NB".to_owned();
    let regex = Regex::new("NB").unwrap();
    let color = "red";
    let cline = colorize(&line, &regex, color).unwrap();
    assert_eq!(cline, "\u{1b}[31mNB\u{1b}[0m");

    let line = "nbNBNBnb".to_owned();
    let cline = colorize(&line, &regex, color).unwrap();
    assert_eq!(cline, "nb\u{1b}[31mNB\u{1b}[0m\u{1b}[31mNB\u{1b}[0mnb");
}

fn run() -> Result<()> {
    let args = Arguments::from_args();
    ensure!(
        args.regexcolors.len() > 0,
        "Regex and color pair should be more than 1!"
    );
    ensure!(
        args.regexcolors.len() % 2 == 0,
        "Regex and color should be in pair!"
    );

    let mut regexcolors: Vec<(Regex, &str)> = vec![];
    let mut regex = &String::new();
    for (i, rc) in args.regexcolors.iter().enumerate() {
        if i % 2 == 0 {
            regex = rc;
        } else {
            let regex = Regex::new(regex)?;
            let _ = Color::from_str(rc).map_err(|_| failure::err_msg("Color not found!"))?;
            regexcolors.push((regex, rc));
        }
    }

    loop {
        let mut line = String::new();
        let size = io::stdin().read_line(&mut line)?;

        if size == 0 {
            break;
        }

        for pair in &regexcolors {
            line = colorize(&line, &pair.0, pair.1)?;
        }

        print!("{}", line);
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
    }
}
