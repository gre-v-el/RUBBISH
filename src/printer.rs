use std::{time::SystemTime, io::{stdin, stdout, Write}};


pub struct Printer {
	pub start: SystemTime
}

impl Printer {
	pub fn print(&self, str: &str) {
		println!("[{:.2}] {str}", self.start.elapsed().unwrap().as_secs_f32());
	}

	pub fn input(&self) -> String {
		print!("{}> ", " ".repeat(format!("[{:.2}", self.start.elapsed().unwrap().as_secs_f32()).len()));
		stdout().flush().unwrap();

		let mut str = String::new();
		stdin().read_line(&mut str).unwrap();
		str.trim().to_owned()
	}
}



pub fn print_vocab(vocab: &Vec<(Option<(usize, usize)>, String)>) {
	let row_width = 7;
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

pub fn print_lexicon(lexicon: &Vec<(Vec<usize>, u32)>, vocab: &Vec<(Option<(usize, usize)>, String)>, num: usize) {
	for i in 0..num {
		let (entry, frequency) = &lexicon[(i as f32 / num as f32 * lexicon.len() as f32) as usize];
		print!("{:?} = |", entry);
		for t in entry {
			print!("{}|", vocab[*t].1);
		}
		println!(": {frequency}");
	}
}