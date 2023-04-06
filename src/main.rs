mod printer;
mod preprocessing;
mod tokenizer;

use std::{fs::{File, create_dir_all}, io::Write, time::SystemTime, process::exit};

use nanoserde::SerRon;

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


	printer.print("Text to tokenize:");
	let text = printer.input();
	let tokenized = tokenize_text(&text, &vocab, None).unwrap_or_else(|e| {printer.print(&format!("Error while tokenizing: {}, exiting...", e)); exit(0)});
	let tokenized_text = tokens_to_text(&tokenized, &vocab, true);

	printer.print(&format!("{:?}", tokenized));
	printer.print(&format!("{}", tokenized_text));

}


/*
	TODO:
	* read
	* analyze
	* generate
 */