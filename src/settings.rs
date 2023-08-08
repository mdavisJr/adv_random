use crate::random::CurrentData;
use crate::rules::{MapAnyValue, RuleTrait, RandomNumber, ExcludeRuleTrait};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Settings {
    expected_rules: Vec<Box<dyn RuleTrait>>,
    count: usize,
    exclude_rules: Option<Vec<Box<dyn ExcludeRuleTrait>>>
}

impl Settings {
    pub fn new(
        expected_rules: &[Box<dyn RuleTrait>],
        count: usize,
    ) -> Settings {
        return Settings::with_exclude_rules(expected_rules, count, None)
    }

    pub fn with_exclude_rules(
        expected_rules: &[Box<dyn RuleTrait>],
        count: usize,
        exclude_rules: Option<Vec<Box<dyn ExcludeRuleTrait>>>
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
            exclude_rules
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

    pub fn get_number_within_number_range(&self, current_data: &CurrentData) -> Result<Vec<usize>, String> {
        return self.expected_rules().iter().find(|x| x.name() == "RandomNumber").unwrap()
            .get_numbers(current_data);
    }    

    pub fn expected_rules(&self) -> &Vec<Box<dyn RuleTrait>> {
        return &self.expected_rules;
    }

    pub fn exclude_rules(&self) -> &Option<Vec<Box<dyn ExcludeRuleTrait>>> {
        return &self.exclude_rules;
    }

    pub fn count(&self) -> usize {
        return self.count;
    }
}
