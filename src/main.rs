#![feature(splice)]

#[macro_use]
extern crate error_chain;
mod errors {
    error_chain!{}
}
use errors::*;

extern crate structopt;
use structopt::StructOpt;
#[macro_use]
extern crate structopt_derive;

#[derive(Debug, StructOpt)]
struct Arguments {
    #[structopt(help = "regular expression and color pairs")]
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

    if args.regexcolors.len() == 0 {
        bail!(Arguments::clap().get_matches().usage());
    }

    ensure!(
        args.regexcolors.len() % 2 == 0,
        "Wrong number of regex and color"
    );

    let mut regexcolors: Vec<(Regex, &str)> = vec![];
    let mut regex = &String::new();
    for (i, rc) in args.regexcolors.iter().enumerate() {
        if i % 2 == 0 {
            regex = rc;
        } else {
            let regex = Regex::new(regex.as_str()).chain_err(|| {
                format!("Failed to construct regex: {}", regex)
            })?;
            Color::from_str(rc.as_str()).map_err(|_| {
                format!("Failed to convert string to Color: {}", rc)
            })?;
            regexcolors.push((regex, rc));
        }
    }

    loop {
        let mut line = String::new();
        let size = io::stdin().read_line(&mut line).chain_err(
            || "Failed to readline",
        )?;

        if size == 0 {
            break;
        }

        for ref pair in regexcolors.iter() {
            line = colorize(&line, &pair.0, pair.1)?;
        }

        print!("{}", line);
    }

    Ok(())
}

quick_main!(run);
