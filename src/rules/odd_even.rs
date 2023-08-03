use crate::random::CurrentData;
use crate::random_trait::{get_random_trait, get_random_vec_item};
use crate::rules::{MapAnyValue, RuleTrait, IsWithinErrorType};
use crate::settings::Settings;
use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Result};
use std::hash::Hash;

#[derive(Clone, Copy)]
pub struct OddEven {
    odd: usize,
    even: usize,
}

impl OddEven {
    pub fn new(odd: usize, even: usize) -> OddEven {
        return OddEven { odd, even };
    }

    pub fn from_numbers(numbers: &[usize]) -> OddEven {
        let mut odd: usize = 0_usize;
        let mut even: usize = 0_usize;
        for number in numbers {
            if OddEven::is_even(*number) {
                even += 1;
            } else {
                odd += 1;
            }
        }
        return OddEven { odd, even };
    }

    pub fn odd(&self) -> usize {
        return self.odd;
    }

    pub fn even(&self) -> usize {
        return self.even;
    }

    pub fn is_odd(number: usize) -> bool {
        return !OddEven::is_even(number);
    }
    pub fn is_even(number: usize) -> bool {
        if number % 2 == 0 {
            return true;
        }
        return false;
    }

    pub fn even_number(min: usize, max: usize) -> usize {
        for _ in 0..20 {
            let number = get_random_trait().get_number(min, max);
            if OddEven::is_even(number) {
                return number;
            }
        }
        panic!("Could not find an even number in range: {}-{}", min, max);
    }
    pub fn odd_number(min: usize, max: usize) -> usize {
        for _ in 0..20 {
            let number = get_random_trait().get_number(min, max);
            if OddEven::is_odd(number) {
                return number;
            }
        }
        panic!("Could not find an odd number in range: {}-{}", min, max);
    }

    pub fn needs_even(&self, other: &Self) -> bool {
        return self.even < other.even;
    }

    pub fn needs_odd(&self, other: &Self) -> bool {
        return self.odd < other.odd;
    }

    pub fn needs_only_even(&self, other: &Self) -> bool {
        return self.odd == other.odd && self.even < other.even;
    }

    pub fn needs_only_odd(&self, other: &Self) -> bool {
        return self.even == other.even && self.odd < other.odd;
    }
}

impl Display for OddEven {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            "ODD:".to_owned() + &self.odd.to_string() + ",EVEN:" + &self.even.to_string()
        )
    }
}

impl Debug for OddEven {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum OddEvenKey {
    Odd,
    Even,
}

impl RuleTrait for OddEven {
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
        let act_odd_even = OddEven::from_numbers(current_data.selected_numbers());
        let mut pool_keys: Vec<OddEvenKey> = Vec::with_capacity(2);
        if act_odd_even.needs_even(self) {
            pool_keys.push(OddEvenKey::Even);
        }
        if act_odd_even.needs_odd(self) {
            pool_keys.push(OddEvenKey::Odd);
        }
        if !pool_keys.is_empty() {
            let (min, max) = Settings::get_min_max("NumberRange", current_data.shared_data());
            let selected_pool_key = get_random_vec_item(&pool_keys);
            let mut number = 0_usize;
            if *selected_pool_key == OddEvenKey::Odd {
                number = OddEven::odd_number(min, max);
            } else if *selected_pool_key == OddEvenKey::Even {
                number = OddEven::even_number(min, max);
            }
            return Ok(vec![number]);
        }
        return Err(String::from("Skip"));
    }

    fn is_within_range(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<(), (IsWithinErrorType, String)> {
        let other = OddEven::from_numbers(current_data.selected_numbers());
        if other.odd > self.odd {
            return Err((IsWithinErrorType::Regular, "Too Many Odds".to_owned()));
        }
        if other.even > self.even {
            return Err((IsWithinErrorType::Regular, "Too Many Evens".to_owned()));
        }
        return Ok(());
    }

    fn is_match(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<(), String> {
        let other = OddEven::from_numbers(current_data.selected_numbers());
        if self.odd == other.odd && self.even == other.even {
            return Ok(());
        }
        return Err(format!(
            "Expected Odd: {} and Even: {}.  Actual Odd: {} and Even: {}.",
            self.odd, self.even, other.odd, other.even
        ));
    }

    fn name(&self) -> String {
        return String::from("OddEven");
    }

    fn check_count(
        &self,
        count: usize,
    ) -> std::result::Result<bool, String> {
        if (self.even + self.odd) <= count {
            return Ok(true);
        }
        return Err(format!("Odd count: {} and Even count: {} is greater than count: {} ", self.odd, self.even, count));
    }
}
