use crate::rules::{MapAnyValue, RuleTrait, IsWithinErrorType};
use crate::settings::Settings;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Clone)]
pub struct ExcludeNumberSets {
    excluded_number_sets: HashSet<Vec<usize>>,
}

impl ExcludeNumberSets {
    pub fn new(excluded_number_sets: &HashSet<Vec<usize>>) -> ExcludeNumberSets {
        return ExcludeNumberSets {
            excluded_number_sets: excluded_number_sets.clone(),
        };
    }

    pub fn new_string(excluded_sets: &HashSet<String>) -> ExcludeNumberSets {
        return ExcludeNumberSets { 
            excluded_number_sets: excluded_sets.iter()
            .map(|s| s.chars().map(|c| c as usize).collect::<Vec<usize>>()).collect::<HashSet<Vec<usize>>>() 
        };
    }

    pub fn excluded_number_sets(&self) -> &HashSet<Vec<usize>> {
        return &self.excluded_number_sets;
    }
}

impl Display for ExcludeNumberSets {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ExcludeNumberSets")
    }
}

impl Debug for ExcludeNumberSets {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl RuleTrait for ExcludeNumberSets {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn share_data(
        &self,
        _selected_numbers_set: &HashSet<usize>,
        _selected_numbers: &[usize],
        _settings: &Settings,
    ) -> Option<HashMap<String, MapAnyValue>> {
        None
    }

    fn get_numbers(
        &self,
        _selected_numbers_set: &HashSet<usize>,
        _selected_numbers: &[usize],
        _settings: &Settings,
        _shared_data: &HashMap<String, HashMap<String, MapAnyValue>>,
    ) -> std::result::Result<Vec<usize>, String> {
        return Err(String::from("Skip"));
    }

    fn is_within_range(
        &self,
        _selected_numbers_set: &HashSet<usize>,
        _selected_numbers: &[usize],
        _settings: &Settings,
        _shared_data: &HashMap<String, HashMap<String, MapAnyValue>>,
    ) -> std::result::Result<(), (IsWithinErrorType, String)> {
        return Ok(());
    }

    fn is_match(
        &self,
        _selected_numbers_set: &HashSet<usize>,
        selected_numbers: &[usize],
        _settings: &Settings,
        _shared_data: &HashMap<String, HashMap<String, MapAnyValue>>,
    ) -> std::result::Result<(), String> {
        if self.excluded_number_sets.contains(selected_numbers) {
            return Err(format!(
                "Excluded Number Set found: {:?}",
                selected_numbers
            ));
        }
        return Ok(());
    }

    fn name(&self) -> String {
        return String::from("ExcludeNumberSets");
    }

    fn check_count(
        &self,
        _count: usize,
    ) -> std::result::Result<bool, String> {
        return Ok(true);
    }
}
