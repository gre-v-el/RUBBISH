use std::{fs::{File, read_to_string}, io::BufReader, collections::{HashSet, HashMap}, ops::Index};

fn corpus_to_words(corpus: &str) -> Vec<(String, u32)> {
	let mut words = HashMap::new();
	for word in corpus.split_whitespace() {
		words.insert(word, if let Some(v) = words.get(word) {*v+1} else {1});
	}
	
	let mut to_sort = words.into_iter().map(|(w, n)| (w.to_owned(), n)).collect::<Vec<(String, u32)>>();
	to_sort.sort_unstable_by_key(|(_, n)| { *n as i32 });
	to_sort.iter_mut().for_each(|(w, _)| {w.push(' ')});

	to_sort
}

fn words_to_vocab(words: &Vec<(String, u32)>) -> Vec<String> {
	let mut set = HashSet::new();

	for (word, _) in words {
		for char in word.chars() {
			set.insert(format!("{char}"));
		}
	}

	let mut v = set.into_iter().collect::<Vec<String>>();
	v.sort_unstable();
	v
}

// todo: binary search?
fn shatter_words(words: &Vec<(String, u32)>, vocab: &Vec<String>) -> Result<Vec<(Vec<usize>, u32)>, String> {
	let mut shattered = Vec::new();

	for (word, number) in words {
		let mut tokens = Vec::new();
		for c in word.chars() {
			let target_token = format!("{c}");
			let mut index = None;
			for (i, t) in vocab.iter().enumerate() {
				if *t == target_token {
					index = Some(i);
					break;
				}
			}
			if let Some(i) = index {
				tokens.push(i);
			}
			else {
				return Err(format!("shatter_words(): vocab doesn't contain the token '{c}' from the corpus."));
			}
		}
		shattered.push((tokens, *number));
	}

	Ok(shattered)
}

fn main() {
	let mut corpus = read_to_string(".\\corpora\\TOKE.txt").unwrap();
	corpus.make_ascii_lowercase();

	let words = corpus_to_words(&corpus);
	let vocab = words_to_vocab(&words);

	let lexicon = shatter_words(&words, &vocab).unwrap();

	for (entry, frequency) in lexicon {
		print!("{:?} = |", entry);
		for t in entry {
			print!("{}|", vocab[t]);
		}
		println!(": {frequency}");
	}
}
