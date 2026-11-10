use crate::random::CurrentData;
use crate::rules::PoolType;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Display;

/// A value that custom rules share with each other.  See SharedData::with_custom.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CustomValue {
    /// A number.
    Usize(usize),
    /// Text.
    String(String),
    /// True or false.
    Bool(bool),
}

/// What the rules tell each other, and CurrentData::get_rand, about the next number
#[derive(Debug, Clone, Default)]
pub struct SharedData {
    min: Option<usize>,
    max: Option<usize>,
    no_duplicates: bool,
    exclusion_pools: Vec<PoolType>,
    custom: HashMap<String, CustomValue>,
}

impl SharedData {
    /// Nothing shared yet.
    pub fn new() -> SharedData {
        return SharedData::default();
    }

    /// The next number is from min to max.
    pub fn with_min_max(mut self, min: usize, max: usize) -> SharedData {
        self.min = Some(min);
        self.max = Some(max);
        return self;
    }

    /// No number can be there more than once.
    pub fn with_no_duplicates(mut self) -> SharedData {
        self.no_duplicates = true;
        return self;
    }

    /// The next number can not be from any of these pools
    pub fn with_exclusion_pools(mut self, exclusion_pools: Vec<PoolType>) -> SharedData {
        self.exclusion_pools = exclusion_pools;
        return self;
    }

    /// Adds a value for other custom rules to read with custom.  Only one rule can set a key.  random_numbers returns a
    /// BadRequest result when two rules set the same key.
    pub fn with_custom(mut self, key: &str, value: CustomValue) -> SharedData {
        self.custom.insert(key.to_owned(), value);
        return self;
    }

    /// The value a rule shared under this key with `with_custom`.
    pub fn custom(&self, key: &str) -> Option<&CustomValue> {
        return self.custom.get(key);
    }

    /// Every key a rule shared a value under.
    pub fn custom_keys(&self) -> impl Iterator<Item = &String> {
        return self.custom.keys();
    }

    /// Any number is allowed when no rule has shared a min or max
    pub fn min_max(&self) -> (usize, usize) {
        return (self.min.unwrap_or(usize::MIN), self.max.unwrap_or(usize::MAX));
    }

    /// Whether a number can not be there more than once.
    pub fn no_duplicates(&self) -> bool {
        return self.no_duplicates;
    }

    /// Pools the next number can not come from.
    pub fn exclusion_pools(&self) -> &[PoolType] {
        return &self.exclusion_pools;
    }

    /// Adds what another rule shared.  Its min and max replace the ones that are already here.
    /// Returns the key as the error, and adds nothing, when the other rule set a custom key that is already here.
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
/// Why `is_within_range` said no.
pub enum IsWithinErrorType {
    /// The numbers so far can not work.
    Regular,
    /// The numbers so far can not work unless this rule gets the next numbers, so it is asked first next time.
    MakePriority,
}

/// A rule the numbers have to match.  Every rule in adv_random implements it, and your own rules can too: see Custom Rules in the README.
pub trait RuleTrait: RuleTraitClone + Display + std::fmt::Debug + Send + Sync {
    /// Without uniform settings, called every time numbers are added: an error when the numbers so far can not work.
    fn is_within_range(&self, current_data: &CurrentData) -> std::result::Result<(), (IsWithinErrorType, String)>;

    /// The rule as `Any`, to find out which rule it is.
    fn as_any(&self) -> &dyn Any;

    /// Called when all of the numbers are there: an error when they don't match.
    fn is_match(&self, current_data: &CurrentData) -> std::result::Result<(), String>;

    /// Ok(None) means this rule has no numbers to add right now
    fn get_numbers(&self, current_data: &CurrentData) -> std::result::Result<Option<Vec<usize>>, String>;

    /// What the rule tells the other rules about the next number, such as its range.
    fn share_data(&self, current_data: &CurrentData) -> Option<SharedData>;

    /// An error when the count can not work with this rule.
    fn check_count(&self, count: usize) -> std::result::Result<bool, String>;

    /// The rule's name, for logs and errors.
    fn name(&self) -> String;

    /// Returns every problem with this rule for this count.  Settings::builder(...).build() calls it.
    /// A rule only needs its own version when it can have more problems than the one from check_count.
    fn validate(&self, count: usize) -> Vec<String> {
        return match self.check_count(count) {
            Ok(_) => Vec::new(),
            Err(e) => vec![e],
        };
    }

    /// Whether numbers can still be added to the selected numbers so they match.  Uniform settings throw a
    /// ticket away as soon as a rule says false, so it must only say false when nothing added could make the
    /// numbers match: a rule that says it too soon makes uniform settings leave results out, and
    /// uniform_check::check_uniform finds where it does.  Settings that are not uniform never call it.  By
    /// default it is always true, so the rule is only checked with is_match on full tickets.  When it says no,
    /// the log names the rule, which makes its Display once a generate, so keep Display quick.
    fn can_still_match(&self, _current_data: &CurrentData) -> bool {
        return true;
    }

    /// For uniform settings: the numbers that can be at this index whatever else is drawn, or None when the
    /// rule does not limit them.  count is how many numbers there are.  Uniform settings only draw from the
    /// choices every rule gives, which is as fair as drawing others and throwing them away, so a rule must not
    /// leave out a number that a match could have there.  Settings that are not uniform never call it.
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
