use crate::random::CurrentData;
use std::any::Any;
use std::fmt::Display;

use super::IsWithinErrorType;

/// For an exclude rule made from a rule: an error when the rule matches, so the numbers are left out.
pub fn is_excluded_helper(is_match_results: &std::result::Result<(), String>, rule_str: &str) -> std::result::Result<(), String> {
    return match is_match_results {
        Ok(()) => Err(format!("Number Contains Rule {}, that should be excluded.", rule_str)),
        Err(_) => Ok(()),
    };
}

/// A rule the numbers can not match, added with `.exclude(...)`.
pub trait ExcludeRuleTrait: ExcludeRuleTraitClone + Display + std::fmt::Debug + Send + Sync {

    /// The rule as `Any`, to find out which rule it is.
    fn as_any(&self) -> &dyn Any;

    /// Without uniform settings, called every time numbers are added: an error when the numbers so far are sure to be left out.
    fn is_within_excluded_range(
        &self,
        current_data: &CurrentData,
    ) -> std::result::Result<(), (IsWithinErrorType, String)>;

    /// Called when all of the numbers are there: an error when they are left out.
    fn is_excluded(
        &self,
        current_data: &CurrentData,
    ) -> std::result::Result<(), String>;

    /// The rule's name, for logs and errors.
    fn exclude_name(&self) -> String;

    /// The same as RuleTrait::can_still_match, for whether numbers can still be added so the rule does not
    /// exclude them.  By default it is always true, so the rule is only checked with is_excluded on full tickets.
    fn can_still_match_excluded(&self, _current_data: &CurrentData) -> bool {
        return true;
    }

    /// Returns every problem with how this rule was created.  Settings::builder(...).build() calls it.
    fn validate_exclude(&self) -> Vec<String> {
        return Vec::new();
    }
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
