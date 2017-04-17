extern crate regex;

mod WordCounter {
	use std::io::{BufRead,BufReader,Read,stdin};
	use std::string::String;
	use std::collections::HashMap;

	use self::regex::Regex;

	type WordCounter = HashMap<String, usize>;

	#[derive(Debug)]
	struct WordFreq{
	    word: String,
	    freq: usize
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

	/// Extracts all words from a string and puts them into a vector
	fn parse_line(line: &str) -> Vec<String> {
		let mut tokens: Vec<String> = vec![];
		let re = Regex::new(r"[\w']+").unwrap();

		for caps in re.captures_iter(line) {
			for m in caps.iter() {
				let tok = m.unwrap().as_str().to_string(); 
				tokens.push(tok);
			}			
		}	
		return tokens;
	}

	/// Exracts all words from a vector of strings
	fn get_words(lines: Vec<String>) -> Vec<String> {
		let mut words: Vec<String> = vec![];

		for l in lines {
			let line_words = parse_line(&l);
			for lw in line_words {
				words.push(lw);
			}
		}

		return words;
	}

	/// Counts words (treated as lowercase) and returns a map of the form: <word, word_count>
	fn count_words(words: Vec<String>) -> HashMap<String, usize> {
		let mut counts = HashMap::new();

		for w in words {
			let lc = w.to_lowercase();
			*counts.entry(lc).or_insert(0) += 1;
		}

		return counts;
	}


	#[cfg(test)]
	mod read_lines_tests {
	    use super::read_lines;
	    use std::io::Cursor;

	    #[test]
	    fn read_nothing_test() {
	        assert_read(&[], "");
	    }

	    #[test]
	    fn read_something_test() {
	        assert_read(&["hello world,".to_string(), "bye world".to_string()], "hello world,\nbye world");
	    }

	    fn assert_read(expected: &[String], input: &str) {
	        let mock_read = Cursor::new(input);
	        let lines = read_lines(mock_read);
	        assert_eq!(expected.to_owned(), lines);
	    }
	}

	#[cfg(test)]
	mod parse_line_tests {
	    use super::parse_line;
	   		
	    #[test]	
	    fn parse_empty_test() {
	    	assert_parse(&[], "");
	    	assert_parse(&[], " \n \t");
	    }

	    #[test]
	    fn parse_whitespace_test() {
	        assert_parse(&["hello".to_string(), "world".to_string(), "wow".to_string()], 
	        	" hello world\twow ");
	    }

	    #[test]
		fn parse_punctuation_test() {
	        assert_parse(&["hello".to_string(), "world".to_string(), "It's".to_string(), "me".to_string()], 
	        	" hello, world! \tIt's me! ");
	    }

	 	fn assert_parse(expected: &[String], input: &str) {
	        let parsed = parse_line(input);
	        assert_eq!(expected.to_owned(), parsed);
	    }
	}

	#[cfg(test)]
	mod get_words_tests {
		use super::get_words;

		#[test]
		fn empty_line_test() {
			let expected: Vec<String> = vec![];
			assert_eq!(expected, get_words(vec![]));
		}

		#[test]
		fn whitespace_line_test() {
			let expected: Vec<String> = vec![];
			assert_eq!(expected, get_words(vec![" \t \n ".to_string()]));
		}

		#[test]
		fn one_line_test() {
			assert_eq!(vec!["hello".to_string(), "world".to_string()], 
				get_words(vec!["hello world!".to_string()]));
		}

		#[test]
		fn multi_line_test() {
			assert_eq!(vec!["hello".to_string(), "world".to_string(), "It's".to_string(), "me".to_string(), "Mario".to_string()],
				get_words(vec!["hello world!".to_string(), "It's me Mario!!!!".to_string()]));
		}
	}

	#[cfg(test)]
	mod count_words_tests {
		use super::count_words;
		use std::collections::HashMap;

		#[test]	
		fn empty_words_test() {
			let words = vec![];
			let expected: HashMap<String, usize> = HashMap::new();
			assert_eq!(expected, count_words(words));
		}

		#[test]
		fn test_unique() {
			let words = vec!["hello".to_string(), "world".to_string(), "It's".to_string(), "me".to_string(), "Mario".to_string()];
			let mut expected: HashMap<String, usize> = HashMap::new();
			
			expected.insert("hello".to_string(), 1);
			expected.insert("world".to_string(), 1);
			expected.insert("it's".to_string(), 1);
			expected.insert("me".to_string(), 1);
			expected.insert("mario".to_string(), 1);

			assert_eq!(expected, count_words(words));
		}

		#[test]
		fn test_duplicates() {
			let words = vec!["hello".to_string(), "world".to_string(), "It's".to_string(), "me".to_string(), "Mario".to_string(),
							 "hello".to_string(), "world".to_string(), "It's".to_string(), "me".to_string(), "Mario".to_string()];
			let mut expected: HashMap<String, usize> = HashMap::new();
			
			expected.insert("hello".to_string(), 2);
			expected.insert("world".to_string(), 2);
			expected.insert("it's".to_string(), 2);
			expected.insert("me".to_string(), 2);
			expected.insert("mario".to_string(), 2);

			assert_eq!(expected, count_words(words));
		}
	}	
}
