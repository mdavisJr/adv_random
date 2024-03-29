use crate::random::CurrentData;
use crate::random_trait::{get_random_trait, get_random_vec_item};
use crate::rules::{IsWithinErrorType, MapAnyValue, RuleTrait};
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Mutex;
use std::fmt::{Debug, Display, Formatter, Result};
use once_cell::sync::Lazy;

use super::ExcludeRuleTrait;
use super::exclude_rule_trait::is_excluded_helper;


pub static NP_ALPHABET_SET: Lazy<Mutex<HashSet<char>>> = Lazy::new(|| {
    Mutex::new(HashSet::from_iter(['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z']))
});
pub static NP_NUMERIC_SET: Lazy<Mutex<HashSet<char>>> = Lazy::new(|| {
    Mutex::new(HashSet::from_iter(['0','1','2','3','4','5','6','7','8','9']))
});
pub static NP_SPECIAL_CHAR_SET: Lazy<Mutex<HashSet<char>>> = Lazy::new(|| {
    Mutex::new(HashSet::from_iter(['!','@','#','$','%','^','&','*','(',')','-','+','=']))
});


#[derive(Clone, Debug)]
pub enum PoolType {
    Set(HashSet<usize>),
    MinMax(usize, usize),
}

impl PoolType {
    pub fn new(char_set: &HashSet<char>) -> PoolType {
        return PoolType::Set(char_set.iter().map(|x| *x as usize).collect());
    }

    pub fn random_number(&self) -> usize {
        match self {
            PoolType::Set(set) => {
                let rand_pool: Vec<usize> = set.iter().copied().collect();
                return *get_random_vec_item(&rand_pool);
            }
            PoolType::MinMax(min, max) => {
                return get_random_trait().get_number(*min, *max);
            }
        }
    }

    pub fn has(&self, numbers_set: &HashSet<usize>) -> usize {
        match self {
            PoolType::Set(set) => {
                let intersection = set.intersection(numbers_set);
                return intersection.count();
            }
            PoolType::MinMax(min, max) => {
                let mut has = 0;
                for number in numbers_set {
                    if number >= min && number <= max {
                        has += 1;
                    }
                }
                return has;
            }
        }
    }

    pub fn difference(&self, numbers_set: &HashSet<usize>) -> Vec<usize> {
        match self {
            PoolType::Set(set) => {
                let difference = set.difference(numbers_set);
                return difference.copied().collect();
            }
            PoolType::MinMax(min, max) => {
                let mut differences: Vec<usize> = Vec::new();
                for number in *min..=*max {
                    if !numbers_set.contains(&number) {
                        differences.push(number);
                    }
                }
                return differences;
            }
        }
    }

