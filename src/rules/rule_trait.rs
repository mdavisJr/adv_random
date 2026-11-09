use crate::random::CurrentData;
use crate::rules::PoolType;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Display;

//A value that custom rules share with each other.  See SharedData::with_custom.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CustomValue {
    Usize(usize),
    String(String),
    Bool(bool),
}

//What the rules tell each other, and CurrentData::get_rand, about the next number
#[derive(Debug, Clone, Default)]
pub struct SharedData {
    min: Option<usize>,
    max: Option<usize>,
    no_duplicates: bool,
    exclusion_pools: Vec<PoolType>,
    custom: HashMap<String, CustomValue>,
}

impl SharedData {
    pub fn new() -> SharedData {
        return SharedData::default();
    }

    pub fn with_min_max(mut self, min: usize, max: usize) -> SharedData {
        self.min = Some(min);
        self.max = Some(max);
        return self;
    }

    pub fn with_no_duplicates(mut self) -> SharedData {
        self.no_duplicates = true;
        return self;
    }

    //The next number can not be from any of these pools
    pub fn with_exclusion_pools(mut self, exclusion_pools: Vec<PoolType>) -> SharedData {
        self.exclusion_pools = exclusion_pools;
        return self;
    }

    //Adds a value for other custom rules to read with custom.  Only one rule can set a key.  random_numbers returns a
    //BadRequest result when two rules set the same key.
    pub fn with_custom(mut self, key: &str, value: CustomValue) -> SharedData {
        self.custom.insert(key.to_owned(), value);
        return self;
    }

    pub fn custom(&self, key: &str) -> Option<&CustomValue> {
        return self.custom.get(key);
    }

    pub fn custom_keys(&self) -> impl Iterator<Item = &String> {
        return self.custom.keys();
    }

    //Any number is allowed when no rule has shared a min or max
    pub fn min_max(&self) -> (usize, usize) {
        return (self.min.unwrap_or(usize::MIN), self.max.unwrap_or(usize::MAX));
    }

    pub fn no_duplicates(&self) -> bool {
        return self.no_duplicates;
    }

    pub fn exclusion_pools(&self) -> &[PoolType] {
        return &self.exclusion_pools;
    }

    //Adds what another rule shared.  Its min and max replace the ones that are already here.
    //Returns the key as the error, and adds nothing, when the other rule set a custom key that is already here.
    pub fn merge(&mut self, other: SharedData) -> std::result::Result<(), String> {
        if let Some(key) = other.custom.keys().filter(|x| self.custom.contains_key(*x)).min() {
            return Err(key.to_owned());
        }
        if other.min.is_some() {
            self.min = other.min;
        }
        if other.max.is_some() {
            self.max = other.max;
        }
        self.no_duplicates = self.no_duplicates || other.no_duplicates;
        self.exclusion_pools.extend(other.exclusion_pools);
        self.custom.extend(other.custom);
        return Ok(());
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum IsWithinErrorType {
    Regular,
    MakePriority,
}

pub trait RuleTrait: RuleTraitClone + Display + std::fmt::Debug + Send + Sync {
    fn is_within_range(
        &self,
        current_data: &CurrentData,
    ) -> std::result::Result<(), (IsWithinErrorType, String)>;

    fn as_any(&self) -> &dyn Any;

    fn is_match(
        &self,
        current_data: &CurrentData,
    ) -> std::result::Result<(), String>;

    //Ok(None) means this rule has no numbers to add right now
    fn get_numbers(
        &self,
        current_data: &CurrentData,
    ) -> std::result::Result<Option<Vec<usize>>, String>;

    fn share_data(
        &self,
        current_data: &CurrentData,
    ) -> Option<SharedData>;

    fn check_count(
        &self,
        count: usize,
    ) -> std::result::Result<bool, String>;

    fn name(&self) -> String;

    //Returns every problem with this rule for this count.  Settings::builder(...).build() calls it.
    //A rule only needs its own version when it can have more problems than the one from check_count.
    fn validate(&self, count: usize) -> Vec<String> {
        return match self.check_count(count) {
            Ok(_) => Vec::new(),
            Err(e) => vec![e]
        };
    }

    //Whether numbers can still be added to the selected numbers so they match.  Uniform settings throw a
    //ticket away as soon as a rule says false, so it must only say false when nothing added could make the
    //numbers match: a rule that says it too soon makes uniform settings leave results out, and
    //uniform_check::check_uniform finds where it does.  Settings that are not uniform never call it.  By
    //default it is always true, so the rule is only checked with is_match on full tickets.
    fn can_still_match(&self, _current_data: &CurrentData) -> bool {
        return true;
    }

    //For uniform settings: the numbers that can be at this index whatever else is drawn, or None when the
    //rule does not limit them.  count is how many numbers there are.  Uniform settings only draw from the
    //choices every rule gives, which is as fair as drawing others and throwing them away, so a rule must not
    //leave out a number that a match could have there.  Settings that are not uniform never call it.
    fn uniform_choices(&self, _index: usize, _count: usize) -> Option<PoolType> {
        return None;
    }
}

pub trait RuleTraitClone {
    fn clone_box(&self) -> Box<dyn RuleTrait>;
}

impl<T> RuleTraitClone for T
where
    T: 'static + RuleTrait + Clone,
{
    fn clone_box(&self) -> Box<dyn RuleTrait> {
        Box::new(self.clone())
    }
}


impl Clone for Box<dyn RuleTrait> {
    fn clone(&self) -> Box<dyn RuleTrait> {
        self.clone_box()
    }
}
