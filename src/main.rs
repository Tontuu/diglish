// TODO: Get all meanings
// TODO: Get all examples
// TODO: Sanatize meanings and examples
// TODO: Proper error handling

use scraper::{Html, Selector};
use std::io::{stdin, stdout, Write};
use std::process::exit;
use colored::Colorize;

fn get_word() -> String {
    print!("Enter word: ");
    stdout().flush().unwrap();
    let mut input_str = String::new();
    stdin().read_line(&mut input_str)
	.ok()
	.expect("Failed to read line!");

    input_str
}

fn url_exists(mut url: String) -> bool {
    // Remove first character from url that tipically is /dictionary/english/word
    url = url[1..url.len()].to_string();

    let subdomains: Vec<&str> = url.split("/").collect();

    return !subdomains.last().unwrap().is_empty()
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

	result.trim().to_owned()
    }
}

#[tokio::main]
async fn main() {
    //let word: String = get_word().to_lowercase();

    //let url = format!("https://dictionary.cambridge.org/dictionary/english/{}", &word);
    let url = "https://dictionary.cambridge.org/dictionary/english/table";
    let res = reqwest::get(url).await.unwrap();

    if !url_exists(res.url().path().to_string()) {
	eprintln!("{}: Unable to find word: {}", "ERROR".red(), "URL doesn't exist");
	exit(1);
    }

    let body = res.text().await.unwrap();
    let document = Html::parse_document(body.as_str());

    let meaning_block_selector = Selector::parse("div.def-block.ddef_block").unwrap();
    let meaning_selector = Selector::parse("div.def.ddef_d.db").unwrap();
    let example_selector = Selector::parse("span.eg.deg").unwrap();

    let mut meanings: Vec<String> = Vec::new();
    let mut examples: Vec<String> = Vec::new();

    for element in document.select(&meaning_block_selector) {
	let meaning_element = element.select(&meaning_selector).next();

	let meaning = match meaning_element {
	    Some(meaning_element) => meaning_element.text().collect::<Vec<_>>().join("").remove_whitespaces(),
	    None => "".to_string()
	};

	let example_element = element.select(&example_selector).next();

	let example = match example_element {
	    Some(example_element) => example_element.text().collect::<Vec<_>>().join("").remove_whitespaces(),
	    None => "".to_string()
	};

	meanings.push(meaning);
	examples.push(example);
    }

    // for i in 0..meanings.len() {
    // 	println!("{}\n\t- {}", meanings[i], examples[i]);
    // }

    println!("Meanings: {} {:#?}", meanings.len(), meanings);
    println!("Examples: {} {:#?}", examples.len(), examples);

}

