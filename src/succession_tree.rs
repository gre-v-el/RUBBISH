use nanoserde::{SerRon, DeRon};

#[derive(SerRon, DeRon)]
pub struct Node {
	value: usize,
	children: Vec<(Node, usize)> // child, strength
}

impl Node {
	pub fn new(value: usize) -> Self {
		Self { value, children: Vec::new() }
	}
}

#[derive(SerRon, DeRon)]
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
}