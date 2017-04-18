use std::io::{BufRead,BufReader,Read,stdin};
use std::string::String;
// use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;

mod counting;
use counting::*;

mod correcting;
use correcting::{correction, edits1, edits2};

fn main() {
    //for w in edits2("to") {
    //	println!("{}", w);
    //}
	let w = edits1("world");
	let w2 = edits2("world");
	println!("{}", w.len()+w2.len());


	let args: Vec<String> = env::args().collect();
	let ref path = args[1];

	let file = File::open(path).unwrap();
	let buf_reader = BufReader::new(file);
	let corpus = read_corpus(buf_reader);

	for (word, ct) in corpus {
		println!("{} : {}", word, ct);
	}

	let words = read_lines(stdin());
	for w in words {
		println!("{}", w);
	}

}

/// Reads all lines of an input source into a vector
fn read_lines<R: Read>(reader: R) -> Vec<String> {
    let mut input: Vec<String> = vec![];
    let mut lines = BufReader::new(reader).lines();

    while let Some(Ok(line)) = lines.next() {
    	input.push(line);
    }

    return input;
}

#[cfg(test)]
mod read_corpus_tests {
	use super::read_corpus;
	use super::WordCounts;
	use std::io::Cursor;


    #[test]
    fn read_nothing_test() {
    	let expected = WordCounts::new();
        assert_read(&expected, "");
    }

    #[test]
    fn read_unique_test() {
    	let input = "hello world, it's me! \n Mario";
    	let mut expected = WordCounts::new();
		
		expected.insert("hello".to_string(), 1);
		expected.insert("world".to_string(), 1);
		expected.insert("it's".to_string(), 1);
		expected.insert("me".to_string(), 1);
		expected.insert("mario".to_string(), 1);

        assert_read(&expected, input);
    }

    #[test]
    fn read_multiples_test() {
    	let input = "hello world, it's me! \n Mario \n Mario it's me!";
    	let mut expected = WordCounts::new();
		
		expected.insert("hello".to_string(), 1);
		expected.insert("world".to_string(), 1);
		expected.insert("it's".to_string(), 2);
		expected.insert("me".to_string(), 2);
		expected.insert("mario".to_string(), 2);

        assert_read(&expected, input);
    }

    fn assert_read(expected: &WordCounts, input: &str) {
        let mock_read = Cursor::new(input);
        let counts = read_corpus(mock_read);
        assert_eq!(expected.to_owned(), counts);
    }
}

#[cfg(test)]
mod word_probability_tests {
	use super::{WordCounts, word_probability};

	#[test]
	fn empty_test() {
		let counts = WordCounts::new();
		assert_eq!(0., word_probability("nope", &counts));

	}

	#[test]
	fn zero_test() {
		let mut counts = WordCounts::new();
		counts.insert("hello".to_string(), 1);
		assert_eq!(0., word_probability("nope", &counts));
	}

	#[test]
	fn pos_test() {
		let mut counts = WordCounts::new();
		counts.insert("hello".to_string(), 1);
		counts.insert("world".to_string(), 1);
		assert_eq!(0.5, word_probability("hello", &counts));
	}
}

#[cfg(test)]
mod known_tests {
	use super::{WordCounts, known};
	use std::collections::HashSet;

	fn mock_counts() -> WordCounts {
		let mut mock = WordCounts::new();
		mock.insert("apples".to_string(), 1);
		mock.insert("bananas".to_string(), 1);
		mock.insert("cats".to_string(), 1);
		mock.insert("dogs".to_string(), 1);
		mock.insert("fish".to_string(), 1);

		return mock;
	}

	#[test]
	fn no_known_test() {
		let counts = mock_counts();
		let words = ["rust", "is", "hard"];
		assert_eq!(HashSet::new(), known(&words, &counts));
	}

	#[test]
	fn one_known_test() {
		let counts = mock_counts();
		let words = ["rust", "is", "hard", "apples"];
		let mut expected = HashSet::new();
		expected.insert("apples");

		assert_eq!(expected, known(&words, &counts));
	}

	#[test]
	fn multiple_known_test() {
		let counts = mock_counts();
		let words = ["rust", "is", "hard", "apples", "bananas", "cats", "dogs", "fish"];
		let mut expected = HashSet::new();
		expected.insert("apples");
		expected.insert("bananas");
		expected.insert("cats");
		expected.insert("dogs");
		expected.insert("fish");

		assert_eq!(expected, known(&words, &counts));
	}
}
