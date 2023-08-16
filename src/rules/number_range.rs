use crate::random::CurrentData;
use crate::rules::{MapAnyValue, RuleTrait, IsWithinErrorType};
use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Result};

use super::ExcludeRuleTrait;

fn is_within_range_helper(
    number_range: &NumberRange,
    current_data: &CurrentData,
    invert: bool
) -> std::result::Result<(), (IsWithinErrorType, String)> {
    for (idx, selected_number) in current_data.selected_numbers().iter().copied().enumerate() {
        let key = if number_range.use_0_idx_for_all {0} else {idx};
        if number_range.ranges.contains_key(&key) {
            let (min, max) = number_range.ranges[&key];
            if (!invert && (selected_number < min || selected_number > max)) || (invert && (selected_number >= min && selected_number <= max)) {
                return Err((IsWithinErrorType::Regular, format!(
                    "Invert: {} - Selected number {} at index {} is not within range of min: {} and max: {}. Numbers:{:?}.  Map Index:{}",
                    invert, selected_number, idx, min, max, current_data.selected_numbers(), key
                )));
            }
        }
    }
    return Ok(());
}

#[derive(Clone)]
pub struct NumberRange {
    ranges: HashMap<usize, (usize, usize)>,
    use_0_idx_for_all: bool
}

impl NumberRange {
    pub fn all(min: usize, max: usize) -> NumberRange {
        let mut ranges: HashMap<usize, (usize, usize)> = HashMap::new();
        ranges.insert(0, (min, max));
        return NumberRange { ranges, use_0_idx_for_all: true };
    }

    pub fn from_map(ranges: &[(&[usize], usize, usize)]) -> NumberRange {
        let mut ranges_map: HashMap<usize, (usize, usize)> = HashMap::new();
        for range in ranges {
            for idx in range.0 {
                ranges_map.insert(*idx, (range.1, range.2));    
            }            
        }
        return NumberRange {
            ranges: ranges_map.clone(),
            use_0_idx_for_all: false
        };
    }

    pub fn len(&self) -> usize {
        return self.ranges.len();
    }

    pub fn is_empty(&self) -> bool {
        return self.len() == 0;
    }
}

impl Display for NumberRange {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut keys = self.ranges.keys().copied().collect::<Vec<usize>>();
        keys.sort();
        write!(f, "{:?}", keys.iter().map(|k| (*k, self.ranges[k])).collect::<Vec<(usize, (usize, usize))>>())
    }
}

impl Debug for NumberRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl RuleTrait for NumberRange {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn share_data(
        &self,
        current_data: &CurrentData,
    ) -> Option<HashMap<String, MapAnyValue>> {        
        let key = if self.use_0_idx_for_all {0} else {current_data.selected_numbers().len()};
        if self.ranges.contains_key(&key) {
            let mut map: HashMap<String, MapAnyValue> = HashMap::new();
            let (min, max) = self.ranges[&key];
            map.insert(
                "min".to_owned(),
                MapAnyValue::Usize(min),
            );
            map.insert(
                "max".to_owned(),
                MapAnyValue::Usize(max),
            );
        
            return Option::from(map);
        }
        return None;
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
        return is_within_range_helper(self, current_data, false);
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
        return String::from("NumberRange");
    }

    fn check_count(
        &self,
        _count: usize,
    ) -> std::result::Result<bool, String> {
        return Ok(true);
    }
}

impl ExcludeRuleTrait for NumberRange {
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
        return is_within_range_helper(self, current_data, true);
    }

    fn exclude_name(&self) -> String {
        return self.name();
    }
}
