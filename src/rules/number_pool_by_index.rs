use crate::rules::{
    IsWithinErrorType, MapAnyValue, RuleTrait, PoolType,
};
use crate::settings::Settings;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Clone)]
pub struct NumberPoolByIndex {
    number_pool_items: Vec<NumberPoolItemByIndex>,
}

#[derive(Clone, Debug)]
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
        _selected_numbers_set: &HashSet<usize>,
        _selected_numbers: &[usize],
        _settings: &Settings,
    ) -> Option<HashMap<String, MapAnyValue>> {
        None
    }

    fn get_numbers(
        &self,
        _selected_numbers_set: &HashSet<usize>,
        selected_numbers: &[usize],
        _settings: &Settings,
        _shared_data: &HashMap<String, HashMap<String, MapAnyValue>>,
    ) -> std::result::Result<Vec<usize>, String> {
        for number_pool_item in &self.number_pool_items {
            if number_pool_item.indexes.contains(&selected_numbers.len()) {
                let number = number_pool_item.pool.random_number();
                return Ok(vec![number]);
            }
        }
        return Err(String::from("Skip"));
    }

    fn is_within_range(
        &self,
        _selected_numbers_set: &HashSet<usize>,
        selected_numbers: &[usize],
        _settings: &Settings,
        _shared_data: &HashMap<String, HashMap<String, MapAnyValue>>,
    ) -> std::result::Result<(), (IsWithinErrorType, String)> {
        for (idx, selected_number) in selected_numbers.iter().copied().enumerate() {
            for number_pool_item in &self.number_pool_items {
                if number_pool_item.indexes.contains(&idx)
                    && !number_pool_item.pool.contains(selected_number)
                {
                    return Err((
                        IsWithinErrorType::Regular,
                        format!(
                            "Selected number {} at index {} is not: {:?}. Numbers:{:?}",
                            selected_number, idx, number_pool_item, selected_numbers
                        ),
                    ));
                }
            }
        }
        return Ok(());
    }

    fn is_match(
        &self,
        selected_numbers_set: &HashSet<usize>,
        selected_numbers: &[usize],
        settings: &Settings,
        shared_data: &HashMap<String, HashMap<String, MapAnyValue>>,
    ) -> std::result::Result<(), String> {
        match self.is_within_range(
            selected_numbers_set,
            selected_numbers,
            settings,
            shared_data,
        ) {
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
