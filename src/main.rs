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

fn print_lexicon(lexicon: &Vec<(Vec<usize>, u32)>, vocab: &Vec<String>) {
	for (entry, frequency) in lexicon {
		print!("{:?} = |", entry);
		for t in entry {
			print!("{}|", vocab[*t]);
		}
		println!(": {frequency}");
	}
}

fn most_frequent_pair(lexicon: &Vec<(Vec<usize>, u32)>) -> (usize, usize) {
	let mut best_pair = (0, 0);
	let mut largest_frequency = 0;

	let mut frequencies = HashMap::new();

	for (entry, frequency) in lexicon {
		for pair in entry.windows(2) {
			let key = (pair[0], pair[1]);
			if let Some(f) = frequencies.get_mut(&key) {
				*f += frequency;
				if *f > largest_frequency {
					best_pair= key;
					largest_frequency = *f;
				}
			}
			else {
				frequencies.insert(key, *frequency);
			}

			frequencies.insert(key, if let Some(f) = frequencies.get(&key) {f+1} else {1});
		}
	}

	best_pair
}

fn merge_tokens(lexicon: &mut Vec<(Vec<usize>, u32)>, pair: (usize, usize), new: usize) {

	for (entry, _) in lexicon {
		for i in 0..(entry.len()-1) {
			if entry[i] == pair.0 && entry[i+1] == pair.1 {
				entry.remove(i+1);
				entry[i] = new;
			}
		}
	}
}

fn main() {
	let mut corpus = read_to_string(".\\corpora\\TOKE.txt").unwrap();
	corpus.make_ascii_lowercase();

	let words = corpus_to_words(&corpus);
	let mut vocab = words_to_vocab(&words);

	let mut lexicon = shatter_words(&words, &vocab).unwrap();


	let pair = most_frequent_pair(&lexicon);
	let new_token = vocab[pair.0].clone() + &vocab[pair.1];
	vocab.push(new_token);
	let new_token_id = vocab.len() - 1;
	merge_tokens(&mut lexicon, pair, new_token_id);

	print_lexicon(&lexicon, &vocab);

}
