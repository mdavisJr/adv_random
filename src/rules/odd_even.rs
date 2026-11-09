use crate::random::{CurrentData, NumberType, NO_RANDOM_ERR};
use crate::random_trait::get_random_vec_item;
use crate::rules::{SharedData, RuleTrait, IsWithinErrorType};
use std::any::Any;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Result};
use std::hash::Hash;

use super::ExcludeRuleTrait;
use super::exclude_rule_trait::is_excluded_helper;

#[derive(Clone, Copy, Default)]
pub struct OddEven {
    odd: usize,
    even: usize,
    //how many times odd and even were called, to find the ones that were called more than once
    odd_calls: u8,
    even_calls: u8,
}

impl OddEven {
    //No odd and no even numbers yet.  Say how many with odd and even.
    pub fn new() -> OddEven {
        return OddEven::default();
    }

    //How many of the numbers are odd
    pub fn odd(mut self, count: usize) -> OddEven {
        self.odd = count;
        self.odd_calls = self.odd_calls.saturating_add(1);
        return self;
    }

    //How many of the numbers are even
    pub fn even(mut self, count: usize) -> OddEven {
        self.even = count;
        self.even_calls = self.even_calls.saturating_add(1);
        return self;
    }

    pub fn from_numbers(numbers: &[usize]) -> OddEven {
        let mut odd: usize = 0_usize;
        let mut even: usize = 0_usize;
        for number in numbers {
            if OddEven::is_even(*number) {
                even += 1;
            } else {
                odd += 1;
            }
        }
        return OddEven { odd, even, odd_calls: 0, even_calls: 0 };
    }

    //The problems with how this was created
    fn chain_errors(&self) -> Vec<String> {
        let mut errors: Vec<String> = Vec::new();
        if self.odd_calls > 1 {
            errors.push(format!("{} odd was set more than once", self.name()));
        }
        if self.even_calls > 1 {
            errors.push(format!("{} even was set more than once", self.name()));
        }
        return errors;
    }

    pub fn odd_count(&self) -> usize {
        return self.odd;
    }

    pub fn even_count(&self) -> usize {
        return self.even;
    }

    pub fn is_odd(number: usize) -> bool {
        return !OddEven::is_even(number);
    }
    pub fn is_even(number: usize) -> bool {
        if number.is_multiple_of(2) {
            return true;
        }
        return false;
    }

    pub fn needs_even(&self, other: &Self) -> bool {
        return self.even < other.even;
    }

    pub fn needs_odd(&self, other: &Self) -> bool {
        return self.odd < other.odd;
    }

    pub fn needs_only_even(&self, other: &Self) -> bool {
        return self.odd == other.odd && self.even < other.even;
    }

    pub fn needs_only_odd(&self, other: &Self) -> bool {
        return self.even == other.even && self.odd < other.odd;
    }
}

impl Display for OddEven {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            "ODD:".to_owned() + &self.odd.to_string() + ",EVEN:" + &self.even.to_string()
        )
    }
}

impl Debug for OddEven {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum OddEvenKey {
    Odd,
    Even,
}

impl RuleTrait for OddEven {
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
        let act_odd_even = OddEven::from_numbers(current_data.selected_numbers());
        let mut pool_keys: Vec<OddEvenKey> = Vec::with_capacity(2);
        if act_odd_even.needs_even(self) {
            pool_keys.push(OddEvenKey::Even);
        }
        if act_odd_even.needs_odd(self) {
            pool_keys.push(OddEvenKey::Odd);
        }
        if pool_keys.is_empty() {
            return Ok(None);
        }
        let random = current_data.random().ok_or_else(|| NO_RANDOM_ERR.to_owned())?;
        let number_type = match get_random_vec_item(random, &pool_keys) {
            Some(OddEvenKey::Even) => NumberType::Even,
            _ => NumberType::Odd
        };
        return Ok(Some(vec![current_data.get_rand(number_type, None, None).ok_or_else(|| current_data.no_number_err())?]));
    }

    fn is_within_range(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<(), (IsWithinErrorType, String)> {
        let other = OddEven::from_numbers(current_data.selected_numbers());
        if other.odd > self.odd {
            return Err((IsWithinErrorType::Regular, "Too Many Odds".to_owned()));
        }
        if other.even > self.even {
            return Err((IsWithinErrorType::Regular, "Too Many Evens".to_owned()));
        }
        return Ok(());
    }

    fn is_match(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<(), String> {
        let other = OddEven::from_numbers(current_data.selected_numbers());
        if self.odd == other.odd && self.even == other.even {
            return Ok(());
        }
        return Err(format!(
            "Expected Odd: {} and Even: {}.  Actual Odd: {} and Even: {}.",
            self.odd, self.even, other.odd, other.even
        ));
    }

    //Its is_within_range only says no when nothing added could make the numbers match.
    //rules_only_stop_tickets_that_can_not_match checks it.
    fn can_still_match(&self, current_data: &CurrentData) -> bool {
        return self.is_within_range(current_data).is_ok();
    }

    fn name(&self) -> String {
        return String::from("OddEven");
    }

    fn check_count(
        &self,
        count: usize,
    ) -> std::result::Result<bool, String> {
        if (self.even + self.odd) == count {
            return Ok(true);
        }
        return Err(format!("Odd count: {} and Even count: {} must add up to count: {} ", self.odd, self.even, count));
    }

    fn validate(&self, count: usize) -> Vec<String> {
        let mut errors = self.chain_errors();
        if let Err(e) = self.check_count(count) {
            errors.push(e);
        }
        return errors;
    }
}

impl ExcludeRuleTrait for OddEven {
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
