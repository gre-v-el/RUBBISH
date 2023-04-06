use std::collections::{HashMap, HashSet};


pub fn corpus_to_words(corpus: &str) -> Vec<(String, u32)> {
	let mut words = HashMap::new();
	for word in corpus.split_whitespace() {
		words.insert(word, if let Some(v) = words.get(word) {*v+1} else {1});
	}
	
	let mut to_sort = words.into_iter().map(|(w, n)| (w.to_owned(), n)).collect::<Vec<(String, u32)>>();
	to_sort.sort_unstable_by_key(|(_, n)| { *n as i32 });
	to_sort.iter_mut().for_each(|(w, _)| {w.push(' ')});

	to_sort
}

pub fn words_to_vocab(words: &Vec<(String, u32)>) -> Vec<(Option<(usize, usize)>, String)> {
	let mut set = HashSet::new();

	for (word, _) in words {
		for char in word.chars() {
			set.insert((None, format!("{char}")));
		}
	}

	let mut v = set.into_iter().collect::<Vec<(Option<(usize, usize)>, String)>>();
	v.sort_unstable_by(|p, q| {p.1.cmp(&q.1)});
	v
}

pub fn shatter_words(words: &Vec<(String, u32)>, vocab: &Vec<(Option<(usize, usize)>, String)>) -> Result<Vec<(Vec<usize>, u32)>, String> {
	let mut shattered = Vec::new();

	for (word, number) in words {
		let mut tokens = Vec::new();
		for c in word.chars() {
			let target_token = format!("{c}");
			let index = vocab.binary_search_by(|v| {v.1.cmp(&target_token)});
			if let Ok(i) = index {
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
