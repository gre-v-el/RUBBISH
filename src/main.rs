mod printer;
mod preprocessing;
mod tokenizer;

use std::{fs::{read_dir, File}, io::{stdout, Write, stdin, BufWriter}, time::SystemTime, process::exit};

use crate::{printer::*, preprocessing::*, tokenizer::*};


fn main() {
	let printer = Printer { start: SystemTime::now() };

	let options = get_corpora_names();
	printer.print(&format!("Choose corpus:"));
	for (i, option) in options.iter().enumerate() {
		println!("{i}: {option}");
	}
	let corpus_id = printer.input().parse::<usize>().unwrap_or_else(|_| {printer.print("Invalid index, exiting..."); exit(0)});

	printer.print("reading corpus and extracting word frequencies...");
	let words = corpus_to_words(&options[corpus_id]).unwrap_or_else(|e| {printer.print(&format!("Error: {}...", e)); exit(0)});

	printer.print("producing starting tokens...");
	let mut vocab = words_to_vocab(&words);

	printer.print("tokenizing words...");
	let mut lexicon = shatter_words(&words, &vocab).unwrap();

	printer.print("Input new tokens count:");
	let new_tokens = printer.input().parse::<usize>().unwrap_or_else(|_| {printer.print("Invalid number, exiting..."); exit(0)});
	generate_tokens(&mut vocab, &mut lexicon, new_tokens);
	
	printer.print("tokens:");
	print_vocab(&vocab);
	
	println!("\n");
	printer.print("example words:");
	print_lexicon(&lexicon, &vocab, 20);
	println!();

	printer.print(&format!("saving vocabulary to ./vocabs/{}_{new_tokens}.csv", options[corpus_id]));
	let file = File::create(format!("./vocabs/{}_{new_tokens}.csv", options[corpus_id])).unwrap();
	



	printer.print("Text to tokenize:");
	let text = printer.input();
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