    pub fn contains(&self, number: usize) -> bool {
        match self {
            PoolType::Set(set) => {
                return set.contains(&number);
            }
            PoolType::MinMax(min, max) => {
                return *min <= number && *max >= number;
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.len() == 0;
    }

    pub fn len(&self) -> usize {
        match self {
            PoolType::Set(set) => set.len(),
            PoolType::MinMax(min, max) => max - min + 1,
        }
    }
}

#[derive(Clone, Debug)]
pub struct NumberPoolItem {
    pool: PoolType,
    needs: usize,
    key: String,
}

impl NumberPoolItem {
    pub fn new(key: &str, pool: &PoolType, needs: usize) -> NumberPoolItem {
        return NumberPoolItem {
            pool: pool.clone(),
            needs,
            key: key.to_owned(),
        };
    }
}

#[derive(Clone, Debug)]
pub struct NumberPoolItemFull {
    pool: PoolType,
    needs: usize,
    has: usize,
    missing: usize,
}

impl NumberPoolItemFull {
    pub fn new(pool: &PoolType, needs: usize, has: usize, missing: usize) -> NumberPoolItemFull {
        return NumberPoolItemFull {
            pool: pool.clone(),
            needs,
            has,
            missing,
        };
    }
    pub fn pool(&self) -> &PoolType {
        return &self.pool;
    }

    pub fn needs(&self) -> usize {
        return self.needs;
    }

    pub fn has(&self) -> usize {
        return self.has;
    }
}

#[derive(Clone)]
pub struct NumberPool {
    number_pool_items: HashMap<String, NumberPoolItemFull>,
}

impl NumberPool {
    pub fn alphanumeric(count: usize, include_special_char: bool) -> NumberPool {
        if (!include_special_char && count < 2) || (include_special_char && count < 3) {
            panic!("Count Must be {} or More", if include_special_char { 3 } else { 2 });
        }
        let mut numeric_count:usize = 1;
        let mut special_char_count: usize = if include_special_char { 1 } else { 0 };

        if count >= 10 {
            numeric_count = get_random_trait().get_number(1, 3);
        }

        if include_special_char && count >= 10 {
            special_char_count = get_random_trait().get_number(1, 2);
        }

        let alpha_count = count - numeric_count - special_char_count;
        if (alpha_count + special_char_count + numeric_count) != count {
            panic!("Counts do not match up. numeric_count:{}, special_char_count:{}, alpha_count:{}", numeric_count, special_char_count, alpha_count);
        }

        return NumberPool::alphanumeric_specs(alpha_count, numeric_count, special_char_count);
    }
    pub fn alphanumeric_specs(alpha_count: usize, numeric_count: usize, special_char_count: usize) -> NumberPool {
        
        return NumberPool::new(&[
            NumberPoolItem::new("alpha_set", &PoolType::new(&NP_ALPHABET_SET.lock().unwrap()), alpha_count),
            NumberPoolItem::new("numeric_set", &PoolType::new(&NP_NUMERIC_SET.lock().unwrap()), numeric_count),
            NumberPoolItem::new("special_char_set", &PoolType::new(&NP_SPECIAL_CHAR_SET.lock().unwrap()), special_char_count),         
        ]);
    }

    pub fn new(number_pool_items: &[NumberPoolItem]) -> NumberPool {
        return NumberPool {
            number_pool_items: number_pool_items
                .iter()
                .map(|x| {
                    (
                        x.key.to_owned(),
                        NumberPoolItemFull {
                            pool: x.pool.clone(),
                            needs: x.needs,
                            has: 0,
                            missing: x.needs,
                        },
                    )
                })
                .collect(),
        };
    }

    pub fn from_number_pool_item_full(
        number_pool_items: &[(&str, NumberPoolItemFull)],
    ) -> NumberPool {
        return NumberPool {
            number_pool_items: number_pool_items
                .iter()
                .map(|x| (x.0.to_owned(), x.1.to_owned()))
                .collect(),
        };
    }

    fn from_numbers_2(
        number_pool_items: &HashMap<String, NumberPoolItemFull>,
        numbers_set: &HashSet<usize>,
    ) -> NumberPool {
        return NumberPool::from_numbers(
            &number_pool_items
                .iter()
                .map(|(k, v)| NumberPoolItem::new(k, &v.pool, v.needs))
                .collect::<Vec<NumberPoolItem>>(),
            numbers_set,
            false,
        );
    }

    pub fn from_numbers(
        number_pool_items: &[NumberPoolItem],
        numbers_set: &HashSet<usize>,
        set_needs_eq_match: bool,
    ) -> NumberPool {
        let mut number_pool_items_full: HashMap<String, NumberPoolItemFull> = HashMap::new();
        for number_pool_item in number_pool_items {
            let match_count = number_pool_item.pool.has(numbers_set);
            let missing = if match_count > number_pool_item.needs {
                0
            } else {
                number_pool_item.needs - match_count
            };
            number_pool_items_full.insert(
                number_pool_item.key.to_owned(),
                NumberPoolItemFull {
                    pool: number_pool_item.pool.clone(),
                    needs: if set_needs_eq_match {
                        match_count
                    } else {
                        number_pool_item.needs
                    },
                    has: match_count,
                    missing,
                },
            );
        }
        return NumberPool {
            number_pool_items: number_pool_items_full,
        };
    }

    pub fn number_pool_items(&self) -> &HashMap<String, NumberPoolItemFull> {
        return &self.number_pool_items;
    }

    pub fn number_pool_item(&self, key: &str) -> Option<&NumberPoolItemFull> {
        return self.number_pool_items.get(key);
    }

    pub fn pool(&self, key: &str) -> Option<&PoolType> {
        return self.number_pool_items.get(key).map(|v| &v.pool);
    }

    pub fn has(&self, key: &str) -> Option<usize> {
        return self.number_pool_items.get(key).map(|v| v.has);
    }

    pub fn needs(&self, key: &str) -> Option<usize> {
        return self.number_pool_items.get(key).map(|v| v.needs);
    }

    pub fn missing(&self, key: &str) -> Option<usize> {
        return self.number_pool_items.get(key).map(|v| v.missing);
    }

    pub fn find_first_has_value(&self) -> Option<&NumberPoolItemFull> {
        for number_pool_item in self.number_pool_items().values() {
            if number_pool_item.has() > 0 {
                return Some(number_pool_item);
            }
        }
        return None;
    }
}

impl Display for NumberPool {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.number_pool_items)
    }
}

