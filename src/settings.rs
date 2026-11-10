#[cfg(feature="rand")]
use crate::default_random::DefaultRandom;
use crate::random::{generate_result, RandomError, RandomNumbers};
use crate::random_trait::RandomTrait;
use crate::rules::{RuleTrait, RandomNumber, ExcludeRuleTrait, NoDuplicate, NumberPool, NumberRange, NumberSpace, OddEven};
use std::fmt;
use std::sync::Arc;

/// How many tickets uniform generation starts before it stops, unless max_draws says otherwise
pub const DEFAULT_MAX_DRAWS: usize = 1_000_000;

#[derive(Clone)]
/// What to generate: how many numbers, the rules they have to match, and the random number generator.  Create it with `Settings::builder`, and use it as many times as you want.
pub struct Settings {
    expected_rules: Vec<Box<dyn RuleTrait>>,
    count: usize,
    exclude_rules: Option<Vec<Box<dyn ExcludeRuleTrait>>>,
    max_tries: usize,
    max_is_match_attempts: usize,
    max_specific_error_count: usize,
    random: Option<Arc<dyn RandomTrait + Send + Sync>>,
    errors: Vec<String>,
    uniform: bool,
    max_draws: usize,
}

//The same as a derived Debug.  uniform and max_draws are only shown when uniform is on, so the
//settings in reports are the same as before there was uniform.
impl fmt::Debug for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Settings");
        d.field("expected_rules", &self.expected_rules)
            .field("count", &self.count)
            .field("exclude_rules", &self.exclude_rules)
            .field("max_tries", &self.max_tries)
            .field("max_is_match_attempts", &self.max_is_match_attempts)
            .field("max_specific_error_count", &self.max_specific_error_count)
            .field("random", &self.random)
            .field("errors", &self.errors);
        if self.uniform {
            d.field("uniform", &self.uniform).field("max_draws", &self.max_draws);
        }
        return d.finish();
    }
}

/// Builds Settings.  Created with Settings::builder.
#[derive(Clone, Debug)]
pub struct SettingsBuilder {
    count: usize,
    expected_rules: Vec<Box<dyn RuleTrait>>,
    exclude_rules: Vec<Box<dyn ExcludeRuleTrait>>,
    random: Option<Arc<dyn RandomTrait + Send + Sync>>,
    uniform: bool,
    max_draws: usize,
}

impl SettingsBuilder {
    /// Adds a rule that the random numbers have to match
    pub fn rule(mut self, rule: impl RuleTrait + 'static) -> SettingsBuilder {
        self.expected_rules.push(Box::new(rule));
        return self;
    }

    /// Adds rules that are already in boxes, such as a list of rules that was put together while the program runs
    pub fn boxed_rules(mut self, rules: impl IntoIterator<Item = Box<dyn RuleTrait>>) -> SettingsBuilder {
        self.expected_rules.extend(rules);
        return self;
    }

    /// Adds a rule that the random numbers can not match
    pub fn exclude(mut self, rule: impl ExcludeRuleTrait + 'static) -> SettingsBuilder {
        self.exclude_rules.push(Box::new(rule));
        return self;
    }

    /// Adds exclude rules that are already in boxes
    pub fn boxed_excludes(mut self, rules: impl IntoIterator<Item = Box<dyn ExcludeRuleTrait>>) -> SettingsBuilder {
        self.exclude_rules.extend(rules);
        return self;
    }

    /// Every result that matches the rules is equally likely.  The numbers are drawn at random, without the rules
    /// steering them, and a ticket is started again as soon as a rule says it can not match.  Rules that only a
    /// few tickets match take many draws: see max_draws.
    pub fn uniform(mut self) -> SettingsBuilder {
        self.uniform = true;
        return self;
    }

    /// With uniform, how many tickets to start before giving up.  DEFAULT_MAX_DRAWS without it.
    pub fn max_draws(mut self, max_draws: usize) -> SettingsBuilder {
        self.max_draws = max_draws;
        return self;
    }

    /// Uses this random number generator instead of DefaultRandom
    pub fn random(mut self, random: impl RandomTrait + Send + Sync + 'static) -> SettingsBuilder {
        self.random = Some(Arc::new(random));
        return self;
    }

    /// Every number is from min to max.  The same as rule(NumberRange::all(min, max)).
    pub fn range(self, min: usize, max: usize) -> SettingsBuilder {
        return self.rule(NumberRange::all(min, max));
    }

    /// No number is there more than once.  The same as rule(NoDuplicate).
    pub fn no_duplicates(self) -> SettingsBuilder {
        return self.rule(NoDuplicate);
    }

    /// This many odd numbers and this many even numbers.  The same as rule(OddEven::new().odd(odd).even(even)).
    pub fn odd_even(self, odd: usize, even: usize) -> SettingsBuilder {
        return self.rule(OddEven::new().odd(odd).even(even));
    }

