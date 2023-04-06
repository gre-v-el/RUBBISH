use std::time::SystemTime;


pub struct Printer {
	pub start: SystemTime
}

impl Printer {
	pub fn print(&self, str: &str) {
		println!("[{:.2}] {str}", self.start.elapsed().unwrap().as_secs_f32());
	}
}



pub fn print_vocab(vocab: &Vec<(Option<(usize, usize)>, String)>) {
	let row_width = 5;
	let mut longest_token = 1;
	for (_, t) in vocab {
		longest_token = longest_token.max(t.chars().count());
	}

	for (i, (p, t)) in vocab.iter().enumerate() {
		let token_len = t.chars().count();
		let padding = " ".repeat(longest_token - token_len);
		print!("{:?}'{t}'{padding} ", p);
		if i % row_width == row_width-1 {
			println!();
		}
	}
	println!();
}

pub fn print_lexicon(lexicon: &Vec<(Vec<usize>, u32)>, vocab: &Vec<(Option<(usize, usize)>, String)>, num: usize) {
	for (entry, frequency) in &lexicon[(lexicon.len() - num)..] {
		print!("{:?} = |", entry);
		for t in entry {
			print!("{}|", vocab[*t].1);
		}
		println!(": {frequency}");
	}
}