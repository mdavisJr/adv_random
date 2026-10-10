use crate::random::{CurrentData, NumberType};
use crate::rules::{IsWithinErrorType, SharedData, RuleTrait, OddEvenKey, OddEven};
use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Result};

use super::ExcludeRuleTrait;
use super::exclude_rule_trait::is_excluded_helper;

#[derive(Clone, Default)]
pub struct OddEvenByIndex {
    odd_even: HashMap<usize, OddEvenKey>,
    errors: Vec<String>,
}

impl OddEvenByIndex {
    //No numbers have to be odd or even yet.  Say which ones with odd and even.
    pub fn new() -> OddEvenByIndex {
        return OddEvenByIndex::default();
    }

    //The numbers at these indexes are odd.  The first number is index 0.
    pub fn odd(self, indexes: impl IntoIterator<Item = usize>) -> OddEvenByIndex {
        return self.add(indexes, OddEvenKey::Odd);
    }

    //The numbers at these indexes are even.  The first number is index 0.
    pub fn even(self, indexes: impl IntoIterator<Item = usize>) -> OddEvenByIndex {
        return self.add(indexes, OddEvenKey::Even);
    }

    //The problems with how this was created.  Nothing added to it is one of them, because then it does nothing.
    fn chain_errors(&self) -> Vec<String> {
        let mut errors = self.errors.clone();
        if errors.is_empty() && self.odd_even.is_empty() {
            errors.push(format!("{} has no indexes", self.name()));
        }
        return errors;
    }

    fn add(mut self, indexes: impl IntoIterator<Item = usize>, odd_even_key: OddEvenKey) -> OddEvenByIndex {
        for index in indexes {
            if let Some(other) = self.odd_even.insert(index, odd_even_key) {
                if other != odd_even_key {
                    self.errors.push(format!("{} index: {} can not be both odd and even", self.name(), index));
                }
            }
        }
        return self;
    }

    pub fn from_numbers(numbers: &[usize]) -> OddEvenByIndex {
        let mut odd_even: HashMap<usize, OddEvenKey> = HashMap::new();
        for (idx, number) in numbers.iter().enumerate() {
            if OddEven::is_even(*number) {
                odd_even.insert(idx, OddEvenKey::Even);
            } else {
                odd_even.insert(idx, OddEvenKey::Odd);
            }
        }
        return OddEvenByIndex {
            odd_even,
            errors: Vec::new(),
        };
    }
}

impl Display for OddEvenByIndex {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{:?}",
            self.odd_even
        )
    }
}

impl Debug for OddEvenByIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl RuleTrait for OddEvenByIndex {
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
        current_data: &CurrentData
    ) -> std::result::Result<Option<Vec<usize>>, String> {
        if self.odd_even.contains_key(&current_data.selected_numbers().len()) {            
            if self.odd_even[&current_data.selected_numbers().len()] == OddEvenKey::Odd {
                return Ok(Some(vec![current_data.get_rand(NumberType::Odd, None, None).ok_or_else(|| current_data.no_number_err())?]));
            } else if self.odd_even[&current_data.selected_numbers().len()] == OddEvenKey::Even {
                return Ok(Some(vec![current_data.get_rand(NumberType::Even, None, None).ok_or_else(|| current_data.no_number_err())?]));
            } else {
                return Ok(None);
            }
        }
        return Ok(None);     
    }

    fn is_within_range(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<(), (IsWithinErrorType, String)> {
        for (idx, selected_number) in current_data.selected_numbers().iter().copied().enumerate() {
            if self.odd_even.contains_key(&idx) {
                let number_type: OddEvenKey = self.odd_even[&idx];
                if (number_type == OddEvenKey::Odd && !OddEven::is_odd(selected_number))
                    || (number_type == OddEvenKey::Even
                        && !OddEven::is_even(selected_number))
                {
                    return Err((
                        IsWithinErrorType::Regular,
                        format!(
                            "Selected number {} at index {} is not: {:?}. Numbers:{:?}",
                            selected_number, idx, number_type, current_data.selected_numbers()
                        ),
                    ));
                }
            }
        }
        return Ok(());
    }

    fn is_match(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<(), String> {
        match self.is_within_range(current_data) {
            Ok(()) => Ok(()),
            Err(e) => Err(e.1)
        }
    }

    fn name(&self) -> String {
        return String::from("OddEvenByIndex");
    }

    fn check_count(
        &self,
        count: usize,
    ) -> std::result::Result<bool, String> {
        if let Some(index) = self.odd_even.keys().copied().filter(|x| *x >= count).min() {
            return Err(format!("{} index: {} is past the last number for count: {}", self.name(), index, count));
        }
        return Ok(true);
    }

    fn validate(&self, count: usize) -> Vec<String> {
        let mut errors = self.chain_errors();
        if let Err(e) = self.check_count(count) {
            errors.push(e);
        }
        return errors;
    }
}

impl ExcludeRuleTrait for OddEvenByIndex {
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

    fn validate_exclude(&self) -> Vec<String> {
        return self.chain_errors();
    }
}
