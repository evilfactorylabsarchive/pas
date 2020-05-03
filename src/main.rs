use rand::seq::SliceRandom;
use std::io::{BufRead, BufReader};
use std::{fs, io};

fn main() {
    // thanks to @geovedi's indonesian-wordlist
    // https://github.com/geovedi/indonesian-wordlist
    let source = "05-ivanlanin2011-sort-alpha.lst";

    let twelve_passphrase = generate_random_words(source, 12);

    println!("{}", twelve_passphrase);
}

fn generate_random_words(source: &str, len: usize) -> String {
    // in future we should use `include_bytes!` macro
    let lines = file_to_vec(source).expect("unable to open file");

    let random_words: Vec<String> = lines
        .choose_multiple(&mut rand::thread_rng(), len)
        .map(String::from)
        .collect();

    random_words.join(" ")
}

fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);

    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}
