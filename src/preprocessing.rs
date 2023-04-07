use std::{collections::{HashMap, HashSet}, fs::{read_dir, read_to_string, create_dir_all}, path::Path, ffi::OsStr};

pub fn get_corpora_names() -> Vec<String> {
	create_dir_all(".\\corpora\\").unwrap();
	let paths = read_dir(".\\corpora\\").unwrap();
	paths.filter_map(|e| {
		let e = e.unwrap();
		if e.metadata().unwrap().is_dir() {
			Some(e.file_name().into_string().unwrap())
		}
		else {
			None
		}
	}).collect::<Vec<String>>()
}

pub fn corpus_to_words(folder: &str) -> Result<Vec<(String, u32)>, String> {
	let mut words = HashMap::new();

	let paths = read_dir(format!(".\\corpora\\{folder}\\")).unwrap();

    for path in paths {
		let path = path.unwrap();
        if let Some("txt") = Path::new(&path.file_name()).extension().and_then(OsStr::to_str) {
			let content = read_to_string(format!(".\\corpora\\{folder}\\{}", path.file_name().to_str().unwrap()));
			if let Ok(content) = content {

				words.extend(string_to_words(&content.to_lowercase()));
			}
			else {
				return Err(format!("Non-txt file found: {}", path.file_name().to_str().unwrap()));
			}
		}
		else if path.metadata().unwrap().is_file() {
			return Err(format!("Error while reading file: {}", path.file_name().to_str().unwrap()));
		}
    }

	let mut to_sort = words.into_iter().collect::<Vec<(String, u32)>>();
	to_sort.sort_unstable_by_key(|(_, n)| { *n as i32 });
	to_sort.iter_mut().for_each(|(w, _)| {w.push(' ')});

	Ok(to_sort)
}


pub fn string_to_words(corpus: &str) -> HashMap<String, u32> {
	let mut words = HashMap::new();
	for word in corpus.split_whitespace() {
		words.insert(word.to_owned(), if let Some(v) = words.get(word) {*v+1} else {1});
	}
	
	words
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
