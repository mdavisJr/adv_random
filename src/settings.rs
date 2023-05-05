use crate::rules::{MapAnyValue, RuleTrait, RandomNumber};
use std::collections::{HashMap};

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Hash, Copy, Clone)]
pub enum UseNumberType {
    ALL,
    SomeOf(usize),
    Exclude,
}

#[derive(Clone)]
pub struct Settings {
    expected_rules: Vec<Box<dyn RuleTrait>>,
    count: usize,
}

impl Settings {
    pub fn new(
        expected_rules: &[Box<dyn RuleTrait>],
        count: usize,
    ) -> Settings {
        let mut expected_rules_clone = expected_rules.to_vec();
        if expected_rules_clone.iter().all(|x| x.name() != "RandomNumber") {
            expected_rules_clone.push(Box::new(RandomNumber{}));
        }
        for rule in &expected_rules_clone {
            if let Some(e) = rule.check_count(count).err() {
                panic!("{}", e);
            }
        }
        return Settings {
            expected_rules: expected_rules_clone,
            count,
        };
    }

    pub fn get_min_max(
        key: &str,
        shared_data: &HashMap<String, HashMap<String, MapAnyValue>>,
    ) -> (usize, usize) {
        let mut min = usize::MIN;
        let mut max = usize::MAX;
        if let Some(number_range_data) = shared_data.get(key) {
            if let Some(MapAnyValue::Usize(v)) = number_range_data.get("min") {
                min = *v;
            }
            if let Some(MapAnyValue::Usize(v)) = number_range_data.get("max") {
                max = *v;
            }
        }
        return (min, max);
    }

    pub fn expected_rules(&self) -> &Vec<Box<dyn RuleTrait>> {
        return &self.expected_rules;
    }

    pub fn count(&self) -> usize {
        return self.count;
    }
}