impl Debug for NumberPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl RuleTrait for NumberPool {
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
        let other_number_pool =
            NumberPool::from_numbers_2(&self.number_pool_items, current_data.selected_numbers_set());
        let mut numbers: Vec<usize> = Vec::new();
        for (_key, number_pool_item) in &other_number_pool.number_pool_items {
            if number_pool_item.missing > 0 {
                if number_pool_item.pool.len() == number_pool_item.needs {
                    numbers.extend(number_pool_item.pool.difference(current_data.selected_numbers_set()));
                } else {
                    numbers.push(number_pool_item
                        .pool
                        .random_number()
                    );
                }
            }
        }
        if !numbers.is_empty() {
            return Ok(numbers);
        }

        return Err(String::from("Skip"));
    }

    fn is_within_range(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<(), (IsWithinErrorType, String)> {
        let other_number_pool =
            NumberPool::from_numbers_2(&self.number_pool_items, current_data.selected_numbers_set());

        let mut total_missing: usize = 0;
        for (key, number_pool_item) in &other_number_pool.number_pool_items {
            if number_pool_item.has > number_pool_item.needs {
                return Err((IsWithinErrorType::Regular, format!(
                    "Too many from pool {:?}, \"needs\" is {} and \"has\" {} from this pool",
                    key, number_pool_item.needs, number_pool_item.has
                )));
            }
            if number_pool_item.needs > 0 {
                total_missing += number_pool_item.missing;
            }
        }
        let len_remaining = if current_data.selected_numbers().len() > current_data.settings().count() {
            0
        } else {
            current_data.settings().count() - current_data.selected_numbers().len()
        };
        if total_missing > 0 && total_missing > len_remaining {
            return Err((IsWithinErrorType::MakePriority, format!(
                "Need to pull from number pool, \"missing\" {} and there are {} numbers left to pick",
                total_missing, len_remaining
            )));
        }
        return Ok(());
    }

    fn is_match(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<(), String> {
        let other_number_pool =
            NumberPool::from_numbers_2(&self.number_pool_items, current_data.selected_numbers_set());
        for (key, number_pool_item) in other_number_pool.number_pool_items {
            if number_pool_item.has != number_pool_item.needs {
                return Err(format!(
                    "Expected--Pool:{:?}--{:?}--Needs:{}. Actual Count:{}",
                    key, number_pool_item.pool, number_pool_item.needs, number_pool_item.has
                ));
            }
        }
        return Ok(());
    }

    fn name(&self) -> String {
        return String::from("NumberPool");
    }

    fn check_count(
        &self,
        count: usize,
    ) -> std::result::Result<bool, String> {
        let needs_count:usize = self.number_pool_items.values().into_iter().map(|x| x.needs).sum::<usize>();
        if needs_count <= count {
            return Ok(true);
        }
        return Err(format!("{} count: {} is greater than count: {} ", self.name(), needs_count, count));
    }
}

impl ExcludeRuleTrait for NumberPool {
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
