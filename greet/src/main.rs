use std::{collections::HashMap, env, fs::File, io::{BufReader, BufRead}};

struct WordCounter(HashMap<String, u64>);

impl WordCounter {
    fn new() -> WordCounter {
        WordCounter(HashMap::new())
    }

    fn increment_count(&self, s: &str) {
        let mut map = self.0.clone();
        let count = map.entry(s.to_string()).or_insert(0);
        *count += 1;
    }
    fn display(&self) {
        for (word, count) in &self.0 {
            println!("{}: {}", word, count);
        }
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let filename = &arguments[1];
    println!("Processing file {}", filename);
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let word_counter = WordCounter::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let words = line.split(" ");
        for word in words {
            if word == "" {
                continue;
            }else {
                word_counter.increment_count(word);
            }
        }
    }
    word_counter.display();
}
