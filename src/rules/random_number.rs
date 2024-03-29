use crate::random::CurrentData;
use crate::random_trait::get_random_trait;
use crate::rules::{MapAnyValue, RuleTrait, IsWithinErrorType};
use crate::settings::Settings;
use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Clone)]
pub struct RandomNumber {}

impl RandomNumber {
    // pub fn get_numbers_by_shared_data(
    //     shared_data: &HashMap<String, HashMap<String, MapAnyValue>>,
    //     settings: &Settings,
    // ) -> Vec<usize> {
    //     let (min, max) = Settings::get_min_max("NumberRange", shared_data);
    //     return RandomNumber::get_numbers(min, max);
    // }

    // pub fn get_numbers(min: usize, max: usize) -> Vec<usize> {
    //     let mut rng = thread_rng();
    //     let range = Uniform::from(min..=max);
    //     let number = range.sample(&mut rng);

    //     return vec![number];
    // }
}

impl Display for RandomNumber {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "RandomNumber")
    }
}

impl Debug for RandomNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl RuleTrait for RandomNumber {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn share_data(
        &self,
        _current_data: &CurrentData,
    ) -> Option<HashMap<String, MapAnyValue>> {
        None
    }

    fn get_numbers(
        &self,
        current_data: &CurrentData,
    ) -> std::result::Result<Vec<usize>, String> {
        let (min, max) = Settings::get_min_max("NumberRange", current_data.shared_data());
        return Ok(vec![get_random_trait().get_number(min, max)]);
    }

    fn is_within_range(
        &self,
        _current_data: &CurrentData,
    ) -> std::result::Result<(), (IsWithinErrorType, String)> {
        return Ok(());
    }

    fn is_match(
        &self,
        _current_data: &CurrentData,
    ) -> std::result::Result<(), String> {
        return Ok(());
    }

    fn name(&self) -> String {
        return String::from("RandomNumber");
    }

    fn check_count(
        &self,
        _count: usize,
    ) -> std::result::Result<bool, String> {
        return Ok(true);
    }
}
