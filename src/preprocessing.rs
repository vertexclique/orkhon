use std::collections::HashMap;
use std::hash::Hash;
use crate::errors::*;
use std::collections::hash_map::Entry;
use std::fmt::Debug;
use tract_core::itertools::Itertools;

pub struct OrdinalEncoder<I>
where
    I: Eq + Hash + Clone + Debug + Ord
{
    seen: HashMap<I, usize>,
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
        x.into_iter().for_each(|e| {
            match self.seen.entry(e.to_owned()) {
                Entry::Vacant(_) => {
                    let entry_key = self.encoded.len();
                    self.encoded.push(entry_key);
                    self.seen.insert(e, entry_key);
                }
                _ => {}
            }
        });
        let seen = self.seen.iter().sorted_by(|a, b| a.0.cmp(&b.0)).map(|e| e.0.clone()).collect::<Vec<I>>();
        self.categories = seen;

        self
    }

    pub fn transform(&self, x: Vec<I>) -> Result<Vec<usize>> {
        x.iter().map(|r| {
            self.seen.get(r).map_or_else(|| {
                Err(OrkhonError::Preprocessing(format!("Found unknown category '{:?}' during transform", r)))
            }, |i|
                Ok(i.to_owned())
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
    use serde::*;
    use serde_json::Value;

    #[derive(Deserialize)]
    struct EncoderBlock {
        data: Vec<i32>,
    }

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
    fn test_ordinal_encoder_long_i32() {
        let content = std::fs::read_to_string("testdata/ordinal_encoder_data.json").unwrap();
        let v: EncoderBlock = serde_json::from_str(&*content).unwrap();

        let data = v.data;
        let enc = OrdinalEncoder::<i32>::new();

        let enc = enc.fit(data.clone());
        assert_eq!(enc.categories(), vec![1, 2, 5, 7, 35, 36]);

        let tcontent = std::fs::read_to_string("testdata/ordinal_encoder_transformed.json").unwrap();
        let v: EncoderBlock = serde_json::from_str(&*tcontent).unwrap();
        assert_eq!(enc.transform(data).unwrap(), v.data.iter().map(|e| *e as usize).collect::<Vec<usize>>());
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
