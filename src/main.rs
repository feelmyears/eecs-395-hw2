use std::io::{BufRead,BufReader,Read,stdin};
use std::env;
use std::fs::File;

mod counting;
use counting::{WordCounts, read_corpus};

mod correcting;
use correcting::correction;

fn main() {
	let corpus_reader = get_corpus_reader();
	let corpus = read_corpus(corpus_reader);
	predict_corrections(stdin(), &corpus);
}

fn get_corpus_reader() -> BufReader<File> {
	let args: Vec<String> = env::args().collect();
	let ref path = args[1];

	let file = File::open(path).unwrap();
	let buf_reader = BufReader::new(file);
	return buf_reader
}

fn predict_corrections<R: Read>(reader: R, corpus: &WordCounts) {
	let mut lines = BufReader::new(reader).lines();

    while let Some(Ok(line)) = lines.next() {
    	let word = line.trim();
    	let word_correction = correction(&word, corpus);
		if word == word_correction {
			println!("{}", word);
		}
    	else {println!("{}, {}", word, word_correction);}
    }
}

#[cfg(test)]
mod correct_tests {
	use counting::WordCounts;
	use correcting::correction;

	#[test]
    fn correct_test() {
		let mut corpus = WordCounts::new();
		corpus.insert("correct".to_string(), 1);
		let input = "corect";
		let output = correction(input, &corpus);
		assert_eq!("correct", output);
	}
}