use crate::settings::Settings;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

#[derive(Debug)]
pub enum MapAnyValue {
    String(String),
    Usize(usize),
}

#[derive(Debug, Eq, PartialEq)]
pub enum IsWithinErrorType {
    Regular,
    MakePriority,
}

pub trait RuleTrait: RuleTraitClone + Display + std::fmt::Debug {
    fn is_within_range(
        &self,
        selected_numbers_set: &HashSet<usize>,
        selected_numbers: &[usize],
        settings: &Settings,
        shared_data: &HashMap<String, HashMap<String, MapAnyValue>>,
    ) -> std::result::Result<(), (IsWithinErrorType, String)>;

    fn as_any(&self) -> &dyn Any;

    fn is_match(
        &self,
        selected_numbers_set: &HashSet<usize>,
        selected_numbers: &[usize],
        settings: &Settings,
        shared_data: &HashMap<String, HashMap<String, MapAnyValue>>,
    ) -> std::result::Result<(), String>;

    fn get_numbers(
        &self,
        selected_numbers_set: &HashSet<usize>,
        selected_numbers: &[usize],
        settings: &Settings,
        shared_data: &HashMap<String, HashMap<String, MapAnyValue>>,
    ) -> std::result::Result<Vec<usize>, String>;

    fn share_data(
        &self,
        selected_numbers_set: &HashSet<usize>,
        selected_numbers: &[usize],
        settings: &Settings,
    ) -> Option<HashMap<String, MapAnyValue>>;

    fn check_count(
        &self,
        count: usize,
    ) -> std::result::Result<bool, String>;

    fn name(&self) -> String;
}

pub trait RuleTraitClone {
    fn clone_box(&self) -> Box<dyn RuleTrait>;
}

impl<T> RuleTraitClone for T
where
    T: 'static + RuleTrait + Clone,
{
    fn clone_box(&self) -> Box<dyn RuleTrait> {
        Box::new(self.clone())
    }
}


impl Clone for Box<dyn RuleTrait> {
    fn clone(&self) -> Box<dyn RuleTrait> {
        self.clone_box()
    }
}
