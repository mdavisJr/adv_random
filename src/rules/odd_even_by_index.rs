use crate::rules::{IsWithinErrorType, MapAnyValue, RuleTrait, OddEvenKey, OddEven};
use crate::settings::Settings;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Clone)]
pub struct OddEvenByIndex {
    odd_even: HashMap<usize, OddEvenKey>,
}

impl OddEvenByIndex {
    pub fn new(odd_indexes: &[usize], even_indexes: &[usize]) -> OddEvenByIndex {
        let mut odd_even: HashMap<usize, OddEvenKey> = HashMap::new();
        for odd_index in odd_indexes {
            odd_even.insert(*odd_index, OddEvenKey::Odd);
        }
        for even_index in even_indexes {
            odd_even.insert(*even_index, OddEvenKey::Even);
        }
        return OddEvenByIndex {
            odd_even
        };
    }

    pub fn from_numbers(numbers: &[usize]) -> OddEvenByIndex {
        let mut odd_even: HashMap<usize, OddEvenKey> = HashMap::new();
        for (idx, number) in numbers.iter().enumerate() {
            if OddEven::is_even(*number) {
                odd_even.insert(idx, OddEvenKey::Even);
            } else {
                odd_even.insert(idx, OddEvenKey::Odd);
            }
        }
        return OddEvenByIndex {
            odd_even
        };
    }
}

impl Display for OddEvenByIndex {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{:?}",
            self.odd_even
        )
    }
}

impl Debug for OddEvenByIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl RuleTrait for OddEvenByIndex {
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
        shared_data: &HashMap<String, HashMap<String, MapAnyValue>>,
    ) -> std::result::Result<Vec<usize>, String> {
        if self.odd_even.contains_key(&selected_numbers.len()) {
            let (min, max) = Settings::get_min_max("NumberRange", shared_data);
            
            if self.odd_even[&selected_numbers.len()] == OddEvenKey::Odd {
                return Ok(vec![OddEven::odd_number(min, max)]);
            } else if self.odd_even[&selected_numbers.len()] == OddEvenKey::Even {
                return Ok(vec![OddEven::even_number(min, max)]);
            } else {
                return Err(String::from("Skip"));
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
            if self.odd_even.contains_key(&idx) {
                let number_type: OddEvenKey = self.odd_even[&idx];
                if (number_type == OddEvenKey::Odd && !OddEven::is_odd(selected_number))
                    || (number_type == OddEvenKey::Even
                        && !OddEven::is_even(selected_number))
                {
                    return Err((
                        IsWithinErrorType::Regular,
                        format!(
                            "Selected number {} at index {} is not: {:?}. Numbers:{:?}",
                            selected_number, idx, number_type, selected_numbers
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
            Err(e) => Err(e.1)
        }
    }

    fn name(&self) -> String {
        return String::from("OddEvenByIndex");
    }

    fn check_count(
        &self,
        _count: usize,
    ) -> std::result::Result<bool, String> {
        return Ok(true);
    }
}
