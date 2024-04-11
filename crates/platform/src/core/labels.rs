use std::collections::BTreeMap;
pub struct Labels {}

impl Labels {
	pub fn labels<'a>(values: Vec<(&'a str, &'a str)>) -> BTreeMap<String, String> {
		let mut labels: BTreeMap<String, String> = BTreeMap::new();

		for vals in values {
			labels.insert(vals.0.to_string(), vals.1.to_string());
		}

		labels
	}
}
