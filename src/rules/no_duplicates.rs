use crate::random::CurrentData;
use crate::rules::{MapAnyValue, RuleTrait, IsWithinErrorType};
use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Clone)]
pub struct NoDuplicate {}

impl Display for NoDuplicate {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "No Duplicate")
    }
}

impl Debug for NoDuplicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl RuleTrait for NoDuplicate {
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
        _current_data: &CurrentData
    ) -> std::result::Result<Vec<usize>, String> {
        return Err(String::from("Skip"));
    }

    fn is_within_range(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<(), (IsWithinErrorType, String)> {
        if current_data.selected_numbers().len() != current_data.selected_numbers_set().len() {
            return Err((IsWithinErrorType::Regular, format!("Duplicate found in {:?}", current_data.selected_numbers())));
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
        return String::from("NoDuplicate");
    }

    fn check_count(
        &self,
        _count: usize,
    ) -> std::result::Result<bool, String> {
        return Ok(true);
    }
}
