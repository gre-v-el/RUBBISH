mod printer;
mod preprocessing;
mod tokenizer;
mod succession_tree;

use std::{fs::{File, create_dir_all, read_to_string}, io::Write, time::SystemTime, process::exit, path::Path};

use nanoserde::{SerRon, DeRon};

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

	let vocab: Vec<(Option<(usize, usize)>, String)> =
	if Path::new(&format!("./corpora/{}/data/{new_tokens}_tokens.ron", options[corpus_id])).exists() {
		printer.print(&format!("found previously created tokens at ./corpora/{}/data/{new_tokens}_tokens.ron, loading...", options[corpus_id]));

		let string = read_to_string(format!("./corpora/{}/data/{new_tokens}_tokens.ron", options[corpus_id]));
		let string = if let Ok(s) = string { s } else {
			printer.print("Error while reading the file, exiting...");
			exit(0);
		};


		if let Ok(v) = DeRon::deserialize_ron(&string) { v }
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

		printer.print(&format!("saving vocabulary to ./corpora/{}/data/{new_tokens}_tokens.ron", options[corpus_id]));
		create_dir_all(&format!("./corpora/{}/data/", options[corpus_id])).unwrap();
		let mut file = File::create(format!("./corpora/{}/data/{new_tokens}_tokens.ron", options[corpus_id])).unwrap();
		let serialized = vocab.serialize_ron();
		file.write_all(serialized.as_bytes()).unwrap_or_else(|e| {printer.print(&format!("Filesystem error: '{}'. Exiting...", e)); exit(0)});
		drop(file);

		vocab
	};

	let tokenized: Vec<usize> =
	if Path::new(&format!("./corpora/{}/data/{new_tokens}_tokenization.ron", options[corpus_id])).exists() {
		printer.print(&format!("found previously created tokenization at ./corpora/{}/data/{new_tokens}_tokenization.ron, loading...", options[corpus_id]));
		
		let string = read_to_string(format!("./corpora/{}/data/{new_tokens}_tokenization.ron", options[corpus_id]));
		let string = if let Ok(s) = string { s } else {
			printer.print("Error while reading the file, exiting...");
			exit(0);
		};


		if let Ok(v) = DeRon::deserialize_ron(&string) { v }
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

		printer.print(&format!("saving tokenization to ./corpora/{}/data/{new_tokens}_tokenization.ron", options[corpus_id]));
		let mut file = File::create(format!("./corpora/{}/data/{new_tokens}_tokenization.ron", options[corpus_id])).unwrap();
		let serialized = tokenized.serialize_ron();
		file.write_all(serialized.as_bytes()).unwrap_or_else(|e| {printer.print(&format!("Filesystem error: '{}'. Exiting...", e)); exit(0)});
		drop(file);

		tokenized
	};

	
	printer.print("Text to tokenize:");
	let text = printer.input();
	let tokenized = tokenize_text(&text, &vocab, None).unwrap_or_else(|e| {printer.print(&format!("Error while tokenizing: {}, exiting...", e)); exit(0)});
	let tokenized_text = tokens_to_text(&tokenized, &vocab, true);

	printer.print(&format!("{:?}", tokenized));
	printer.print(&format!("{}", tokenized_text));

}


/*
	TODO:
	* analyze
	* generate
 */