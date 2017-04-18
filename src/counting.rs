use std::io::{BufRead, BufReader, Read};
use std::string::String;
use std::collections::HashMap;
// use std::env;
// use std::fs::File;
// use std::io::prelude::*;

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