#[cfg(feature="rand")]
use crate::default_random::DefaultRandom;
use crate::random_trait::RandomTrait;
use crate::rules::{RuleTrait, RandomNumber, ExcludeRuleTrait, NoDuplicate, NumberSpace};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Settings {
    expected_rules: Vec<Box<dyn RuleTrait>>,
    count: usize,
    exclude_rules: Option<Vec<Box<dyn ExcludeRuleTrait>>>,
    max_tries: usize,
    max_is_match_attempts: usize,
    max_specific_error_count: usize,
    random: Option<Arc<dyn RandomTrait + Send + Sync>>,
    errors: Vec<String>
}

impl Settings {
    pub fn new(
        expected_rules: &[Box<dyn RuleTrait>],
        count: usize,
    ) -> Settings {
        return Settings::with_exclude_rules(expected_rules, count, None)
    }

    pub fn with_exclude_rules(
        expected_rules: &[Box<dyn RuleTrait>],
        count: usize,
        exclude_rules: Option<Vec<Box<dyn ExcludeRuleTrait>>>
    ) -> Settings {
        let mut expected_rules_clone = expected_rules.to_vec();
        if expected_rules_clone.iter().all(|x| !x.as_any().is::<RandomNumber>()) {
            expected_rules_clone.push(Box::new(RandomNumber{}));
        }
        let mut errors: Vec<String> = Vec::new();
        for rule in &expected_rules_clone {
            if let Some(e) = rule.check_count(count).err() {
                errors.push(e);
            }
        }
        //checks that need to know about more than one rule
        if expected_rules_clone.iter().any(|x| x.as_any().is::<NoDuplicate>()) {
            for rule in &expected_rules_clone {
                if let Some(number_space) = rule.as_any().downcast_ref::<NumberSpace>() {
                    if let Some(e) = number_space.check_no_duplicates(count).err() {
                        errors.push(e);
                    }
                }
            }
        }
        return Settings {
            expected_rules: expected_rules_clone,
            count,
            exclude_rules,
            max_tries: 500,
            max_is_match_attempts: 10,
            max_specific_error_count: 3 * count,
            random: Settings::default_random(),
            errors
        };
    }

    #[cfg(feature="rand")]
    fn default_random() -> Option<Arc<dyn RandomTrait + Send + Sync>> {
        return Some(Arc::new(DefaultRandom{}));
    }

    #[cfg(not(feature="rand"))]
    fn default_random() -> Option<Arc<dyn RandomTrait + Send + Sync>> {
        return None;
    }

    pub fn with_random(mut self, random: impl RandomTrait + Send + Sync + 'static) -> Settings {
        self.random = Some(Arc::new(random));
        return self;
    }

    pub fn random(&self) -> Option<&Arc<dyn RandomTrait + Send + Sync>> {
        return self.random.as_ref();
    }

    //Problems found with the rules.  random_numbers returns a BadRequest result when there are any.
    pub fn errors(&self) -> &[String] {
        return &self.errors;
    }

    pub fn expected_rules(&self) -> &Vec<Box<dyn RuleTrait>> {
        return &self.expected_rules;
    }

    pub fn exclude_rules(&self) -> &Option<Vec<Box<dyn ExcludeRuleTrait>>> {
        return &self.exclude_rules;
    }

    pub fn count(&self) -> usize {
        return self.count;
    }

    pub fn max_tries(&self) -> usize {
        return self.max_tries;
    }

    pub fn max_is_match_attempts(&self) -> usize {
        return self.max_is_match_attempts;
    }

    pub fn max_specific_error_count(&self) -> usize {
        return self.max_specific_error_count;
    }
}
