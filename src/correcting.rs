use std::string::String;
use std::collections::HashSet;

use counting::WordCounts;

type WordSet = HashSet<String>;

fn word_probability(word: &str, counts: &WordCounts) -> f64 {
	match counts.get(word) {
		Some(&count) => (count as f64) / (counts.len() as f64),
		_ => 0.
	}
}

fn known(words: &WordSet, counts: &WordCounts) -> WordSet {
	let mut known_words = WordSet::new();

	for w in words {
		if counts.contains_key(w) {
			known_words.insert(w.to_string()); 
		}
	} 

	return known_words;
}

struct WordSplits {
    word1: String,
    word2: String,
}

const LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyz";
fn splits(word: &str) -> Vec<WordSplits> {
    let mut splits: Vec<WordSplits> = Vec::new();
    for i in 0..word.chars().count()+1 {
        let mut word1 = "".to_string();
        let mut word2 = "".to_string();
        for j in 0..word.chars().count() {
            if j < i {word1 = word1 + word.chars().nth(j).unwrap().to_string().as_str();}
            else {word2 = word2 + word.chars().nth(j).unwrap().to_string().as_str();}
            //splits.append((word1, word2));       
        }
        let w = WordSplits { word1: word1.clone(), word2: word2.clone() };
        splits.push(w);
    }

    return splits;
}

fn deletes(splits: &Vec<WordSplits>, bucket: &mut WordSet) {
    for w in splits {
        let mut w2 = w.word1.clone();
        if !w.word2.is_empty() {
            //let mut w2 = w.word1 + w.word2[1:]
            for i in 1..w.word2.chars().count() {
                w2 = w2 + w.word2.chars().nth(i).unwrap().to_string().as_str();
            }
            bucket.insert(w2);
        }
    }
}

fn transposes(splits: &Vec<WordSplits>, bucket: &mut WordSet) {
    for w in splits {
        if w.word2.chars().count()>1 {
            let mut w2 = w.word1.clone();
            w2 = w2 + w.word2.chars().nth(1).unwrap().to_string().as_str();
            w2 = w2 + w.word2.chars().nth(0).unwrap().to_string().as_str();
            for i in 2..w.word2.chars().count() {
                w2 = w2 + w.word2.chars().nth(i).unwrap().to_string().as_str();
            }
            bucket.insert(w2);
        }
    }
}

fn replaces(splits: &Vec<WordSplits>, bucket: &mut WordSet) {
    for w in splits { 
        if !w.word2.is_empty() {
            for c in LETTERS.chars() {
                let mut w2 = w.word1.clone();
                w2 = w2 + c.to_string().as_str();
                for i in 1..w.word2.chars().count() {
                    w2 = w2 + w.word2.chars().nth(i).unwrap().to_string().as_str();
                }
                bucket.insert(w2);
            }   
        }
    }
}

fn inserts(splits: &Vec<WordSplits>, bucket: &mut WordSet) {
    for w in splits {
        if !w.word2.is_empty() {
            for c in LETTERS.chars() {
                let mut w2 = w.word1.clone();
                w2 = w2 + c.to_string().as_str();
                for i in 0..w.word2.chars().count() {
                    w2 = w2 + w.word2.chars().nth(i).unwrap().to_string().as_str();
                }
                bucket.insert(w2);
            }
        }
        else {
            for c in LETTERS.chars() {
                let mut w2 = w.word1.clone();
                w2 = w2 + c.to_string().as_str();
                bucket.insert(w2);
            }
        }
    }
}

fn edits1(word: &str) -> WordSet {
    let splits = splits(word);
    let mut edits = WordSet::new();
   
   	deletes(&splits, &mut edits);
   	transposes(&splits, &mut edits);
   	replaces(&splits, &mut edits);
   	inserts(&splits, &mut edits);    
    
    return edits;
}

fn edits2(word: &str) -> WordSet {
	let mut edits = WordSet::new();
	for e1 in edits1(word) {
		for e2 in edits1(e1.as_str()) {edits.insert(e2);}
	}
	edits
}

fn candidates(word: String, counts: &WordCounts) -> WordSet {
	let mut original_word = WordSet::new();
	original_word.insert(word.clone());
	let candidates = known(&original_word, counts);
	if candidates.len() > 0 {
		return candidates;
	}
	let edits1_words = edits1(&word);
	let candidates = known(&edits1_words, counts);
	if candidates.len() > 0 {return candidates;}
	let edits2_words = edits2(&word);
	let candidates = known(&edits2_words, counts);
	if candidates.len() > 0 {
	 	return candidates;
	}
	return original_word;
}

pub fn correction(word: &str, counts: &WordCounts) -> String {
	let mut max: f64 = -1.;
	let mut max_word = word.to_string();
	for c in candidates(word.to_string(), counts) {
		if word_probability(c.as_str(), counts) > max {
			max = word_probability(c.as_str(), counts);
			max_word = c.to_string().clone();
		}
	}
	max_word.to_string()
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
	use super::{WordCounts, WordSet, known};
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
		//let words = ["rust", "is", "hard"];
        let mut words = WordSet::new();
        words.insert("rust".to_string());
        words.insert("is".to_string());
        words.insert("hard".to_string());
		assert_eq!(HashSet::new(), known(&words, &counts));
	}

	#[test]
	fn one_known_test() {
		let counts = mock_counts();
		//let words = ["rust", "is", "hard", "apples"];
        let mut words = WordSet::new();
        words.insert("rust".to_string());
        words.insert("is".to_string());
        words.insert("hard".to_string());
        words.insert("apples".to_string());

		let mut expected = HashSet::new();
		expected.insert("apples".to_string());

		assert_eq!(expected, known(&words, &counts));
	}

	#[test]
	fn multiple_known_test() {
		let counts = mock_counts();
		//let words = ["rust", "is", "hard", "apples", "bananas", "cats", "dogs", "fish"];
        let mut words = WordSet::new();
        words.insert("rust".to_string());
        words.insert("is".to_string());
        words.insert("hard".to_string());
        words.insert("apples".to_string());
        words.insert("bananas".to_string());
        words.insert("cats".to_string());
        words.insert("dogs".to_string());
        words.insert("fish".to_string());

		let mut expected = HashSet::new();
		expected.insert("apples".to_string());
		expected.insert("bananas".to_string());
		expected.insert("cats".to_string());
		expected.insert("dogs".to_string());
		expected.insert("fish".to_string());

		assert_eq!(expected, known(&words, &counts));
	}
}
