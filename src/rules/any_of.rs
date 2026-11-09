use crate::random::CurrentData;
use crate::rules::{IsWithinErrorType, RuleTrait, SharedData};
use std::any::Any;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

//Matches when any one of its rules matches, such as one of a few odd/even counts.  A ticket can still
//match while any one of them can.  It gets no numbers itself, so it suits uniform settings best.
#[derive(Clone, Default)]
pub struct AnyOf {
    rules: Vec<Box<dyn RuleTrait>>,
}

impl AnyOf {
    //No rules yet.  Add them with rule.
    pub fn new() -> AnyOf {
        return AnyOf::default();
    }

    //Adds a rule that can match
    pub fn rule(mut self, rule: impl RuleTrait + 'static) -> AnyOf {
        self.rules.push(Box::new(rule));
        return self;
    }

    //Adds rules that are already in boxes
    pub fn boxed_rules(mut self, rules: impl IntoIterator<Item = Box<dyn RuleTrait>>) -> AnyOf {
        self.rules.extend(rules);
        return self;
    }

    pub fn rules(&self) -> &Vec<Box<dyn RuleTrait>> {
        return &self.rules;
    }
}

impl Display for AnyOf {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}[{}]", self.name(), self.rules.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" | "))
    }
}

impl Debug for AnyOf {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl RuleTrait for AnyOf {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn share_data(&self, _current_data: &CurrentData) -> Option<SharedData> {
        return None;
    }

    fn get_numbers(&self, _current_data: &CurrentData) -> std::result::Result<Option<Vec<usize>>, String> {
        return Ok(None);
    }

    fn is_within_range(&self, current_data: &CurrentData) -> std::result::Result<(), (IsWithinErrorType, String)> {
        let mut first_error = None;
        for rule in &self.rules {
            match rule.is_within_range(current_data) {
                Ok(()) => return Ok(()),
                Err(e) => { first_error.get_or_insert(e.1); }
            }
        }
        return Err((IsWithinErrorType::Regular, format!("{} none of its rules can match.  The first: {}", self.name(), first_error.unwrap_or_default())));
    }

    fn is_match(&self, current_data: &CurrentData) -> std::result::Result<(), String> {
        let mut first_error = None;
        for rule in &self.rules {
            match rule.is_match(current_data) {
                Ok(()) => return Ok(()),
                Err(e) => { first_error.get_or_insert(e); }
            }
        }
        return Err(format!("{} none of its rules match.  The first: {}", self.name(), first_error.unwrap_or_default()));
    }

    fn check_count(&self, count: usize) -> std::result::Result<bool, String> {
        for rule in &self.rules {
            rule.check_count(count)?;
        }
        return Ok(true);
    }

    fn validate(&self, count: usize) -> Vec<String> {
        if self.rules.is_empty() {
            return vec![format!("{} has no rules", self.name())];
        }
        return self.rules.iter().flat_map(|x| x.validate(count)).collect();
    }

    //It can still match while any one of its rules can
    fn can_still_match(&self, current_data: &CurrentData) -> bool {
        return self.rules.iter().any(|x| x.can_still_match(current_data));
    }

    fn name(&self) -> String {
        return String::from("AnyOf");
    }
}
