mod printer;
mod preprocessing;
mod tokenizer;
mod succession_tree;

use std::{fs::{File, create_dir_all, read}, io::Write, time::SystemTime, process::exit, path::Path};

use nanoserde::{SerBin, DeBin};
use succession_tree::{SuccessionTree, TokenGenerationError};

use crate::{printer::*, preprocessing::*, tokenizer::*};


fn main() {

	let printer = Printer { start: SystemTime::now() };

	let options = get_corpora_names();
	printer.print(&format!("Choose corpus:"));
	for (i, option) in options.iter().enumerate() {
		println!("{i}: {option}");
	}
	let corpus_id = printer.input().parse::<usize>().unwrap_or_else(|_| {printer.print("Invalid index, exiting..."); exit(0)});
	if corpus_id >= options.len() {
		printer.print("Invalid index, exiting..."); 
		exit(0);
	}
	
	printer.print("Input new tokens count:");
	let new_tokens = printer.input().parse::<usize>().unwrap_or_else(|_| {printer.print("Invalid number, exiting..."); exit(0)});

	printer.print("Input phrase length:");
	let phrase_length = printer.input().parse::<usize>().unwrap_or_else(|_| {printer.print("Invalid number, exiting..."); exit(0)});
	if phrase_length == 0 {
		printer.print("Invalid number, exiting..."); 
		exit(0);
	}


	let vocab: Vec<(Option<(usize, usize)>, String)> =
	if Path::new(&format!("./corpora/{}/data/{new_tokens}_tokens.bin", options[corpus_id])).exists() {
		printer.print(&format!("found previously created tokens at ./corpora/{}/data/{new_tokens}_tokens.bin, loading...", options[corpus_id]));

		let bytes = read(format!("./corpora/{}/data/{new_tokens}_tokens.bin", options[corpus_id]));
		let bytes = if let Ok(s) = bytes { s } else {
			printer.print("Error while reading the file, exiting...");
			exit(0);
		};


		if let Ok(v) = DeBin::deserialize_bin(&bytes) { v }
		else {
			printer.print("invalid file contents, exiting...");
			exit(0)
		}
	}
	else {
		printer.print("reading corpus and extracting word frequencies...");
		let words = corpus_to_words(&options[corpus_id]).unwrap_or_else(|e| {printer.print(&format!("Error: '{}', exiting...", e)); exit(0)});

		printer.print("producing starting tokens...");
		let mut vocab = words_to_vocab(&words);

		printer.print("tokenizing words...");
		let mut lexicon = shatter_words(&words, &vocab).unwrap();

		printer.print("generating new tokens...");
		generate_tokens(&mut vocab, &mut lexicon, new_tokens);

		printer.print("example words:");
		print_lexicon(&lexicon, &vocab, 10);
		println!();

		printer.print(&format!("saving vocabulary to ./corpora/{}/data/{new_tokens}_tokens.bin", options[corpus_id]));
		create_dir_all(&format!("./corpora/{}/data/", options[corpus_id])).unwrap();
		let mut file = File::create(format!("./corpora/{}/data/{new_tokens}_tokens.bin", options[corpus_id])).unwrap();
		let serialized = vocab.serialize_bin();
		file.write_all(&serialized).unwrap_or_else(|e| {printer.print(&format!("Filesystem error: '{}'. Exiting...", e)); exit(0)});
		drop(file);

		vocab
	};

	let tokenized: Vec<usize> =
	if Path::new(&format!("./corpora/{}/data/{new_tokens}_tokenization.bin", options[corpus_id])).exists() {
		printer.print(&format!("found previously created tokenization at ./corpora/{}/data/{new_tokens}_tokenization.bin, loading...", options[corpus_id]));
		
		let bytes = read(format!("./corpora/{}/data/{new_tokens}_tokenization.bin", options[corpus_id]));
		let bytes = if let Ok(s) = bytes { s } else {
			printer.print("Error while reading the file, exiting...");
			exit(0);
		};


		if let Ok(v) = DeBin::deserialize_bin(&bytes) { v }
		else {
			printer.print("invalid file contents, exiting...");
			exit(0)
		}
	}
	else {
		printer.print("tokenizing whole corpus");
		let tokenized = tokenize_corpus(&options[corpus_id], &vocab).unwrap_or_else(|e| {
			printer.print(&format!("Error while tokenizing: '{}'. Exiting...", e));
			exit(0);
		});

		printer.print(&format!("saving tokenization to ./corpora/{}/data/{new_tokens}_tokenization.bin", options[corpus_id]));
		let mut file = File::create(format!("./corpora/{}/data/{new_tokens}_tokenization.bin", options[corpus_id])).unwrap();
		let serialized = tokenized.serialize_bin();
		file.write_all(&serialized).unwrap_or_else(|e| {printer.print(&format!("Filesystem error: '{}'. Exiting...", e)); exit(0)});
		drop(file);

		tokenized
	};

	let tree: SuccessionTree =
	if Path::new(&format!("./corpora/{}/data/{new_tokens}_{phrase_length}_tree.bin", options[corpus_id])).exists() {
		printer.print(&format!("found previously created tree at ./corpora/{}/data/{new_tokens}_{phrase_length}_tree.bin, loading...", options[corpus_id]));
		
		let bytes = read(format!("./corpora/{}/data/{new_tokens}_{phrase_length}_tree.bin", options[corpus_id]));
		let bytes = if let Ok(s) = bytes { s } else {
			printer.print("Error while reading the file, exiting...");
			exit(0);
		};


		if let Ok(v) = DeBin::deserialize_bin(&bytes) { v }
		else {
			printer.print("invalid file contents, exiting...");
			exit(0)
		}
	}
	else {
		printer.print("generating succession tree...");
		let mut tree = SuccessionTree::new(vocab.len(), phrase_length + 1);
		for (i, tokens) in tokenized.windows(phrase_length + 1).enumerate() {
			print_progressbar(50, (i+1) as f32 / (tokenized.len() - phrase_length) as f32, "");
	
			if let Err(e) = tree.register(tokens) {
				printer.print(&e);
				exit(0);
			}
		}
		println!();
	
		printer.print(&format!("saving succession tree to ./corpora/{}/data/{new_tokens}_{phrase_length}_tree.bin", options[corpus_id]));
		let mut file = File::create(format!("./corpora/{}/data/{new_tokens}_{phrase_length}_tree.bin", options[corpus_id])).unwrap();
		let serialized = tree.serialize_bin();
		file.write_all(&serialized).unwrap_or_else(|e| {printer.print(&format!("Filesystem error: '{}'. Exiting...", e)); exit(0)});
		drop(file);

		tree
	};
	
	printer.print("Text to be continued:");
	let text = printer.input();
	let mut tokenized = tokenize_text(&text, &vocab, None).unwrap_or_else(|e| {printer.print(&format!("Error while tokenizing: {}, exiting...", e)); exit(0)});
	
	for _ in 0..1000 {
		let mut slice_boundary = if tokenized.len() < phrase_length {0} else {tokenized.len()-phrase_length};
		loop {
			let res = tree.next_token(&tokenized[slice_boundary..]);
			if let Ok(token) = res {
				tokenized.push(token);
				break;
			}
			else if let Err(e) = res {
				match e {
					TokenGenerationError::NoInput => {
						tokenized.push(rand::random::<usize>() % vocab.len());
						break;
					},
					TokenGenerationError::UnknownSequence => {
						slice_boundary += 1;
					}
				}
			}
		}
	}
	
	let tokenized_text = tokens_to_text(&tokenized, &vocab, false);
	printer.print(&format!("{}", tokenized_text));

}

// implement creativity
// look for optimizations - multithreading
// whitespace handling