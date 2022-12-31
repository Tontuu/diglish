// TODO: Get all meanings
// TODO: Get all examples
// TODO: Sanatize meanings and examples
// TODO: Proper error handling
// TODO: xclip support
// TODO: libnotify support

mod cli;

use {
    colored::Colorize,
    clap::Parser,
    scraper::{Html, Selector},
    std::io::{stdin, stdout, Write},
    std::process::exit,
};

fn get_word() -> String {
    print!("Enter word: ");

    stdout().flush().expect("Failed to flush!");

    let mut input_str = String::new();

    stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line!");

    return input_str;
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

    let matches = cli::parse_arguments().get_matches();

    match matches.subcommand() {
	Some(("clip", sub_matches)) => {
	    println!("Copied to clipboard");
	}
	_ => unreachable!(),
    }

    todo!();

    let url = if !cfg!(debug_assertions) {
	let word: String = get_word().to_lowercase();
	format!("https://dictionary.cambridge.org/dictionary/english/{}", &word)
    } else {
	"https://dictionary.cambridge.org/dictionary/english/table".to_string()
    };

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
