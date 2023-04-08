use nanoserde::{SerBin, DeBin};

#[derive(SerBin, DeBin)]
pub struct Node {
	value: usize,
	children: Vec<(Node, usize)> // child, strength
}

impl Node {
	pub fn new(value: usize) -> Self {
		Self { value, children: Vec::new() }
	}
}

#[derive(SerBin, DeBin)]
pub struct SuccessionTree {
	entry_points: Vec<Node>,
	depth: usize,
}

impl SuccessionTree {
	pub fn new(num_tokens: usize, depth: usize) -> Self {
		let mut entry_points = Vec::new();

		for i in 0..num_tokens {
			entry_points.push(Node::new(i));
		}

		Self { entry_points, depth }
	}

	pub fn register(&mut self, tokens: &[usize]) -> Result<(), String> {
		if tokens.len() != self.depth { return Err("Error while inserting tokens into the succession tree.".into()) }

		let mut current = &mut self.entry_points[tokens[0]];

		for token in &tokens[1..] {
			let next = current.children.binary_search_by(|(n, _)| n.value.cmp(token));
			if let Ok(id) = next {
				current.children[id].1 += 1;
				current = &mut current.children[id].0;
			}
			else if let Err(id) = next {
				current.children.insert(id, (Node::new(*token), 1));
				current = &mut current.children[id].0;
			}
		}

		Ok(())
	}

	pub fn next_token(&self, tokens: &[usize], creativity: f32) -> Result<usize, TokenGenerationError> {
		if creativity > 1.0 || creativity < 0.0 { return Err(TokenGenerationError::CreativityOutsideOfRange); }
		let creativity = 1.0/(0.1 + 0.9*creativity) - 1.0;


		if tokens.len() == 0 {
			return Err(TokenGenerationError::NoInput);
		}

		let depth = tokens.len().min(self.depth - 1);

		let mut current = &self.entry_points[tokens[0]];
		for i in 1..depth {
			let next = current.children.binary_search_by(|(n, _)| n.value.cmp(&tokens[i]));
			let next = if let Ok(n) = next { n } else { return Err(TokenGenerationError::UnknownSequence); };
			current = &current.children[next].0;
		}

		if current.children.len() == 0 { return Err(TokenGenerationError::UnknownSequence); }

		let mut sum_weights= 0.0;
		for (_, weight) in &current.children {
			sum_weights += (*weight as f32).powf(creativity);
		}

		let mut choice = rand::random::<f32>() * sum_weights;
		for (n, weight) in &current.children {
			choice -= (*weight as f32).powf(creativity);
			if choice < 0.0 {
				return Ok(n.value);
			}
		}

		unreachable!()
	}
}

pub enum TokenGenerationError {
	NoInput,
	UnknownSequence,
	CreativityOutsideOfRange,
}