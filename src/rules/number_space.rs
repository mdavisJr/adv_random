
use crate::random::CurrentData;
use crate::rules::{MapAnyValue, RuleTrait, IsWithinErrorType};
use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Result};

use super::ExcludeRuleTrait;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NumberSpaceType {
    Lt,
    Lte,
    Eq,
    Gte,
    Gt,
}

impl Display for NumberSpaceType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            NumberSpaceType::Gte => write!(f, "Gte"),
            NumberSpaceType::Eq => write!(f, "Eq"),
            NumberSpaceType::Lt => write!(f, "Lt"),
            NumberSpaceType::Lte => write!(f, "Lte"),
            NumberSpaceType::Gt => write!(f, "Gt"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct NumberSpace {
    number_space_type: NumberSpaceType,
    value: usize,
}

impl NumberSpace {
    pub fn new(number_space_type: NumberSpaceType, value: usize) -> NumberSpace {
        return NumberSpace { number_space_type, value };
    }

    pub fn value(&self) -> usize {
        return self.value;
    }

    pub fn number_space_type(&self) -> NumberSpaceType {
        return self.number_space_type;
    }
}

impl Display for NumberSpace {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            "TYPE:".to_owned() + &self.number_space_type.to_string() + ",VALUE:" + &self.value.to_string()
        )
    }
}

impl Debug for NumberSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl RuleTrait for NumberSpace {
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
        current_data: &CurrentData
    ) -> std::result::Result<Vec<usize>, String> {
        if self.number_space_type == NumberSpaceType::Eq {
            let mut numbers: Vec<usize> = vec![];

            let mut number= 0;
            if current_data.selected_numbers().len() == 0 {
                number = current_data.settings().get_number_within_number_range(current_data).unwrap()[0];
            } else if current_data.selected_numbers().len() > 1 {
                number = *current_data.selected_numbers().iter().max().unwrap(); //TODO use selected_numbers_sorted in future release 
            }
            
            while (numbers.len() + current_data.selected_numbers().len()) < current_data.settings().count() {
                number = number + self.value;
                numbers.push(number);
            }
        }
        return Err(String::from("Skip"));
    }

    fn is_within_range(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<(), (IsWithinErrorType, String)> {

        let mut i = 1;
        while i < current_data.selected_numbers_sorted().len() {                
            let num_space = current_data.selected_numbers_sorted()[i] - current_data.selected_numbers_sorted()[i-1];
            let err_str = "Expected ".to_owned() + &self.to_string() + "; Actual Value" + &num_space.to_string();
            match self.number_space_type {
                NumberSpaceType::Lt => {
                    if num_space >= self.value {
                        return Err((IsWithinErrorType::Regular, err_str));
                    }
                },
                NumberSpaceType::Lte => {
                    if num_space > self.value {
                        return Err((IsWithinErrorType::Regular, err_str));
                    }
                },
                NumberSpaceType::Eq => {
                    if num_space != self.value {
                        return Err((IsWithinErrorType::Regular, err_str));
                    }
                },
                NumberSpaceType::Gte => {
                    if num_space < self.value {
                        return Err((IsWithinErrorType::Regular, err_str));
                    }
                },                
                NumberSpaceType::Gt => {
                    if num_space <= self.value {
                        return Err((IsWithinErrorType::Regular, err_str));
                    }
                }
            }
            i += 1;
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
        return String::from("NumberSpace");
    }

    fn check_count(
        &self,
        _count: usize,
    ) -> std::result::Result<bool, String> {
        return Ok(true);
    }
}

impl ExcludeRuleTrait for NumberSpace {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_excluded(
        &self,
        current_data: &CurrentData,
    ) -> std::result::Result<(), String> {
        match self.is_within_excluded_range(current_data) {
            Ok(()) => Ok(()),
            Err(e) => Err(e.1)
        }
    }

    fn is_within_excluded_range(
        &self,
        current_data: &CurrentData,
    ) -> std::result::Result<(), (IsWithinErrorType, String)> {
        let mut i = 1;
        while i < current_data.selected_numbers_sorted().len() {                
            let num_space = current_data.selected_numbers_sorted()[i] - current_data.selected_numbers_sorted()[i-1];
            let err_str = "Expected ".to_owned() + &self.to_string() + "; Actual Value" + &num_space.to_string();
            match self.number_space_type {
                NumberSpaceType::Lt => {
                    if num_space < self.value {
                        return Err((IsWithinErrorType::Regular, err_str));
                    }
                },
                NumberSpaceType::Lte => {
                    if num_space <= self.value {
                        return Err((IsWithinErrorType::Regular, err_str));
                    }
                },
                NumberSpaceType::Eq => {
                    if num_space == self.value {
                        return Err((IsWithinErrorType::Regular, err_str));
                    }
                },
                NumberSpaceType::Gte => {
                    if num_space >= self.value {
                        return Err((IsWithinErrorType::Regular, err_str));
                    }
                },                
                NumberSpaceType::Gt => {
                    if num_space > self.value {
                        return Err((IsWithinErrorType::Regular, err_str));
                    }
                }
            }
            i += 1;
        }

        return Ok(());
    }

    fn exclude_name(&self) -> String {
        return self.name();
    }
}
