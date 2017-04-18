use std::io::{BufRead, BufReader, Read};
use std::string::String;
use std::collections::HashMap;

extern crate regex;
use self::regex::Regex;

pub type WordCounts = HashMap<String, usize>;

pub fn read_corpus<R: Read>(reader: R) -> WordCounts {
	let mut counts: WordCounts = WordCounts::new();
	let mut lines = BufReader::new(reader).lines();
	let re = Regex::new(r"[\w']+").unwrap();

    while let Some(Ok(line)) = lines.next() {
    	for caps in re.captures_iter(&line) {
			for m in caps.iter() {
				let tok = m.unwrap().as_str().to_lowercase(); 
				*counts.entry(tok).or_insert(0) += 1;
			}			
		}	
    }

	return counts;
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
