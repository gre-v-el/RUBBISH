mod printer;
mod preprocessing;
mod tokenizer;

use std::{fs::{read_to_string, read_dir}, io::{stdout, Write, stdin}, time::SystemTime, process::exit};

use crate::{printer::*, preprocessing::*, tokenizer::*};


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
	let new_tokens = 300;
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
		merge_tokens_lexicon(&mut lexicon, pair, new_token_id);

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
	println!();

	printer.print("Text to tokenize:");
	let text = {
		let mut str = String::new();
		stdin().read_line(&mut str).unwrap();
		str.trim().to_owned()
	};
	let tokenized = tokenize_text(&text, &vocab).unwrap_or_else(|e| {printer.print(&format!("Error while tokenizing: {}, exiting...", e)); exit(0)});
	let tokenized_text = tokens_to_text(&tokenized, &vocab, true);

	printer.print(&format!("{:?}", tokenized));
	printer.print(&format!("{}", tokenized_text));

}


/*
	TODO:
	* save
	* analyze
	* generate
 */