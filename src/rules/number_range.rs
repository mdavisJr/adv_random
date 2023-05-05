use crate::rules::{MapAnyValue, RuleTrait, IsWithinErrorType};
use crate::settings::Settings;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Result};

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
        _selected_numbers_set: &HashSet<usize>,
        selected_numbers: &[usize],
        _settings: &Settings,
    ) -> Option<HashMap<String, MapAnyValue>> {        
        let key = if self.use_0_idx_for_all {0} else {selected_numbers.len()};
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
        _selected_numbers_set: &HashSet<usize>,
        _selected_numbers: &[usize],
        _settings: &Settings,
        _shared_data: &HashMap<String, HashMap<String, MapAnyValue>>,
    ) -> std::result::Result<Vec<usize>, String> {
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
            let key = if self.use_0_idx_for_all {0} else {idx};
            if self.ranges.contains_key(&key) {
                let (min, max) = self.ranges[&key];
                if selected_number < min || selected_number > max {
                    return Err((IsWithinErrorType::Regular, format!(
                        "Selected number {} at index {} is not within range of min: {} and max: {}. Numbers:{:?}.  Map Index:{}",
                        selected_number, idx, min, max, selected_numbers, key
                    )));
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
