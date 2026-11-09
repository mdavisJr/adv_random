use crate::random::CurrentData;
use crate::rules::{SharedData, RuleTrait, IsWithinErrorType};
use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Result};

use super::ExcludeRuleTrait;

fn is_within_range_helper(
    number_range: &NumberRange,
    current_data: &CurrentData,
    exclude: bool
) -> std::result::Result<(), (IsWithinErrorType, String)> {
    for (idx, selected_number) in current_data.selected_numbers().iter().copied().enumerate() {
        let key = if number_range.all.is_some() {0} else {idx};
        if let Some((min, max)) = number_range.range_at(idx) {
            if (!exclude && (selected_number < min || selected_number > max)) || (exclude && (selected_number >= min && selected_number <= max)) {
                return Err((IsWithinErrorType::Regular, format!(
                    "Exclude: {} - Selected number {} at index {} is not within range of min: {} and max: {}. Numbers:{:?}.  Map Index:{}",
                    exclude, selected_number, idx, min, max, current_data.selected_numbers(), key
                )));
            }
        }
    }
    return Ok(());
}

#[derive(Clone, Default)]
pub struct NumberRange {
    ranges: HashMap<usize, (usize, usize)>,
    //The range of every number, when all made it.  It is in ranges too, at index 0.
    all: Option<(usize, usize)>,
    errors: Vec<String>,
}

impl NumberRange {
    //Every number is from min to max
    pub fn all(min: usize, max: usize) -> NumberRange {
        let mut ranges: HashMap<usize, (usize, usize)> = HashMap::new();
        ranges.insert(0, (min, max));
        return NumberRange { ranges, all: Some((min, max)), errors: Vec::new() };
    }

    //No number has a range yet.  Give the numbers at some indexes a range with at.
    pub fn new() -> NumberRange {
        return NumberRange::default();
    }

    //The numbers at these indexes are from min to max.  The first number is index 0.
    pub fn at(mut self, indexes: impl IntoIterator<Item = usize>, min: usize, max: usize) -> NumberRange {
        if self.all.is_some() {
            self.errors.push(format!("{} at can not be used with all", self.name()));
            return self;
        }
        for index in indexes {
            if self.ranges.insert(index, (min, max)).is_some() {
                self.errors.push(format!("{} index: {} has more than one range", self.name(), index));
            }
        }
        return self;
    }

    //The problems with how this was created.  Nothing added to it is one of them, because then it does nothing.
    fn chain_errors(&self) -> Vec<String> {
        let mut errors = self.errors.clone();
        if errors.is_empty() && self.ranges.is_empty() {
            errors.push(format!("{} has no ranges", self.name()));
        }
        return errors;
    }

    #[deprecated(since = "3.0.0", note = "use NumberRange::new().at(indexes, min, max)")]
    pub fn from_map(ranges: &[(&[usize], usize, usize)]) -> NumberRange {
        let mut number_range = NumberRange::new();
        for range in ranges {
            for idx in range.0 {
                number_range.ranges.insert(*idx, (range.1, range.2));
            }
        }
        return number_range;
    }

    //The range of the number at index idx, if it has one
    fn range_at(&self, idx: usize) -> Option<(usize, usize)> {
        return match self.all {
            Some(range) => Some(range),
            None => self.ranges.get(&idx).copied(),
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
        write!(f, "{}", self)
    }
}

impl RuleTrait for NumberRange {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn share_data(
        &self,
        current_data: &CurrentData,
    ) -> Option<SharedData> {
        return self.range_at(current_data.selected_numbers().len()).map(|(min, max)| SharedData::new().with_min_max(min, max));
    }

    fn get_numbers(
        &self,
        _current_data: &CurrentData
    ) -> std::result::Result<Option<Vec<usize>>, String> {
        return Ok(None);
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
        count: usize,
    ) -> std::result::Result<bool, String> {
        let mut keys = self.ranges.keys().copied().collect::<Vec<usize>>();
        keys.sort();
        for key in keys {
            let (min, max) = self.ranges[&key];
            if min > max {
                return Err(format!("{} min: {} is greater than max: {}", self.name(), min, max));
            }
        }
        if self.all.is_none() {
            if let Some(index) = self.ranges.keys().copied().filter(|x| *x >= count).min() {
                return Err(format!("{} index: {} is past the last number for count: {}", self.name(), index, count));
            }
        }
        return Ok(true);
    }

    fn validate(&self, count: usize) -> Vec<String> {
        let mut errors = self.chain_errors();
        if let Err(e) = self.check_count(count) {
            errors.push(e);
        }
        return errors;
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

    fn validate_exclude(&self) -> Vec<String> {
        return self.chain_errors();
    }
}
