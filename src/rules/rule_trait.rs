use crate::random::CurrentData;
use crate::rules::PoolType;
use std::any::Any;
use std::fmt::Display;

//What the rules tell each other, and CurrentData::get_rand, about the next number
#[derive(Debug, Clone, Default)]
pub struct SharedData {
    min: Option<usize>,
    max: Option<usize>,
    no_duplicates: bool,
    exclusion_pools: Vec<PoolType>,
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
    pub fn merge(&mut self, other: SharedData) {
        if other.min.is_some() {
            self.min = other.min;
        }
        if other.max.is_some() {
            self.max = other.max;
        }
        self.no_duplicates = self.no_duplicates || other.no_duplicates;
        self.exclusion_pools.extend(other.exclusion_pools);
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
