use crate::random::CurrentData;
use std::any::Any;
use std::fmt::Display;

pub fn is_excluded_helper(is_match_results: &std::result::Result<(), String>, rule_str: &str) -> std::result::Result<(), String> {
    return match is_match_results {
        Ok(()) => Err(format!("Number Contains Rule {}, that should be excluded.", rule_str)),
        Err(_) => Ok(()),
    };
}

pub trait ExcludeRuleTrait: ExcludeRuleTraitClone + Display + std::fmt::Debug {

    fn as_any(&self) -> &dyn Any;

    fn is_excluded(
        &self,
        current_data: &CurrentData,
    ) -> std::result::Result<(), String>;
}

pub trait ExcludeRuleTraitClone {
    fn clone_box(&self) -> Box<dyn ExcludeRuleTrait>;
}

impl<T> ExcludeRuleTraitClone for T
where
    T: 'static + ExcludeRuleTrait + Clone,
{
    fn clone_box(&self) -> Box<dyn ExcludeRuleTrait> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn ExcludeRuleTrait> {
    fn clone(&self) -> Box<dyn ExcludeRuleTrait> {
        self.clone_box()
    }
}
