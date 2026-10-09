use crate::random::CurrentData;
use crate::rules::{SharedData, RuleTrait, IsWithinErrorType};
use std::any::Any;
use std::collections::HashSet;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Result};

use super::ExcludeRuleTrait;
use super::exclude_rule_trait::is_excluded_helper;

#[derive(Clone, Default)]
pub struct ExcludeNumberSets {
    excluded_number_sets: HashSet<Vec<usize>>,
}

impl ExcludeNumberSets {
    //Nothing is excluded yet.  Say what is with numbers and string.
    pub fn new() -> ExcludeNumberSets {
        return ExcludeNumberSets::default();
    }

    //The random numbers can not be these numbers in this order
    pub fn numbers(mut self, numbers: &[usize]) -> ExcludeNumberSets {
        self.excluded_number_sets.insert(numbers.to_vec());
        return self;
    }

    //Excludes every list of numbers.  The same as calling numbers for each one.
    pub fn numbers_each(mut self, number_sets: impl IntoIterator<Item = Vec<usize>>) -> ExcludeNumberSets {
        self.excluded_number_sets.extend(number_sets);
        return self;
    }

    //The random string can not be this string
    pub fn string(mut self, string: &str) -> ExcludeNumberSets {
        self.excluded_number_sets.insert(string.chars().map(|c| c as usize).collect());
        return self;
    }

    //Excludes every string.  The same as calling string for each one.
    pub fn string_each<T: AsRef<str>>(mut self, strings: impl IntoIterator<Item = T>) -> ExcludeNumberSets {
        for string in strings {
            self = self.string(string.as_ref());
        }
        return self;
    }

    #[deprecated(since = "3.0.0", note = "use ExcludeNumberSets::new().string(...) or .string_each(...)")]
    pub fn new_string(excluded_sets: &HashSet<String>) -> ExcludeNumberSets {
        return ExcludeNumberSets::new().string_each(excluded_sets);
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
        write!(f, "{}", self)
    }
}

impl RuleTrait for ExcludeNumberSets {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn share_data(
        &self,
        _current_data: &CurrentData,
    ) -> Option<SharedData> {
        None
    }

    fn get_numbers(
        &self,
        _current_data: &CurrentData
    ) -> std::result::Result<Option<Vec<usize>>, String> {
        return Ok(None);
    }

    fn is_within_range(
        &self,
        _current_data: &CurrentData
    ) -> std::result::Result<(), (IsWithinErrorType, String)> {
        return Ok(());
    }

    fn is_match(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<(), String> {
        if self.excluded_number_sets.contains(current_data.selected_numbers()) {
            return Err(format!(
                "Excluded Number Set found: {:?}",
                current_data.selected_numbers()
            ));
        }
        return Ok(());
    }

    fn name(&self) -> String {
        return String::from("ExcludeNumberSets");
    }

    fn check_count(
        &self,
        count: usize,
    ) -> std::result::Result<bool, String> {
        //sorted so the same one is reported every time
        let mut wrong_lengths: Vec<&Vec<usize>> = self.excluded_number_sets.iter().filter(|x| x.len() != count).collect();
        wrong_lengths.sort();
        if let Some(numbers) = wrong_lengths.first() {
            return Err(format!(
                "{} {:?} has {} numbers, so it can never be the {} random numbers",
                self.name(), numbers, numbers.len(), count
            ));
        }
        return Ok(true);
    }
}

impl ExcludeRuleTrait for ExcludeNumberSets {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_excluded(
        &self,
        current_data: &CurrentData,
    ) -> std::result::Result<(), String> {
        return is_excluded_helper(&self.is_match(current_data), &self.to_string());
    }

    fn is_within_excluded_range(
        &self,
        _current_data: &CurrentData,
    ) -> std::result::Result<(), (IsWithinErrorType, String)> {
        return Ok(());
    }

    fn exclude_name(&self) -> String {
        return self.name();
    }
}
