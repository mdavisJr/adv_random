use crate::random::{CurrentData, NO_NUMBER_FOUND_ERR};
use crate::rules::{
    IsWithinErrorType, SharedData, RuleTrait, PoolType,
};
use std::any::Any;
use std::collections::HashSet;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Result};

use super::ExcludeRuleTrait;
use super::exclude_rule_trait::is_excluded_helper;

#[derive(Clone)]
pub struct NumberPoolByIndex {
    number_pool_items: Vec<NumberPoolItemByIndex>,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct NumberPoolItemByIndex {
    pool: PoolType,
    indexes: HashSet<usize>,
    key: String,
}

impl NumberPoolItemByIndex {
    pub fn new(key: &str, pool: &PoolType, indexes: &HashSet<usize>) -> NumberPoolItemByIndex {
        return NumberPoolItemByIndex {
            pool: pool.clone(),
            indexes: indexes.clone(),
            key: key.to_owned(),
        };
    }
}

impl NumberPoolByIndex {
    pub fn new(number_pool_items: Vec<NumberPoolItemByIndex>) -> NumberPoolByIndex {
        return NumberPoolByIndex {
            number_pool_items: number_pool_items.to_vec(),
        };
    }
}

impl Display for NumberPoolByIndex {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.number_pool_items)
    }
}

impl Debug for NumberPoolByIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl RuleTrait for NumberPoolByIndex {
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
        for number_pool_item in &self.number_pool_items {
            if number_pool_item.indexes.contains(&current_data.selected_numbers().len()) {
                let number = number_pool_item.pool.random_number(current_data).ok_or_else(|| NO_NUMBER_FOUND_ERR.to_owned())?;
                return Ok(Some(vec![number]));
            }
        }
        return Ok(None);
    }

    fn is_within_range(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<(), (IsWithinErrorType, String)> {
        for (idx, selected_number) in current_data.selected_numbers().iter().copied().enumerate() {
            for number_pool_item in &self.number_pool_items {
                if number_pool_item.indexes.contains(&idx)
                    && !number_pool_item.pool.contains(selected_number)
                {
                    return Err((
                        IsWithinErrorType::Regular,
                        format!(
                            "Selected number {} at index {} is not: {:?}. Numbers:{:?}",
                            selected_number, idx, number_pool_item, current_data.selected_numbers()
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
            Err(e) => Err(e.1),
        }
    }

    fn name(&self) -> String {
        return String::from("NumberPoolByIndex");
    }

    fn check_count(
        &self,
        _count: usize,
    ) -> std::result::Result<bool, String> {
        return Ok(true);
    }
}

impl ExcludeRuleTrait for NumberPoolByIndex {
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
