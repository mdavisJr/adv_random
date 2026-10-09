use crate::random::{CurrentData, NumberType};
#[cfg(feature="rand")]
use crate::default_random::DefaultRandom;
use crate::random_trait::RandomTrait;
use crate::rules::{IsWithinErrorType, SharedData, OperatorType, RuleTrait};
use std::any::Any;
use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Result};

use super::ExcludeRuleTrait;
use super::exclude_rule_trait::is_excluded_helper;


pub const NP_ALPHABET_SET: [char; 52] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
pub const NP_NUMERIC_SET: [char; 10] = ['0','1','2','3','4','5','6','7','8','9'];
pub const NP_SPECIAL_CHAR_SET: [char; 13] = ['!','@','#','$','%','^','&','*','(',')','-','+','='];


//The numbers of a PoolType::Set.  They are kept sorted with no duplicates, so use PoolType::set to create one.
#[derive(Clone, PartialEq, Eq)]
pub struct PoolSet {
    numbers: Vec<usize>,
}

impl PoolSet {
    pub fn numbers(&self) -> &[usize] {
        return &self.numbers;
    }

    pub fn contains(&self, number: usize) -> bool {
        return self.numbers.binary_search(&number).is_ok();
    }
}

impl Debug for PoolSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.numbers)
    }
}

#[derive(Clone, Debug)]
pub enum PoolType {
    Set(PoolSet),
    MinMax(usize, usize),
}

impl PoolType {
    pub fn set(numbers: impl IntoIterator<Item = usize>) -> PoolType {
        let mut numbers: Vec<usize> = numbers.into_iter().collect();
        numbers.sort_unstable();
        numbers.dedup();
        return PoolType::Set(PoolSet { numbers });
    }

    pub fn new(char_set: &HashSet<char>) -> PoolType {
        return PoolType::set(char_set.iter().map(|x| *x as usize));
    }

    pub fn from_chars(chars: &[char]) -> PoolType {
        return PoolType::set(chars.iter().map(|x| *x as usize));
    }

    pub fn random_number(&self, current_data: &CurrentData) -> Option<usize> {
        return current_data.get_rand(NumberType::Any, Some(self), None);
    }

    //counts every number that is in this pool, so a number selected twice counts twice
    pub fn has(&self, numbers: &[usize]) -> usize {
        return numbers.iter().filter(|x| self.contains(**x)).count();
    }

    pub fn difference(&self, numbers_set: &HashSet<usize>) -> Vec<usize> {
        match self {
            PoolType::Set(set) => {
                return set.numbers().iter().copied().filter(|x| !numbers_set.contains(x)).collect();
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
                return set.contains(number);
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
            PoolType::Set(set) => set.numbers().len(),
            PoolType::MinMax(min, max) => if min > max { 0 } else { (max - min).saturating_add(1) },
        }
    }
}

#[derive(Clone, Debug)]
pub struct NumberPoolItem {
    pool: PoolType,
    needs: OperatorType,
    key: String,
}

impl NumberPoolItem {
    pub fn new(key: &str, pool: &PoolType, needs: OperatorType) -> NumberPoolItem {
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
    needs: OperatorType,
    has: usize,
    missing: usize,
}

impl NumberPoolItemFull {
    pub fn new(pool: &PoolType, needs: OperatorType, has: usize, missing: usize) -> NumberPoolItemFull {
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

    pub fn needs(&self) -> OperatorType {
        return self.needs;
    }

    pub fn has(&self) -> usize {
        return self.has;
    }
}

#[derive(Clone)]
pub struct NumberPool {
    //sorted by key, so a seeded random generator gives the same numbers every run
    number_pool_items: BTreeMap<String, NumberPoolItemFull>,
    //set when the number pool could not be created.  check_count returns it.
    error: Option<String>,
}

impl NumberPool {
    #[cfg(feature="rand")]
    pub fn alphanumeric(count: usize, include_special_char: bool) -> NumberPool {
        return NumberPool::alphanumeric_with_random(&DefaultRandom{}, count, include_special_char);
    }

    pub fn alphanumeric_with_random(random: &dyn RandomTrait, count: usize, include_special_char: bool) -> NumberPool {
        let min_count = if include_special_char { 3 } else { 2 };
        if count < min_count {
            return NumberPool {
                number_pool_items: BTreeMap::new(),
                error: Some(format!("NumberPool alphanumeric count: {} must be {} or more", count, min_count)),
            };
        }
        let mut numeric_count:usize = 1;
        let mut special_char_count: usize = if include_special_char { 1 } else { 0 };

        if count >= 10 {
            numeric_count = random.get_number(1, 3);
        }

        if include_special_char && count >= 10 {
            special_char_count = random.get_number(1, 2);
        }

        let alpha_count = count - numeric_count - special_char_count;

        return NumberPool::alphanumeric_specs(alpha_count, numeric_count, special_char_count);
    }
    pub fn alphanumeric_specs(alpha_count: usize, numeric_count: usize, special_char_count: usize) -> NumberPool {
        
        return NumberPool::new(&[
            NumberPoolItem::new("alpha_set", &PoolType::from_chars(&NP_ALPHABET_SET), OperatorType::Eq(alpha_count)),
            NumberPoolItem::new("numeric_set", &PoolType::from_chars(&NP_NUMERIC_SET), OperatorType::Eq(numeric_count)),
            NumberPoolItem::new("special_char_set", &PoolType::from_chars(&NP_SPECIAL_CHAR_SET), OperatorType::Eq(special_char_count)),         
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
                            missing: x.needs.get_missing_without_has(),
                        },
                    )
                })
                .collect(),
            error: None,
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
            error: None,
        };
    }

