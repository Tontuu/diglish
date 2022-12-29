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

    let meaning_selector = Selector::parse("div.def.ddef_d.db").unwrap();
    let example_selector = Selector::parse("span.eg.deg").unwrap();

    let mut meanings: Vec<String> = Vec::new();
    let mut examples: Vec<String> = Vec::new();

    for element in document.select(&example_selector) {
	let text = element.text().collect::<Vec<_>>().join("").trim().to_string();
	examples.push(text);
    }

    for element in document.select(&meaning_selector) {
	let text = element.text().collect::<Vec<_>>().join("").trim().to_string();
	meanings.push(text);
    }

    for i in 0..meanings.len() {
	println!("{}\n\t- {}", meanings[i], examples[i]);
    }

    println!("Meanings: {} {:#?}", meanings.len(), meanings);
    println!("Examples: {} {:#?}", examples.len(), examples);

}

