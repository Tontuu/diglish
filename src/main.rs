// TODO: xclip support
// TODO: libnotify support

#![allow(unreachable_code)]

mod cli;

use {
    colored::Colorize,
    scraper::{Html, Selector},
    std::io::{stdin, stdout, Write},
    std::process::exit,
};

fn get_word() -> String {
    print!("Enter word: ");

    stdout().flush().expect("Failed to flush!");

    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    if let Some('\n') = input.chars().next_back() {
	input.pop();
    }

    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    return input;
}

fn url_exists(mut url: String) -> bool {
    // Remove first character from url that tipically is /dictionary/english/word
    url.remove(0);

    let subdomains: Vec<&str> = url.split('/').collect();

    return !subdomains.last().unwrap().is_empty();
}

trait RemoveWhitespaces {
    fn remove_whitespaces(self) -> String;
}

impl RemoveWhitespaces for &str {
    fn remove_whitespaces(self) -> String {
        let mut result = String::new();

        for word in self.split_whitespace() {
            result.push_str(word);
            result.push(' ');
        }

        return result.trim().to_owned();
    }
}

fn main() {
    // let matches = cli::get_app()
    let matches = cli::new();

    let word = match matches.get_one::<String>("word") {
	Some(word) => word.to_string(),
	None => get_word().to_string(),
    };

    if word.chars().all(char::is_alphabetic) == false {
        eprintln!("{}: Unable to find word: Input is not a valid word", "ERROR".red());
        exit(1);
    }

    let url = format!("https://dictionary.cambridge.org/dictionary/english/{}", word);

    let res = reqwest::blocking::get(url).unwrap();

    if !url_exists(res.url().path().to_string()) {
        eprintln!("{}: Unable to find word: URL doesn't exist", "ERROR".red());
        exit(1);
    }

    let body = res.text().unwrap();
    let document = Html::parse_document(body.as_str());

    let meaning_block_selector = Selector::parse("div.def-block.ddef_block").unwrap();
    let meaning_selector = Selector::parse("div.def.ddef_d.db").unwrap();
    let example_selector = Selector::parse("span.eg.deg").unwrap();

    let mut meanings: Vec<String> = Vec::new();
    let mut examples: Vec<String> = Vec::new();

    for element in document.select(&meaning_block_selector) {
        let meaning_element = element.select(&meaning_selector).next();

        let meaning = meaning_element.map_or_else(
            || String::new(),
            |meaning_element| {
                meaning_element
                    .text()
                    .collect::<Vec<_>>()
                    .join("")
                    .remove_whitespaces()
            },
        );

        let example_element = element.select(&example_selector).next();

        let example = example_element.map_or_else(
            || String::new(),
            |example_element| {
                example_element
                    .text()
                    .collect::<Vec<_>>()
                    .join("")
                    .remove_whitespaces()
            },
        );

        meanings.push(meaning);
        examples.push(example);
    }

    for i in 0..meanings.len() {
        println!("{}\n\t- {}", meanings[i], examples[i]);
    }
}