    //Returns the key, item, has and missing of every pool for these numbers
    fn counts<'a>(&'a self, numbers: &'a [usize]) -> impl Iterator<Item = (&'a String, &'a NumberPoolItemFull, usize, usize)> + 'a {
        return self.number_pool_items.iter().map(move |(key, number_pool_item)| {
            let has = number_pool_item.pool.has(numbers);
            (key, number_pool_item, has, number_pool_item.needs.get_missing(has))
        });
    }

    pub fn from_numbers(
        number_pool_items: &[NumberPoolItem],
        numbers: &[usize],
        set_needs_eq_match: bool,
    ) -> NumberPool {
        let mut number_pool_items_full: BTreeMap<String, NumberPoolItemFull> = BTreeMap::new();
        for number_pool_item in number_pool_items {
            let match_count = number_pool_item.pool.has(numbers);
            let missing = number_pool_item.needs.get_missing(match_count);
            number_pool_items_full.insert(
                number_pool_item.key.to_owned(),
                NumberPoolItemFull {
                    pool: number_pool_item.pool.clone(),
                    needs: if set_needs_eq_match {
                        OperatorType::Eq(match_count)
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
            error: None,
        };
    }

    pub fn number_pool_items(&self) -> &BTreeMap<String, NumberPoolItemFull> {
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

    pub fn needs(&self, key: &str) -> Option<OperatorType> {
        return self.number_pool_items.get(key).map(|v| v.needs);
    }

    pub fn missing(&self, key: &str) -> Option<usize> {
        return self.number_pool_items.get(key).map(|v| v.missing);
    }

    pub fn find_first_has_value(&self) -> Option<&NumberPoolItemFull> {
        return self.number_pool_items().values().find(|&number_pool_item| number_pool_item.has() > 0);        
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
    ) -> Option<SharedData> {
        let pool_types: Vec<PoolType> = self.number_pool_items().values().filter(|x| x.needs.is_absolute_zero()).map(|x|x.pool().clone()).collect();
        
        if pool_types.is_empty() {
            None
        } else {
            Some(SharedData::new().with_exclusion_pools(pool_types))
        }
    }

    fn get_numbers(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<Option<Vec<usize>>, String> {
        let mut numbers: Vec<usize> = Vec::new();
        let mut number_not_found = false;
        for (_, number_pool_item, _, missing) in self.counts(current_data.selected_numbers()) {
            if missing > 0 {
                if let OperatorType::Eq(v) = number_pool_item.needs {
                    if number_pool_item.pool.len() == v {
                        let difference = number_pool_item.pool.difference(current_data.selected_numbers_set());
                        if difference.len() == missing {
                            numbers.extend(difference);
                            continue;
                        }
                    }
                }

                match number_pool_item.pool.random_number(current_data) {
                    Some(number) => numbers.push(number),
                    None => number_not_found = true
                }
            }
        }
        if !numbers.is_empty() {
            return Ok(Some(numbers));
        }
        if number_not_found {
            return Err(current_data.no_number_err());
        }

        return Ok(None);
    }

    fn is_within_range(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<(), (IsWithinErrorType, String)> {
        let mut total_missing: usize = 0;
        for (key, number_pool_item, has, missing) in self.counts(current_data.selected_numbers()) {
            if !number_pool_item.needs.is_within_range(has) {
                return Err((IsWithinErrorType::Regular, format!(
                    "Too many from pool {:?}, \"needs\" is {} and \"has\" {} from this pool",
                    key, number_pool_item.needs, has
                )));
            }
            if !number_pool_item.needs.is_absolute_zero() {
                total_missing += missing;
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
        for (key, number_pool_item, has, _) in self.counts(current_data.selected_numbers()) {
            if !number_pool_item.needs.is_match(has) {
                return Err(format!(
                    "Expected--Pool:{:?}--{:?}--Needs:{}. Actual Count:{}",
                    key, number_pool_item.pool, number_pool_item.needs, has
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
        if let Some(e) = &self.error {
            return Err(e.to_owned());
        }
        let needs_count:usize = self.number_pool_items.values().map(|x| x.needs.get_missing_without_has()).sum::<usize>();
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
