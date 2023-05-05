use crate::rules::{MapAnyValue, RuleTrait, IsWithinErrorType};
use crate::settings::Settings;
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Clone)]
pub struct RandomNumber {}

impl RandomNumber {
    pub fn get_numbers_by_shared_data(
        shared_data: &HashMap<String, HashMap<String, MapAnyValue>>,
    ) -> Vec<usize> {
        let (min, max) = Settings::get_min_max("NumberRange", shared_data);
        return RandomNumber::get_numbers(min, max);
    }

    pub fn get_numbers(min: usize, max: usize) -> Vec<usize> {
        let mut rng = thread_rng();
        let range = Uniform::from(min..=max);
        let number = range.sample(&mut rng);

        return vec![number];
    }
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
        shared_data: &HashMap<String, HashMap<String, MapAnyValue>>,
    ) -> std::result::Result<Vec<usize>, String> {
        return Ok(RandomNumber::get_numbers_by_shared_data(shared_data));
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
        _selected_numbers: &[usize],
        _settings: &Settings,
        _shared_data: &HashMap<String, HashMap<String, MapAnyValue>>,
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
