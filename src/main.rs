use std::{fs::{File, read_to_string}, io::BufReader, collections::{HashSet, HashMap}};

fn list_words(corpus: &str) -> Vec<(String, u32)> {
	let mut words = HashMap::new();
	for word in corpus.split_whitespace() {
		words.insert(word, if let Some(v) = words.get(word) {*v+1} else {1});
	}
	
	let mut to_sort = words.into_iter().map(|(w, n)| (w.to_owned(), n)).collect::<Vec<(String, u32)>>();
	to_sort.sort_unstable_by_key(|(_, n)| { *n as i32 });
	to_sort
}

fn main() {
	let mut corpus = read_to_string(".\\corpora\\TOKE.txt").unwrap();
	corpus.make_ascii_lowercase();

	for (w, n) in list_words(&corpus) {
		println!("{w}: {n}");
	}


}
