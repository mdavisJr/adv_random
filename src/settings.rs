#[cfg(feature="rand")]
use crate::default_random::DefaultRandom;
use crate::random::{generate_result, RandomError, RandomNumbers};
use crate::random_trait::RandomTrait;
use crate::rules::{RuleTrait, RandomNumber, ExcludeRuleTrait, NoDuplicate, NumberRange, NumberSpace, OddEven};
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

//Builds Settings.  Created with Settings::builder.
#[derive(Clone, Debug)]
pub struct SettingsBuilder {
    count: usize,
    expected_rules: Vec<Box<dyn RuleTrait>>,
    exclude_rules: Vec<Box<dyn ExcludeRuleTrait>>,
    random: Option<Arc<dyn RandomTrait + Send + Sync>>,
}

impl SettingsBuilder {
    //Adds a rule that the random numbers have to match
    pub fn rule(mut self, rule: impl RuleTrait + 'static) -> SettingsBuilder {
        self.expected_rules.push(Box::new(rule));
        return self;
    }

    //Adds rules that are already in boxes, such as a list of rules that was put together while the program runs
    pub fn boxed_rules(mut self, rules: impl IntoIterator<Item = Box<dyn RuleTrait>>) -> SettingsBuilder {
        self.expected_rules.extend(rules);
        return self;
    }

    //Adds a rule that the random numbers can not match
    pub fn exclude(mut self, rule: impl ExcludeRuleTrait + 'static) -> SettingsBuilder {
        self.exclude_rules.push(Box::new(rule));
        return self;
    }

    //Adds exclude rules that are already in boxes
    pub fn boxed_excludes(mut self, rules: impl IntoIterator<Item = Box<dyn ExcludeRuleTrait>>) -> SettingsBuilder {
        self.exclude_rules.extend(rules);
        return self;
    }

    //Uses this random number generator instead of DefaultRandom
    pub fn random(mut self, random: impl RandomTrait + Send + Sync + 'static) -> SettingsBuilder {
        self.random = Some(Arc::new(random));
        return self;
    }

    //Every number is from min to max.  The same as rule(NumberRange::all(min, max)).
    pub fn range(self, min: usize, max: usize) -> SettingsBuilder {
        return self.rule(NumberRange::all(min, max));
    }

    //No number is there more than once.  The same as rule(NoDuplicate).
    pub fn no_duplicates(self) -> SettingsBuilder {
        return self.rule(NoDuplicate);
    }

    //This many odd numbers and this many even numbers.  The same as rule(OddEven::new(odd, even)).
    pub fn odd_even(self, odd: usize, even: usize) -> SettingsBuilder {
        return self.rule(OddEven::new(odd, even));
    }

    pub fn build(self) -> Settings {
        let exclude_rules = if self.exclude_rules.is_empty() { None } else { Some(self.exclude_rules) };
        return Settings::from_parts(self.expected_rules, self.count, exclude_rules, self.random);
    }
}

impl Settings {
    //Starts the settings for "count" random numbers.  Add the rules, then call build.
    pub fn builder(count: usize) -> SettingsBuilder {
        return SettingsBuilder {
            count,
            expected_rules: Vec::new(),
            exclude_rules: Vec::new(),
            random: None,
        };
    }

    #[deprecated(since = "3.0.0", note = "use Settings::builder(count).rule(...).build()")]
    pub fn new(
        expected_rules: &[Box<dyn RuleTrait>],
        count: usize,
    ) -> Settings {
        return Settings::from_parts(expected_rules.to_vec(), count, None, None);
    }

    #[deprecated(since = "3.0.0", note = "use Settings::builder(count).rule(...).exclude(...).build()")]
    pub fn with_exclude_rules(
        expected_rules: &[Box<dyn RuleTrait>],
        count: usize,
        exclude_rules: Option<Vec<Box<dyn ExcludeRuleTrait>>>
    ) -> Settings {
        return Settings::from_parts(expected_rules.to_vec(), count, exclude_rules, None);
    }

    fn from_parts(
        expected_rules: Vec<Box<dyn RuleTrait>>,
        count: usize,
        exclude_rules: Option<Vec<Box<dyn ExcludeRuleTrait>>>,
        random: Option<Arc<dyn RandomTrait + Send + Sync>>
    ) -> Settings {
        let mut expected_rules_clone = expected_rules;
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
            random: random.or_else(Settings::default_random),
            errors
        };
    }

    //Creates the random numbers.  Err says why when the settings can not work or no numbers were found.
    pub fn generate(&self) -> Result<RandomNumbers, RandomError> {
        return generate_result(self).into_result();
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
