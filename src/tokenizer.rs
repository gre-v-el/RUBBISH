use std::{collections::HashMap, io::{stdout, Write}};


pub fn most_frequent_pair(lexicon: &Vec<(Vec<usize>, u32)>) -> Option<(usize, usize)> {
	let mut best_pair = (0, 0);
	let mut largest_frequency = 0;

	let mut frequencies = HashMap::new();

	for (entry, frequency) in lexicon {
		for pair in entry.windows(2) {
			let key = (pair[0], pair[1]);
			if let Some(f) = frequencies.get_mut(&key) {
				*f += frequency;
				if *f >= largest_frequency {
					best_pair = key;
					largest_frequency = *f;
				}
			}
			else {
				frequencies.insert(key, *frequency);
				if *frequency >= largest_frequency {
					best_pair = key;
					largest_frequency = *frequency;
				}
			}
		}
	}

	if largest_frequency == 0 {
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
	let bar_width = 50;
	for i in 0..new_tokens {
		let mut should_break = false;
		let pair = most_frequent_pair(&lexicon).unwrap_or_else(|| {
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

		let xs = ((i as f32 / new_tokens as f32) * bar_width as f32) as usize;
		print!("\r[{}{}] {}/{new_tokens}", "X".repeat(xs), " ".repeat(bar_width - xs), i+1);

		stdout().flush().unwrap();
	}
	println!("\n");
}

pub fn tokenize_text(text: &String, vocab: &Vec<(Option<(usize, usize)>, String)>) -> Result<Vec<usize>, String> {
	let text = text.to_lowercase();
	let mut border = 0;
	loop {
		border += 1;
		if vocab.get(border) == None { break; }
		else if let Some(_) = vocab.get(border).unwrap().0 { break; }
	}

	let starting_tokens = &vocab[..border];
	
	let mut out = Vec::new();

	for char in text.chars() {
		let index = starting_tokens.binary_search_by(|e| e.1.chars().next().unwrap().cmp(&char));

		let index = if let Ok(i) = index { i } else { return Err(format!("unknown token in input: {}", char)) };

		out.push(index);
	}

	for (i, (prev, token)) in vocab[border..].iter().enumerate() {
		let pair = if let Some(p) = prev {*p} else {return Err(format!("Token outside of order: '{}'", token))};
		merge_tokens_text(&mut out, pair, i+border);
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
