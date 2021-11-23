use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Ape {
	pub image: String,
	pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Attribute {
	pub trait_type: String,
	pub value: String,
}

impl Display for Ape {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			self
				.attributes
				.iter()
				.map(|attr| { attr.to_string() })
				.collect::<Vec<String>>()
				.join(", ")
		)
	}
}

impl Display for Attribute {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", self.trait_type, self.value)
	}
}
