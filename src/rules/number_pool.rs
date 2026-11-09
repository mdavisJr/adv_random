use crate::random::{CurrentData, NumberType};
use crate::random_trait::{get_random_vec_item, RandomTrait};
use crate::rules::{IsWithinErrorType, SharedData, OperatorType, RuleTrait};
use std::any::Any;
use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::RangeInclusive;

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

//A list or a set of numbers is a pool of those numbers
impl<const N: usize> From<[usize; N]> for PoolType {
    fn from(numbers: [usize; N]) -> PoolType {
        return PoolType::set(numbers);
    }
}

impl From<&[usize]> for PoolType {
    fn from(numbers: &[usize]) -> PoolType {
        return PoolType::set(numbers.iter().copied());
    }
}

impl From<Vec<usize>> for PoolType {
    fn from(numbers: Vec<usize>) -> PoolType {
        return PoolType::set(numbers);
    }
}

impl From<HashSet<usize>> for PoolType {
    fn from(numbers: HashSet<usize>) -> PoolType {
        return PoolType::set(numbers);
    }
}

//1..=10 is a pool of every number from 1 to 10
impl From<RangeInclusive<usize>> for PoolType {
    fn from(range: RangeInclusive<usize>) -> PoolType {
        return PoolType::MinMax(*range.start(), *range.end());
    }
}

//"ABC" is a pool of the characters A, B and C
impl From<&str> for PoolType {
    fn from(characters: &str) -> PoolType {
        return PoolType::set(characters.chars().map(|x| x as usize));
    }
}

impl From<&PoolType> for PoolType {
    fn from(pool: &PoolType) -> PoolType {
        return pool.clone();
    }
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

