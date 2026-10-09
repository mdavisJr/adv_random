use crate::random::CurrentData;
use crate::rules::{
    IsWithinErrorType, SharedData, RuleTrait, PoolType,
};
use std::any::Any;
use std::collections::HashSet;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Result};

use super::ExcludeRuleTrait;
use super::exclude_rule_trait::is_excluded_helper;

#[derive(Clone, Default)]
pub struct NumberPoolByIndex {
    number_pool_items: Vec<NumberPoolItemByIndex>,
    errors: Vec<String>,
}

#[derive(Clone, Debug)]
struct NumberPoolItemByIndex {
    pool: PoolType,
    indexes: HashSet<usize>,
}

impl NumberPoolByIndex {
    //No numbers have to come from a pool yet.  Say which ones with at.
    pub fn new() -> NumberPoolByIndex {
        return NumberPoolByIndex::default();
    }

    //The numbers at these indexes come from this pool.  The first number is index 0.
    pub fn at(mut self, indexes: impl IntoIterator<Item = usize>, pool: impl Into<PoolType>) -> NumberPoolByIndex {
        let pool: PoolType = pool.into();
        let indexes: HashSet<usize> = indexes.into_iter().collect();
        if pool.is_empty() {
            self.errors.push(format!("{} pool is empty", self.name()));
        }
        let mut sorted_indexes: Vec<usize> = indexes.iter().copied().collect();
        sorted_indexes.sort_unstable();
        for index in sorted_indexes {
            if self.number_pool_items.iter().any(|x| x.indexes.contains(&index)) {
                self.errors.push(format!("{} index: {} has more than one pool", self.name(), index));
            }
        }
        self.number_pool_items.push(NumberPoolItemByIndex { pool, indexes });
        return self;
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
                let number = number_pool_item.pool.random_number(current_data).ok_or_else(|| current_data.no_number_err())?;
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
        count: usize,
    ) -> std::result::Result<bool, String> {
        if let Some(index) = self.number_pool_items.iter().flat_map(|x| x.indexes.iter().copied()).filter(|x| *x >= count).min() {
            return Err(format!("{} index: {} is past the last number for count: {}", self.name(), index, count));
        }
        return Ok(true);
    }

    fn validate(&self, count: usize) -> Vec<String> {
        let mut errors = self.errors.clone();
        if let Err(e) = self.check_count(count) {
            errors.push(e);
        }
        return errors;
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

    fn validate_exclude(&self) -> Vec<String> {
        return self.errors.clone();
    }
}
