use std::{collections::HashMap, fs::{read_dir, read_to_string}, path::Path, ffi::OsStr};

use regex::Regex;

use crate::printer::print_progressbar;


pub fn most_frequent_pair(lexicon: &Vec<(Vec<usize>, u32)>, vocab: &Vec<(Option<(usize, usize)>, String)>) -> Option<(usize, usize)> {
	let mut best_pair = (0, 0);
	let mut best_frequency = 0;
	let mut best_length = 0;

	let mut frequencies = HashMap::new();

	for (entry, frequency) in lexicon {
		for pair in entry.windows(2) {
			let key = (pair[0], pair[1]);
			if let Some(f) = frequencies.get_mut(&key) {
				*f += frequency;
				if *f > best_frequency {
					best_pair = key;
					best_frequency = *f;
					best_length = vocab[key.0].1.len() + vocab[key.1].1.len();
				}
				else if *f == best_frequency && vocab[key.0].1.len() + vocab[key.1].1.len() < best_length {
					best_pair = key;
					best_frequency = *f;
					best_length = vocab[key.0].1.len() + vocab[key.1].1.len();
				}
			}
			else {
				frequencies.insert(key, *frequency);
				if *frequency >= best_frequency {
					best_pair = key;
					best_frequency = *frequency;
					best_length = vocab[key.0].1.len() + vocab[key.1].1.len();
				}
			}
		}
	}

	if best_frequency == 0 {
		None
	}
	else {
		Some(best_pair)
	}
}

pub fn merge_tokens_text(tokens: &mut Vec<usize>, pair: (usize, usize), new: usize) {
	let mut i = 0;
	let mut max = tokens.len()-1;
	while i < max {
		if tokens[i] == pair.0 && tokens[i+1] == pair.1 {
			tokens.remove(i+1);
			tokens[i] = new;
			max -= 1;
		}
		i += 1;
	}
}

pub fn merge_tokens_lexicon(lexicon: &mut Vec<(Vec<usize>, u32)>, pair: (usize, usize), new: usize) {

	for (entry, _) in lexicon {
		merge_tokens_text(entry, pair, new);
	}
}

pub fn generate_tokens(vocab: &mut Vec<(Option<(usize, usize)>, String)>, lexicon: &mut Vec<(Vec<usize>, u32)>, new_tokens: usize) {
	for i in 0..new_tokens {
		let mut should_break = false;
		let pair = most_frequent_pair(&lexicon, &vocab).unwrap_or_else(|| {
			should_break = true;
			println!("\n");
			println!("token no. {i} couldn't be created, as every word is a single token now");
			(0, 0)
		});
		if should_break { break; }
		let new_token = vocab[pair.0].1.clone() + &vocab[pair.1].1;
		vocab.push((Some(pair), new_token));
		let new_token_id = vocab.len() - 1;
		merge_tokens_lexicon(lexicon, pair, new_token_id);

		if i % 10 == 9 || i == new_tokens - 1 {
			print_progressbar(50, (i+1) as f32 / new_tokens as f32, &format!("{}/{new_tokens}", i+1));
		}
	}
	println!("");
}

pub fn tokenize_text(text: &str, vocab: &Vec<(Option<(usize, usize)>, String)>, debug: Option<&str>) -> Result<Vec<usize>, String> {
	let text = Regex::new("\\s+").unwrap().replace_all(&text.to_lowercase(), " ").into_owned();
	let mut border = 0;
	loop {
		border += 1;
		if vocab.get(border) == None { break; }
		else if let Some(_) = vocab.get(border).unwrap().0 { break; }
	}

	let starting_tokens = &vocab[..border];
	
	let mut out = Vec::new();

	let text_length = text.chars().count();
	for (i, char) in text.chars().enumerate() {
		if let Some(t) = debug {
			print_progressbar(50, (i+1) as f32 / text_length as f32, &format!("tokenizing: {t}"));
		}

		let index = starting_tokens.binary_search_by(|e| e.1.chars().next().unwrap().cmp(&char));

		let index = if let Ok(i) = index { i } else { return Err(format!("unknown token in input: '{}'", char)) };

		out.push(index);
	}

	for (i, (prev, token)) in vocab[border..].iter().enumerate() {
		if let Some(t) = debug {
			print_progressbar(50, (i+1) as f32 / (vocab.len()-border) as f32, &format!("merging: {t}   "));
		}
		let pair = if let Some(p) = prev {*p} else {return Err(format!("token outside of order: '{}'", token))};
		merge_tokens_text(&mut out, pair, i+border);
	}

	if debug.is_some() {
		println!()
	}

	Ok(out)
}

pub fn tokenize_corpus(folder: &str, vocab: &Vec<(Option<(usize, usize)>, String)>) -> Result<Vec<usize>, String> {
	let mut out = Vec::new();

	let paths = read_dir(format!(".\\corpora\\{folder}\\")).unwrap();

	for path in paths {
		let path = path.unwrap();
        if let Some("txt") = Path::new(&path.file_name()).extension().and_then(OsStr::to_str) {
			let content = read_to_string(format!(".\\corpora\\{folder}\\{}", path.file_name().to_str().unwrap()));
			if let Ok(content) = content {
				let tokens = tokenize_text(&content, vocab, Some(path.file_name().to_str().unwrap_or("unknown")))?;
				out.extend(tokens);
			}
			else {
				return Err(format!("Error while reading file: {}", path.file_name().to_str().unwrap()));
			}
		}
		else if path.metadata().unwrap().is_file() {
			return Err(format!("Unexpected file extension: {}", path.file_name().to_str().unwrap()));
		}
    }

	Ok(out)
}

pub fn tokens_to_text(tokens: &Vec<usize>, vocab: &Vec<(Option<(usize, usize)>, String)>, debug: bool) -> String {
	let mut out = String::new();
	if debug {
		out += "|";
		for t in tokens {
			out += &vocab[*t].1;
			out += "|";
		}
	}
	else {
		for t in tokens {
			out += &vocab[*t].1;
		}
	}

	out
}
