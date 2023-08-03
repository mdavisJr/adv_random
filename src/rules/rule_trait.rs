use crate::random::CurrentData;
use std::any::Any;
use std::collections::HashMap;
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
        current_data: &CurrentData,
    ) -> std::result::Result<(), (IsWithinErrorType, String)>;

    fn as_any(&self) -> &dyn Any;

    fn is_match(
        &self,
        current_data: &CurrentData,
    ) -> std::result::Result<(), String>;

    fn get_numbers(
        &self,
        current_data: &CurrentData,
    ) -> std::result::Result<Vec<usize>, String>;

    fn share_data(
        &self,
        current_data: &CurrentData,
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
