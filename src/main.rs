use std::{fs::{read_to_string, read_dir}, io::{stdout, Write, stdin}, collections::{HashSet, HashMap}, time::SystemTime, process::exit};

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

fn print_lexicon(lexicon: &Vec<(Vec<usize>, u32)>, vocab: &Vec<(Option<(usize, usize)>, String)>, num: usize) {
	for (entry, frequency) in &lexicon[(lexicon.len() - num)..] {
		print!("{:?} = |", entry);
		for t in entry {
			print!("{}|", vocab[*t].1);
		}
		println!(": {frequency}");
	}
}

fn most_frequent_pair(lexicon: &Vec<(Vec<usize>, u32)>) -> Option<(usize, usize)> {
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
	let row_width = 5;
	let mut longest_token = 1;
	for (_, t) in vocab {
		longest_token = longest_token.max(t.chars().count());
	}

	for (i, (_, t)) in vocab.iter().enumerate() {
		let token_len = t.chars().count();
		let padding = " ".repeat(longest_token - token_len);
		print!("'{t}'{padding} ");
		if i % row_width == row_width-1 {
			println!();
		}
	}
	println!();
}

fn tokenize_text(text: &String, vocab: Vec<(Option<(usize, usize)>, String)>) {
	for (previous, t) in vocab {
		if previous == None {

		}
	}
}

struct Printer {
	start: SystemTime
}

impl Printer {
	fn print(&self, str: &str) {
		println!("[{:.2}] {str}", self.start.elapsed().unwrap().as_secs_f32());
	}
}

fn main() {
	let printer = Printer { start: SystemTime::now() };

	let paths = read_dir(".\\corpora\\").unwrap();
	printer.print(&format!("Input corpus name from {:?}:", paths.filter_map(|e| {
		let str = e.unwrap().file_name().to_str().unwrap().trim().to_owned();

		if str.split(".txt").count() == 2 {
			Some(str.split(".txt").next().unwrap().to_owned())
		}
		else {
			None
		}
	}).collect::<Vec<String>>()));
	let corpus_name = {
		let mut str = String::new();
		stdin().read_line(&mut str).unwrap();
		str.trim().to_owned()
	};

	printer.print("reading corpus...");
	let corpus = read_to_string(format!(".\\corpora\\{corpus_name}.txt")).unwrap_or_else(|_| {printer.print("No such corpus, exiting..."); exit(0)});

	printer.print("normalizing corpus...");
	let corpus = corpus.to_lowercase();

	printer.print("extracting word frequencies...");
	let words = corpus_to_words(&corpus);

	printer.print("producing starting tokens...");
	let mut vocab = words_to_vocab(&words);

	printer.print("tokenizing words...");
	let mut lexicon = shatter_words(&words, &vocab).unwrap();

	printer.print("producing new tokens...");
	let new_tokens = 3000;
	let bar_width = 50;
	for i in 0..new_tokens {
		let mut should_break = false;
		let pair = most_frequent_pair(&lexicon).unwrap_or_else(|| {
			should_break = true;
			println!("\n");
			printer.print(&format!("token no. {i} couldn't be created, as every word is a single token now"));
			(0, 0)
		});
		if should_break { break; }
		let new_token = vocab[pair.0].1.clone() + &vocab[pair.1].1;
		vocab.push((Some(pair), new_token));
		let new_token_id = vocab.len() - 1;
		merge_tokens(&mut lexicon, pair, new_token_id);

		let xs = ((i as f32 / new_tokens as f32) * bar_width as f32) as usize;
		print!("\r[{}{}] {}/{new_tokens}", "X".repeat(xs), " ".repeat(bar_width - xs), i+1);

		stdout().flush().unwrap();
	}
	println!("\n");
	
	printer.print("tokens:");
	print_vocab(&vocab);
	
	println!("\n");
	printer.print("top 100 most frequent words:");
	print_lexicon(&lexicon, &vocab, 100);
}
