use clap::{App, Arg};
use onig::{Captures, Regex};

fn titlecase_word(word: &str) -> String {
    word.chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                c.to_uppercase().collect::<String>()
            } else {
                c.to_lowercase().collect::<String>()
            }
        })
        .collect::<String>()
}

fn title_case(text: &str) -> String {
    let small_words = Regex::new(r"\b(?:an?d?|a[st]|because|but|by|en|for|i[fn]|neither|nor|o[fnr]|only|over|per|so|some|tha[tn]|the|to|up|upon|vs?\.?|versus|via|when|with|without|yet)\b").unwrap();
    let tokens = Regex::new(r"[^\s:–—-]+|.").unwrap();
    let manual_case = Regex::new(r".(?=[A-Z]|\..)").unwrap();

    tokens.replace_all(&text, |captures: &Captures| {
        let capture = captures.at(0).unwrap();
        let index = captures.pos(0).unwrap().0;
        let mut retval = String::from(capture);
        if !manual_case.find(capture).is_some()
            && (!small_words.is_match(capture) || index == 0 || index + capture.len() == text.len())
        {
            retval = titlecase_word(capture)
        }
        retval
    })
}

fn main() {
    let matches = App::new("tcc")
        .version("0.1.0")
        .author("Payson Wallach <payson@paysonwallach.com>")
        .about("Convert text to title case.")
        .arg(
            Arg::with_name("text")
                .help("The text to convert to title case.")
                .multiple(true)
                .required(true),
        )
        .get_matches();
    let text = matches
        .values_of("text")
        .unwrap()
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", title_case(&text));
}
