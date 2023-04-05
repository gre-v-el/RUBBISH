use std::{fs::{File, read_to_string}, io::{BufReader, stdout, Write}, collections::{HashSet, HashMap}, ops::Index, time::SystemTime};

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

fn words_to_vocab(words: &Vec<(String, u32)>) -> Vec<(Option<(usize, usize)>, String)> {
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

// todo: binary search?
fn shatter_words(words: &Vec<(String, u32)>, vocab: &Vec<(Option<(usize, usize)>, String)>) -> Result<Vec<(Vec<usize>, u32)>, String> {
	let mut shattered = Vec::new();

	for (word, number) in words {
		let mut tokens = Vec::new();
		for c in word.chars() {
			let target_token = format!("{c}");
			let mut index = None;
			for (i, (_, t)) in vocab.iter().enumerate() {
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

fn print_lexicon(lexicon: &Vec<(Vec<usize>, u32)>, vocab: &Vec<(Option<(usize, usize)>, String)>, percentage: f32) {
	for (entry, frequency) in &lexicon[(lexicon.len() as f32 * (1.0 - percentage)) as usize..] {
		print!("{:?} = |", entry);
		for t in entry {
			print!("{}|", vocab[*t].1);
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
				if *f >= largest_frequency {
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
		let mut i = 0;
		let mut max = entry.len()-1;
		while i < max {
			if entry[i] == pair.0 && entry[i+1] == pair.1 {
				entry.remove(i+1);
				entry[i] = new;
				max -= 1;
			}
			i += 1;
		}
	}
}

fn print_vocab(vocab: &Vec<(Option<(usize, usize)>, String)>) {
	for t in vocab {
		print!("'{}'   ", t.1);
	}
	println!()
}

fn main() {
	let start = SystemTime::now();

	println!("[{:.2}] reading corpus...", start.elapsed().unwrap().as_secs_f32());
	let corpus = read_to_string(".\\corpora\\PT.txt").unwrap();

	println!("[{:.2}] normalizing corpus...", start.elapsed().unwrap().as_secs_f32());
	let corpus = corpus.to_lowercase();

	println!("[{:.2}] extracting word frequencies...", start.elapsed().unwrap().as_secs_f32());
	let words = corpus_to_words(&corpus);

	println!("[{:.2}] producing starting tokens...", start.elapsed().unwrap().as_secs_f32());
	let mut vocab = words_to_vocab(&words);

	println!("[{:.2}] tokenizing words...", start.elapsed().unwrap().as_secs_f32());
	let mut lexicon = shatter_words(&words, &vocab).unwrap();

	// print_vocab(&vocab);
	// print_lexicon(&lexicon, &vocab);

	println!("[{:.2}] producing new tokens:", start.elapsed().unwrap().as_secs_f32());
	let new_tokens = 10;
	for i in 0..new_tokens {
		let pair = most_frequent_pair(&lexicon);
		let new_token = vocab[pair.0].1.clone() + &vocab[pair.1].1;
		vocab.push((Some(pair), new_token));
		let new_token_id = vocab.len() - 1;
		merge_tokens(&mut lexicon, pair, new_token_id);
		print!("\r[{}{}] {}/{new_tokens}", "X".repeat(i), " ".repeat(new_tokens - i), i+1);
		stdout().flush().unwrap();
	}

	println!("\n\n1/1000 of most frequent words:");
	print_lexicon(&lexicon, &vocab, 0.001);
	println!("\ntokens:");
	print_vocab(&vocab);
}