    //Returns a number that is in both pools.  None when they have no number in common.
    pub fn first_shared_number(&self, other: &PoolType) -> Option<usize> {
        return match (self, other) {
            (PoolType::MinMax(min_1, max_1), PoolType::MinMax(min_2, max_2)) => {
                let lowest = *min_1.max(min_2);
                let highest = *max_1.min(max_2);
                if min_1 <= max_1 && min_2 <= max_2 && lowest <= highest { Some(lowest) } else { None }
            },
            (PoolType::Set(set), pool) | (pool, PoolType::Set(set)) => set.numbers().iter().copied().find(|x| pool.contains(*x))
        };
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

//Counts that are spread over pools in any order.  The counts 4, 1, 0 over three pools is 4 numbers from one of the
//pools, 1 number from another one, and none from the pool that is left.  Created with NumberPool::spread.
#[derive(Clone, Debug)]
pub struct NumberPoolSpread {
    //from the largest to the smallest
    counts: Vec<usize>,
    pools: Vec<PoolType>,
}

impl NumberPoolSpread {
    pub fn counts(&self) -> &Vec<usize> {
        return &self.counts;
    }

    pub fn pools(&self) -> &Vec<PoolType> {
        return &self.pools;
    }

    //How many of the numbers are from each pool
    pub fn has(&self, numbers: &[usize]) -> Vec<usize> {
        return self.pools.iter().map(|x| x.has(numbers)).collect();
    }

    //Returns true when the pools can still end up with the counts in some order.  That is when the largest "has" is
    //not more than the largest count, the next largest is not more than the next largest count, and so on.
    fn fits(&self, has: &[usize]) -> bool {
        if has.len() != self.counts.len() {
            return false;
        }
        let mut has = has.to_vec();
        has.sort_unstable_by(|a, b| b.cmp(a));
        return has.iter().zip(self.counts.iter()).all(|(has, count)| has <= count);
    }

    //Returns true when one more number can come from this pool
    fn can_take(&self, has: &[usize], pool_index: usize) -> bool {
        let mut has = has.to_vec();
        has[pool_index] += 1;
        return self.fits(&has);
    }

    fn total(&self) -> usize {
        return self.counts.iter().fold(0, |a, b| a.saturating_add(*b));
    }
}

//How many digits and special characters NumberPool::alphanumeric was asked for
#[derive(Clone, Copy)]
struct Alphanumeric {
    count: usize,
    //None is 1 digit, or 1 to 3 digits when there are 10 or more characters
    digits: Option<usize>,
    special_characters: SpecialCharacters,
}

#[derive(Clone, Copy)]
enum SpecialCharacters {
    None,
    //1 special character, or 1 or 2 when there are 10 or more characters
    Some,
    Exactly(usize),
}

#[derive(Clone, Default)]
pub struct NumberPool {
    //sorted by key, so a seeded random generator gives the same numbers every run
    number_pool_items: BTreeMap<String, NumberPoolItemFull>,
    errors: Vec<String>,
    //Settings turns this into pools of letters, digits and special characters when it is built, because how many
    //digits there are can depend on the random number generator of the settings
    alphanumeric: Option<Alphanumeric>,
    spreads: Vec<NumberPoolSpread>,
}

impl NumberPool {
    //Letters and digits for a string of this many characters.  There is 1 digit, or 1 to 3 digits when there are 10
    //or more characters, and the rest are letters.  Change that with digits, with_special_characters and
    //special_characters.
    pub fn alphanumeric(count: usize) -> NumberPool {
        return NumberPool {
            number_pool_items: BTreeMap::new(),
            errors: Vec::new(),
            alphanumeric: Some(Alphanumeric { count, digits: None, special_characters: SpecialCharacters::None }),
            spreads: Vec::new(),
        };
    }

    //Exactly this many of the characters are digits
    pub fn digits(mut self, count: usize) -> NumberPool {
        match &mut self.alphanumeric {
            Some(alphanumeric) => {
                if alphanumeric.digits.is_some() {
                    self.errors.push(String::from("NumberPool digits was set more than once"));
                }
                alphanumeric.digits = Some(count);
            },
            None => self.errors.push(format!("{} digits can only be used with alphanumeric", self.name()))
        }
        return self;
    }

    //Adds 1 special character, or 1 or 2 when there are 10 or more characters
    pub fn with_special_characters(mut self) -> NumberPool {
        match &mut self.alphanumeric {
            Some(alphanumeric) => {
                if !matches!(alphanumeric.special_characters, SpecialCharacters::None) {
                    self.errors.push(String::from("NumberPool special characters were set more than once"));
                }
                alphanumeric.special_characters = SpecialCharacters::Some;
            },
            None => self.errors.push(format!("{} with_special_characters can only be used with alphanumeric", self.name()))
        }
        return self;
    }

    //Exactly this many of the characters are special characters
    pub fn special_characters(mut self, count: usize) -> NumberPool {
        match &mut self.alphanumeric {
            Some(alphanumeric) => {
                if !matches!(alphanumeric.special_characters, SpecialCharacters::None) {
                    self.errors.push(String::from("NumberPool special characters were set more than once"));
                }
                alphanumeric.special_characters = SpecialCharacters::Exactly(count);
            },
            None => self.errors.push(format!("{} special_characters can only be used with alphanumeric", self.name()))
        }
        return self;
    }

    //Returns this number pool with the pools of letters, digits and special characters that alphanumeric asked for.
    //None when alphanumeric was not used.  Settings calls this when it is built.
    pub fn resolve_alphanumeric(&self, random: Option<&dyn RandomTrait>) -> Option<NumberPool> {
        let alphanumeric = self.alphanumeric?;
        let count = alphanumeric.count;
        let mut number_pool = self.clone();
        number_pool.alphanumeric = None;

        //the digits are picked before the special characters so a seeded random generator always gives the same counts
        let numeric_count = match (alphanumeric.digits, random) {
            (Some(v), _) => v,
            (None, Some(random)) if count >= 10 => random.get_number(1, 3),
            (None, _) => 1
        };
        let special_char_count = match (alphanumeric.special_characters, random) {
            (SpecialCharacters::None, _) => 0,
            (SpecialCharacters::Exactly(v), _) => v,
            (SpecialCharacters::Some, Some(random)) if count >= 10 => random.get_number(1, 2),
            (SpecialCharacters::Some, _) => 1
        };
        let is_exact = alphanumeric.digits.is_some() || matches!(alphanumeric.special_characters, SpecialCharacters::Exactly(_));
        let min_count = if is_exact { numeric_count.saturating_add(special_char_count) } else { numeric_count + special_char_count + 1 };
        if count < min_count {
            number_pool.errors.push(if is_exact {
                format!("{} alphanumeric digits: {} and special characters: {} are more than count: {}", self.name(), numeric_count, special_char_count, count)
            } else {
                format!("{} alphanumeric count: {} must be {} or more", self.name(), count, min_count)
            });
            return Some(number_pool);
        }

        return Some(number_pool
            .item("alpha_set", PoolType::from_chars(&NP_ALPHABET_SET), OperatorType::Eq(count - numeric_count - special_char_count))
            .item("numeric_set", PoolType::from_chars(&NP_NUMERIC_SET), OperatorType::Eq(numeric_count))
            .item("special_char_set", PoolType::from_chars(&NP_SPECIAL_CHAR_SET), OperatorType::Eq(special_char_count)));
    }

    #[deprecated(since = "3.0.0", note = "use NumberPool::alphanumeric(count), and .random(...) on the settings builder")]
    pub fn alphanumeric_with_random(random: &dyn RandomTrait, count: usize, include_special_char: bool) -> NumberPool {
        let number_pool = NumberPool::alphanumeric(count);
        let number_pool = if include_special_char { number_pool.with_special_characters() } else { number_pool };
        return number_pool.resolve_alphanumeric(Some(random)).unwrap_or(number_pool);
    }

    #[deprecated(since = "3.0.0", note = "use NumberPool::alphanumeric(count).digits(...).special_characters(...)")]
    pub fn alphanumeric_specs(alpha_count: usize, numeric_count: usize, special_char_count: usize) -> NumberPool {
        let number_pool = NumberPool::alphanumeric(alpha_count + numeric_count + special_char_count)
            .digits(numeric_count)
            .special_characters(special_char_count);
        return number_pool.resolve_alphanumeric(None).unwrap_or(number_pool);
    }

    //No numbers have to come from a pool yet.  Say what is needed with exactly, at_least, at_most, between and none_of.
    //A pool is a list of numbers like [1, 4], a range like 1..=10, or the characters of a string like "ABC".
    pub fn new() -> NumberPool {
        return NumberPool::default();
    }

    //This many of the numbers come from the pool.  A number that is picked twice counts twice.
    pub fn exactly(self, count: usize, pool: impl Into<PoolType>) -> NumberPool {
        return self.add(pool, OperatorType::Eq(count));
    }

    //This many or more of the numbers come from the pool
    pub fn at_least(self, count: usize, pool: impl Into<PoolType>) -> NumberPool {
        return self.add(pool, OperatorType::Gte(count));
    }

    //This many or fewer of the numbers come from the pool
    pub fn at_most(self, count: usize, pool: impl Into<PoolType>) -> NumberPool {
        return self.add(pool, OperatorType::Lte(count));
    }

    //From min to max of the numbers come from the pool
    pub fn between(self, min: usize, max: usize, pool: impl Into<PoolType>) -> NumberPool {
        return self.add(pool, OperatorType::Between(min, max));
    }

    //None of the numbers come from the pool
    pub fn none_of(self, pool: impl Into<PoolType>) -> NumberPool {
        return self.add(pool, OperatorType::Eq(0));
    }

    //Adds a pool with a key that is made up
    fn add(self, pool: impl Into<PoolType>, needs: OperatorType) -> NumberPool {
        let mut number = self.number_pool_items.len() + 1;
        while self.number_pool_items.contains_key(&format!("pool_{:03}", number)) {
            number += 1;
        }
        return self.item(&format!("pool_{:03}", number), pool, needs);
    }

    //Adds a pool with a key to find it with later, and any type for how many numbers come from it
    pub fn item(mut self, key: &str, pool: impl Into<PoolType>, needs: OperatorType) -> NumberPool {
        let pool: PoolType = pool.into();
        //an empty pool is only a problem when numbers have to come from it
        if pool.is_empty() && needs.get_missing_without_has() > 0 {
            self.errors.push(format!("{} pool {:?} is empty", self.name(), key));
        }
        if let Err(e) = needs.check() {
            self.errors.push(format!("{} pool {:?} {}", self.name(), key, e));
        }
        let number_pool_item = NumberPoolItemFull {
            pool,
            needs,
            has: 0,
            missing: needs.get_missing_without_has(),
        };
        if self.number_pool_items.insert(key.to_owned(), number_pool_item).is_some() {
            self.errors.push(format!("{} has more than one pool with the key {:?}", self.name(), key));
        }
        return self;
    }

    //Starts counts that are spread over pools in any order.  Give the pools with over, one for every count.
    //spread([4, 1, 0]).over(1..=13).over(14..=26).over(27..=39) is 4 numbers from one of the three pools, 1 number
    //from another one, and none from the pool that is left.
    pub fn spread(mut self, counts: impl IntoIterator<Item = usize>) -> NumberPool {
        let mut counts: Vec<usize> = counts.into_iter().collect();
        counts.sort_unstable_by(|a, b| b.cmp(a));
        self.spreads.push(NumberPoolSpread { counts, pools: Vec::new() });
        return self;
    }

    //Adds a pool to the counts that the last call to spread started
    pub fn over(mut self, pool: impl Into<PoolType>) -> NumberPool {
        match self.spreads.last_mut() {
            Some(spread) => spread.pools.push(pool.into()),
            None => self.errors.push(format!("{} over can only be used after spread", self.name()))
        }
        return self;
    }

    pub fn spreads(&self) -> &Vec<NumberPoolSpread> {
        return &self.spreads;
    }

    //The problems with how this was created.  Nothing added to it is one of them, because then it does nothing.
    fn chain_errors(&self) -> Vec<String> {
        let mut errors = self.errors.clone();
        for spread in &self.spreads {
            if spread.counts.is_empty() {
                errors.push(format!("{} spread has no counts", self.name()));
            }
            if spread.counts.len() != spread.pools.len() {
                errors.push(format!(
                    "{} spread {:?} has {} counts and {} pools.  Call over one time for every count",
                    self.name(), spread.counts, spread.counts.len(), spread.pools.len()
                ));
            }
            //a number has to be from one pool only, so that it is counted one time
            for (index_1, pool_1) in spread.pools.iter().enumerate() {
                for (index_2, pool_2) in spread.pools.iter().enumerate().skip(index_1 + 1) {
                    if let Some(number) = pool_1.first_shared_number(pool_2) {
                        errors.push(format!(
                            "{} spread {:?} pools {} and {} both have the number {}",
                            self.name(), spread.counts, index_1 + 1, index_2 + 1, number
                        ));
                    }
                }
            }
        }
        if errors.is_empty() && self.number_pool_items.is_empty() && self.alphanumeric.is_none() && self.spreads.is_empty() {
            errors.push(format!("{} has no pools", self.name()));
        }
        return errors;
    }

    //Checks the pools when no two numbers can be the same.  A pool can not give more numbers than it has.
    pub fn check_no_duplicates(&self) -> std::result::Result<(), String> {
        for spread in &self.spreads {
            //the largest count has to fit in the largest pool, the next largest in the next largest pool, and so on
            let mut pool_sizes: Vec<usize> = spread.pools.iter().map(|x| x.len()).collect();
            pool_sizes.sort_unstable_by(|a, b| b.cmp(a));
            if let Some((count, pool_size)) = spread.counts.iter().zip(pool_sizes.iter()).find(|(count, pool_size)| count > pool_size) {
                return Err(format!(
                    "{} spread {:?} needs {} numbers from a pool that only has {}, which is not possible with NoDuplicate",
                    self.name(), spread.counts, count, pool_size
                ));
            }
        }
        for (key, number_pool_item) in &self.number_pool_items {
            let numbers_needed = number_pool_item.needs.get_missing_without_has();
            if numbers_needed > number_pool_item.pool.len() {
                return Err(format!(
                    "{} pool {:?} needs {} numbers but only has {}, which is not possible with NoDuplicate",
                    self.name(), key, numbers_needed, number_pool_item.pool.len()
                ));
            }
        }
        return Ok(());
    }

    pub fn from_number_pool_item_full(
        number_pool_items: &[(&str, NumberPoolItemFull)],
    ) -> NumberPool {
        return NumberPool {
            number_pool_items: number_pool_items
                .iter()
                .map(|x| (x.0.to_owned(), x.1.to_owned()))
                .collect(),
            errors: Vec::new(),
            alphanumeric: None,
            spreads: Vec::new(),
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
            errors: Vec::new(),
            alphanumeric: None,
            spreads: Vec::new(),
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
        if self.spreads.is_empty() {
            return write!(f, "{:?}", self.number_pool_items);
        }
        write!(f, "{:?} {:?}", self.number_pool_items, self.spreads)
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
        current_data: &CurrentData,
    ) -> Option<SharedData> {
        let mut pool_types: Vec<PoolType> = self.number_pool_items().values().filter(|x| x.needs.is_absolute_zero()).map(|x|x.pool().clone()).collect();
        //the next number can not be from a pool of a spread that can not take another number
        for spread in &self.spreads {
            let has = spread.has(current_data.selected_numbers());
            if spread.fits(&has) {
                for (pool_index, pool) in spread.pools.iter().enumerate() {
                    if !spread.can_take(&has, pool_index) {
                        pool_types.push(pool.clone());
                    }
                }
            }
        }

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
        for spread in &self.spreads {
            let has = spread.has(current_data.selected_numbers());
            if spread.fits(&has) && has.iter().sum::<usize>() < spread.total() {
                //one number from any of the pools that can still take one
                let pools: Vec<&PoolType> = spread.pools.iter().enumerate().filter(|(pool_index, _)| spread.can_take(&has, *pool_index)).map(|(_, pool)| pool).collect();
                let pool = current_data.random().and_then(|random| get_random_vec_item(random, &pools));
                match pool.and_then(|pool| pool.random_number(current_data)) {
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
        for spread in &self.spreads {
            let has = spread.has(current_data.selected_numbers());
            if !spread.fits(&has) {
                return Err((IsWithinErrorType::Regular, format!(
                    "Too many from a pool of the spread {:?}, the pools have {:?}",
                    spread.counts, has
                )));
            }
            total_missing += spread.total() - has.iter().sum::<usize>();
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
        for spread in &self.spreads {
            let has = spread.has(current_data.selected_numbers());
            let mut sorted_has = has.clone();
            sorted_has.sort_unstable_by(|a, b| b.cmp(a));
            if sorted_has != spread.counts {
                return Err(format!(
                    "Expected--Spread:{:?} in any order. Actual Counts:{:?}",
                    spread.counts, has
                ));
            }
        }
        return Ok(());
    }

    //Its is_within_range only says no when nothing added could make the numbers match.
    //rules_only_stop_tickets_that_can_not_match checks it.
    fn can_still_match(&self, current_data: &CurrentData) -> bool {
        return self.is_within_range(current_data).is_ok();
    }

    fn name(&self) -> String {
        return String::from("NumberPool");
    }

    fn check_count(
        &self,
        count: usize,
    ) -> std::result::Result<bool, String> {
        let needs_count:usize = self.number_pool_items.values().map(|x| x.needs.get_missing_without_has()).sum::<usize>()
            + self.spreads.iter().map(|x| x.total()).sum::<usize>();
        if needs_count <= count {
            return Ok(true);
        }
        return Err(format!("{} count: {} is greater than count: {} ", self.name(), needs_count, count));
    }

    fn validate(&self, count: usize) -> Vec<String> {
        let mut errors = self.chain_errors();
        if let Err(e) = self.check_count(count) {
            errors.push(e);
        }
        return errors;
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

    fn validate_exclude(&self) -> Vec<String> {
        return self.chain_errors();
    }
}