    /// Returns the settings.  Problems with them are in `errors()`, and `generate` returns a `BadRequest` error for them.
    pub fn build(self) -> Settings {
        let exclude_rules = if self.exclude_rules.is_empty() { None } else { Some(self.exclude_rules) };
        let mut settings = Settings::from_parts(self.expected_rules, self.count, exclude_rules, self.random);
        settings.uniform = self.uniform;
        settings.max_draws = self.max_draws;
        if self.uniform && self.max_draws == 0 {
            settings.errors.push(String::from("max_draws must be at least 1"));
        }
        return settings;
    }
}

impl Settings {
    /// Starts the settings for "count" random numbers.  Add the rules, then call build.
    pub fn builder(count: usize) -> SettingsBuilder {
        return SettingsBuilder {
            count,
            expected_rules: Vec::new(),
            exclude_rules: Vec::new(),
            random: None,
            uniform: false,
            max_draws: DEFAULT_MAX_DRAWS,
        };
    }

    /// Settings for count numbers that match these rules.  Use `Settings::builder(count)` instead.
    #[deprecated(since = "3.0.0", note = "use Settings::builder(count).rule(...).build()")]
    pub fn new(
        expected_rules: &[Box<dyn RuleTrait>],
        count: usize,
    ) -> Settings {
        return Settings::from_parts(expected_rules.to_vec(), count, None, None);
    }

    /// Settings for count numbers that match these rules and not the exclude rules.  Use
    /// `Settings::builder(count)` instead.
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
        //NumberPool::alphanumeric picks how many digits there are with the random number generator of the settings
        let random = random.or_else(Settings::default_random);
        let random_for_rules: Option<&dyn RandomTrait> = match &random {
            Some(v) => Some(v.as_ref()),
            None => None
        };
        for rule in expected_rules_clone.iter_mut() {
            if let Some(number_pool) = rule.as_any().downcast_ref::<NumberPool>().and_then(|x| x.resolve_alphanumeric(random_for_rules)) {
                *rule = Box::new(number_pool);
            }
        }
        let mut exclude_rules = exclude_rules;
        if let Some(exclude_rules) = exclude_rules.as_mut() {
            for rule in exclude_rules.iter_mut() {
                if let Some(number_pool) = rule.as_any().downcast_ref::<NumberPool>().and_then(|x| x.resolve_alphanumeric(random_for_rules)) {
                    *rule = Box::new(number_pool);
                }
            }
        }
        let mut errors: Vec<String> = Vec::new();
        for rule in &expected_rules_clone {
            errors.extend(rule.validate(count));
        }
        if let Some(exclude_rules) = &exclude_rules {
            for rule in exclude_rules {
                errors.extend(rule.validate_exclude());
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
                if let Some(number_pool) = rule.as_any().downcast_ref::<NumberPool>() {
                    if let Some(e) = number_pool.check_no_duplicates().err() {
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
            random,
            errors,
            uniform: false,
            max_draws: DEFAULT_MAX_DRAWS,
        };
    }

    /// Creates the random numbers.  Err says why when the settings can not work or no numbers were found.
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

    /// Uses this random number generator.
    pub fn with_random(mut self, random: impl RandomTrait + Send + Sync + 'static) -> Settings {
        self.random = Some(Arc::new(random));
        return self;
    }

    /// The random number generator.
    pub fn random(&self) -> Option<&Arc<dyn RandomTrait + Send + Sync>> {
        return self.random.as_ref();
    }

    /// Problems found with the rules.  random_numbers returns a BadRequest result when there are any.
    pub fn errors(&self) -> &[String] {
        return &self.errors;
    }

    /// The rules the numbers have to match, with the `RandomNumber` rule that is added when none of them is one.
    pub fn expected_rules(&self) -> &Vec<Box<dyn RuleTrait>> {
        return &self.expected_rules;
    }

    /// The rules the numbers can not match.
    pub fn exclude_rules(&self) -> &Option<Vec<Box<dyn ExcludeRuleTrait>>> {
        return &self.exclude_rules;
    }

    /// How many numbers are generated.
    pub fn count(&self) -> usize {
        return self.count;
    }

    /// Whether every result that matches the rules is equally likely.  See `SettingsBuilder::uniform`.
    pub fn uniform(&self) -> bool {
        return self.uniform;
    }

    /// With `uniform`, how many tickets are started before giving up.
    pub fn max_draws(&self) -> usize {
        return self.max_draws;
    }

    /// Without `uniform`, how many attempts are made before giving up.
    pub fn max_tries(&self) -> usize {
        return self.max_tries;
    }

    /// How many full sets of numbers are checked before starting over.
    pub fn max_is_match_attempts(&self) -> usize {
        return self.max_is_match_attempts;
    }

    /// How many times the same error can happen before the numbers so far are cleared.
    pub fn max_specific_error_count(&self) -> usize {
        return self.max_specific_error_count;
    }
}
