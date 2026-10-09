use crate::random::{CurrentData, NumberType};
use crate::rules::{SharedData, RuleTrait, IsWithinErrorType};
use std::any::Any;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Clone)]
pub struct RandomNumber {}

impl Display for RandomNumber {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "RandomNumber")
    }
}

impl Debug for RandomNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl RuleTrait for RandomNumber {
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
        current_data: &CurrentData,
    ) -> std::result::Result<Option<Vec<usize>>, String> {
        return Ok(Some(vec![current_data.get_rand(NumberType::Any, None, None).ok_or_else(|| current_data.no_number_err())?]));
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
