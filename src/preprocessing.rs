use std::collections::HashMap;
use std::hash::Hash;
use crate::errors::*;
use std::collections::hash_map::Entry;
use std::fmt::Debug;

pub struct OrdinalEncoder<I>
where
    I: Eq + Hash + Clone + Debug + Ord
{
    seen: HashMap<I, ()>,
    categories: Vec<I>,
    encoded: Vec<usize>
}

impl<I> OrdinalEncoder<I>
where
    I: Eq + Hash + Clone + Debug + Ord
{
    pub fn new() -> Self {
        Self { seen: HashMap::default(), categories: Vec::default(), encoded: Vec::default() }
    }

    pub fn categories(&self) -> Vec<I> {
        self.categories.clone()
    }

    pub fn fit(mut self, x: Vec<I>) -> Self {
        self.seen.clear();
        self.encoded.clear();
        self.categories.clear();
        x.into_iter().enumerate().for_each(|e| {
            match self.seen.entry(e.1.to_owned()) {
                Entry::Vacant(_) => {
                    self.encoded.push(e.0);
                    self.seen.insert(e.1, ());
                }
                _ => {}
            }
        });
        let mut seen: Vec<I> = self.seen.keys().cloned().collect();
        seen.sort();
        self.categories = seen;

        self
    }

    pub fn transform(&self, x: Vec<I>) -> Result<Vec<usize>> {
        x.iter().map(|r| {
            self.categories.iter().position(|e| e == r).map_or_else(|| {
                Err(OrkhonError::Preprocessing(format!("Found unknown category '{:?}' during transform", r)))
            }, |i|
                Ok(self.encoded[i])
            )
        }).collect()
    }

    pub fn inverse_transform(&self, y: Vec<usize>) -> Result<Vec<I>> {
        y.iter().map(|r| {
            self.encoded.iter().position(|e| e == r).map_or_else(|| {
                Err(OrkhonError::Preprocessing(format!("Found unknown label '{:?}' during transform", r)))
            }, |i|
               Ok(self.categories[i].to_owned())
            )
        }).collect()
    }
}

#[cfg(test)]
mod tests_preprocessing {
    use super::*;

    #[test]
    fn test_ordinal_encoder() {
        let data = vec!["Nonbinary", "Male", "Female", "Male", "Nonbinary"];
        let enc = OrdinalEncoder::new();

        let enc = enc.fit(data);
        assert_eq!(enc.categories(), vec!["Female", "Male", "Nonbinary"]);

        assert_eq!(enc.transform(vec!["Nonbinary", "Nonbinary", "Female"]).unwrap(), vec![2, 2, 0]);
        assert_eq!(enc.transform(vec!["Male", "Nonbinary", "Female"]).unwrap(), vec![1, 2, 0]);
    }

    #[test]
    #[should_panic = "Preprocessing(\"Found unknown category \\\'\\\"CategoryNA\\\"\\\' during transform\""]
    fn test_ordinal_encoder_unknown_category() {
        let data = vec!["Nonbinary", "Male", "Female", "Male", "Nonbinary"];
        let enc = OrdinalEncoder::new();

        let enc = enc.fit(data);
        assert_eq!(enc.categories(), vec!["Female", "Male", "Nonbinary"]);

        enc.transform(vec!["CategoryNA", "Nonbinary", "Female"]).unwrap();
    }

    #[test]
    #[should_panic = "Preprocessing(\"Found unknown label \\\'4\\\' during transform\")"]
    fn test_ordinal_encoder_unknown_label() {
        let data = vec!["Nonbinary", "Male", "Female", "Male", "Nonbinary"];
        let enc = OrdinalEncoder::new();

        let enc = enc.fit(data);
        assert_eq!(enc.categories(), vec!["Female", "Male", "Nonbinary"]);

        enc.inverse_transform(vec![4, 5, 6]).unwrap();
    }
}
