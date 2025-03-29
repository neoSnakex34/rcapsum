use rand::prelude::*;
use std::collections::HashMap;

pub struct MarkovChainModel {
    state_size: usize,
    chain: HashMap<String, Vec<String>>,
}

impl MarkovChainModel {
    pub fn new(source: &str, state_size: usize) -> Self {
        let words: Vec<&str> = source.split_whitespace().collect();

        let mut chain = HashMap::new();

        for i in state_size..words.len() {
            let previous_word = words[i - state_size..i].join(" ");
            let current_word = words[i].to_string();

            chain
                .entry(previous_word)
                .or_insert_with(Vec::new)
                .push(current_word)
        }

        // println!("{:?}", chain);
        Self { state_size, chain }
    }

    pub fn generate(&self, min_len: usize) -> String {
        let mut rng = rand::rng();
        let mut output = self.get_random_starter(&mut rng);
        let mut i = self.state_size;

        while i <= min_len || !output.ends_with('.') {
            let last_words: Vec<_> = output
                .split_whitespace()
                .rev()
                .take(self.state_size)
                .collect();

            let key = last_words.into_iter().rev().collect::<Vec<_>>().join(" ");

            if let Some(next_words) = self.chain.get(&key) {
                let next_word = next_words.choose(&mut rng).unwrap();
                output.push(' ');
                output.push_str(next_word);
                i += 1;
            } else {
                output.push_str(" ");
                output.push_str(&self.get_random_starter(&mut rng));
            }
        }

        output
    }

    fn get_random_starter(&self, rng: &mut impl rand::Rng) -> String {
        let starters: Vec<_> = self
            .chain
            .keys()
            .filter(|k| k.starts_with(|c: char| c.is_uppercase()))
            .collect();

        // TODO choose a better fallback starter than '.'
        starters
            .choose(rng)
            .map(|s| s.to_string())
            .unwrap_or_else(|| String::from("."))
    }
}
