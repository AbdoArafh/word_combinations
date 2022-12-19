use colored::Colorize;
use inquire::{Confirm, Text};
use itertools::Itertools;
use std::{collections::HashSet, fs, process};

fn main() {
    let data = fs::read_to_string("words.txt").unwrap_or_else(|err| {
        match err.kind() {
            std::io::ErrorKind::NotFound => eprintln!("{}", "words.txt was not found".bold().red()),
            _ => eprintln!("{}", "something wrong happened".red().bold()),
        }
        process::exit(1);
    });

    let mut words: HashSet<String> = HashSet::new();
    for word in data.lines().map(|line| line.to_string()) {
        words.insert(word.to_ascii_lowercase());
    }

    loop {
        let word = Text::new("What is your word?").prompt().unwrap();

        for perm in word
            .to_lowercase()
            .chars()
            .permutations(word.len())
            .unique()
        {
            let perm = perm.into_iter().collect::<String>();

            if words.contains(&perm) {
                println!("{}", perm);
            }
        }

        if !Confirm::new("You want to continue?")
            .with_default(true)
            .prompt()
            .unwrap()
        {
            break;
        }
    }

    println!("{}", "Thank you for using our software!".bold().green());
}
