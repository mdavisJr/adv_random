use std::sync::{Arc, OnceLock};

use crate::random_trait::{get_random_vec_item, shuffle_vec, RandomTrait};
use crate::rules::{
    ExcludeRuleTrait, IsWithinErrorType, OddEven, PoolType, RuleTrait, SharedData
};
use crate::settings::Settings;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Debug, Formatter, Result};
use std::hash::Hash;

pub const NO_NUMBER_FOUND_ERR: &str = "Could not find a number that fits the rules";
pub const NO_RANDOM_ERR: &str = "No random number generator. Please call Settings::with_random or use feature \"rand\"";

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum RandomResultType {
    Success,
    BadRequest,
    Failed,
}

pub struct RandomResult {
    status: RandomResultType,
    numbers: Vec<usize>,
    attempts: usize,
    logs: Vec<Log>,
    clear_err_tracker: HashSet<usize>,
    settings: String,
    random: Option<Arc<dyn RandomTrait + Send + Sync>>
}

impl Debug for RandomResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self)
    }
}

impl RandomResult {
    pub fn status(&self) -> RandomResultType {
        return self.status;
    }

    pub fn numbers(&self) -> std::result::Result<&Vec<usize>, String> {
        match self.status {
            RandomResultType::Success => {
                return Ok(&self.numbers);
            },
            _ => Err("Error! Please Check Logs!".to_string())
        }
    }

    //The numbers as characters, in the order they were created
    pub fn string(&self) -> std::result::Result<String, String> {
        match self.status {
            RandomResultType::Success => numbers_to_string(&self.numbers),
            _ => Err("Error! Please Check Logs!".to_string())
        }
    }

    //The numbers as characters, in a random order
    pub fn shuffled_string(&self) -> std::result::Result<String, String> {
        match self.status {
            RandomResultType::Success => numbers_to_string(&shuffled_numbers(&self.numbers, &self.random)),
            _ => Err("Error! Please Check Logs!".to_string())
        }
    }

    //Ok with the numbers when the status is Success, and Err with what went wrong when it is not
    pub fn into_result(self) -> std::result::Result<RandomNumbers, RandomError> {
        return match self.status {
            RandomResultType::Success => Ok(RandomNumbers {
                numbers: self.numbers,
                attempts: self.attempts,
                logs: self.logs,
                random: self.random
            }),
            RandomResultType::BadRequest => Err(RandomError {
                error_type: RandomErrorType::BadRequest,
                messages: self.logs.iter().filter_map(|x| if let Log::Error { msg } = x { Some(msg.to_owned()) } else { None }).collect(),
                attempts: self.attempts,
                logs: self.logs,
                settings: self.settings
            }),
            RandomResultType::Failed => Err(RandomError {
                error_type: RandomErrorType::Failed,
                messages: vec![format!("Could not find numbers that match the rules in {} attempts", self.attempts)],
                attempts: self.attempts,
                logs: self.logs,
                settings: self.settings
            })
        };
    }
    pub fn attempts(&self) -> usize {
        return self.attempts;
    }
    pub fn logs(&self) -> &Vec<Log> {
        return &self.logs;
    }
    pub fn clear_err_tracker(&self) -> &HashSet<usize> {
        return &self.clear_err_tracker;
    }
}

impl fmt::Display for RandomResult {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut sorted_clear_err_tracker: Vec<usize> = self.clear_err_tracker().iter().cloned().collect();
        sorted_clear_err_tracker.sort_unstable();      
        write!(
            fmt,
            "\n\nSettings:\n\n{}\n\nStatus - {:?}\nNumbers - {:?}\nAttempts - {}\nClear Errs - {:?}\n{}",
            self.settings,
            self.status(),
            self.numbers(),
            self.attempts,
            sorted_clear_err_tracker,
            self.logs
                .iter()
                .map(|x| if x.to_string().starts_with("Info  : Attempt - ") {
                    format!("\n{}", x)
                } else {
                    x.to_string()
                })
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

fn numbers_to_string(numbers: &[usize]) -> std::result::Result<String, String> {
    let mut random_string = String::new();
    for digit in numbers {
        match u32::try_from(*digit).ok().and_then(char::from_u32) {
            Some(c) => random_string.push(c),
            None => return Err(format!("Number {} is not a valid character", digit))
        }
    }
    return Ok(random_string);
}

fn shuffled_numbers(numbers: &[usize], random: &Option<Arc<dyn RandomTrait + Send + Sync>>) -> Vec<usize> {
    let mut numbers = numbers.to_vec();
    if let Some(random) = random {
        shuffle_vec(random.as_ref(), &mut numbers);
    }
    return numbers;
}

//The random numbers from Settings::generate
pub struct RandomNumbers {
    numbers: Vec<usize>,
    attempts: usize,
    logs: Vec<Log>,
    random: Option<Arc<dyn RandomTrait + Send + Sync>>
}

impl RandomNumbers {
    pub fn numbers(&self) -> &Vec<usize> {
        return &self.numbers;
    }

    pub fn into_numbers(self) -> Vec<usize> {
        return self.numbers;
    }

    //The numbers as characters, in the order they were created
    pub fn string(&self) -> std::result::Result<String, RandomError> {
        return numbers_to_string(&self.numbers).map_err(|x| self.invalid_character(x));
    }

    //The numbers as characters, in a random order
    pub fn shuffled_string(&self) -> std::result::Result<String, RandomError> {
        return numbers_to_string(&shuffled_numbers(&self.numbers, &self.random)).map_err(|x| self.invalid_character(x));
    }

    pub fn attempts(&self) -> usize {
        return self.attempts;
    }

    pub fn logs(&self) -> &Vec<Log> {
        return &self.logs;
    }

    fn invalid_character(&self, message: String) -> RandomError {
        return RandomError {
            error_type: RandomErrorType::InvalidCharacter,
            messages: vec![message],
            attempts: self.attempts,
            logs: Vec::new(),
            settings: String::new()
        };
    }
}

impl Debug for RandomNumbers {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "RandomNumbers {{ numbers: {:?}, attempts: {} }}", self.numbers, self.attempts)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum RandomErrorType {
    //The settings can not work.  The messages say why.
    BadRequest,
    //No numbers that match the rules were found in the attempts that are allowed
    Failed,
    //A number is not a valid character, so the numbers can not be a string
    InvalidCharacter,
}

//Why Settings::generate, or a string from its numbers, did not work
pub struct RandomError {
    error_type: RandomErrorType,
    messages: Vec<String>,
    attempts: usize,
    logs: Vec<Log>,
    settings: String
}

impl RandomError {
    pub fn error_type(&self) -> RandomErrorType {
        return self.error_type;
    }

    pub fn messages(&self) -> &Vec<String> {
        return &self.messages;
    }

    pub fn attempts(&self) -> usize {
        return self.attempts;
    }

    //Everything that happened in every attempt
    pub fn logs(&self) -> &Vec<Log> {
        return &self.logs;
    }
}

//The short version, such as "BadRequest: NumberRange min: 20 is greater than max: 1"
impl fmt::Display for RandomError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}: {}", self.error_type, self.messages.join("; "))
    }
}

//The long version, with the settings and the logs
impl Debug for RandomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}\n\nSettings:\n\n{}\n\nAttempts - {}\n{}",
            self,
            self.settings,
            self.attempts,
            self.logs.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n")
        )
    }
}

impl std::error::Error for RandomError {}

fn set_numbers(
    numbers: &mut Vec<usize>,
    _settings: &Settings,
    ds: &[usize],
) {
    numbers.clear();
    numbers.extend(ds);
}

fn clear_numbers(
    logs: &mut Vec<Log>,
    numbers: &mut Vec<usize>,
    _settings: &Settings,
) {
    logs.push(Log::Info {
        msg: format!("CLEAR - {:?}", numbers),
    });
    numbers.clear();
}

#[derive(Debug, Clone)]
pub enum Log {
    Error { msg: String },
    Info { msg: String },
    UserInfo { msg: String },
}

impl fmt::Display for Log {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Log::Error { msg } => write!(fmt, "Error : {}", msg.to_owned())?,
            Log::Info { msg } => write!(fmt, "Info  : {}", msg.to_owned())?,
            Log::UserInfo { msg } => write!(fmt, "UserInfo  : {}", msg.to_owned())?,
        }
        Ok(())
    }
}

//A rule in the settings.  The number is its index in expected_rules or exclude_rules.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum RuleRef {
    Expected(usize),
    Exclude(usize),
}

//What went wrong and for which rule.  The numbers are reset when the same one happens too many times.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum ErrKey {
    GetNumbers(usize),
    IsWithinRange(RuleRef),
    IsMatch(RuleRef),
    TooManyNumbers,
}

#[allow(clippy::too_many_arguments)]
fn halt_from_error(
    err_key: ErrKey,
    err: &str,
    logs: &mut Vec<Log>,
    err_tracker: &mut HashMap<ErrKey, usize>,
    attempts: usize,
    numbers: &mut Vec<usize>,
    settings: &Settings,
    clear_err_tracker: &mut HashSet<usize>,
) {
    logs.push(Log::Error {
        msg: err.to_owned(),
    });
    let count = *err_tracker
        .entry(err_key)
        .and_modify(|x| *x += 1)
        .or_insert(1);
    if count > (settings.max_specific_error_count()) {
        logs.push(Log::Error {
            msg: format!("Reset Because Of Too Many Same Errors - {:?}: {}", err_key, err),
        });
        reset(logs, numbers, settings, err_tracker);
        clear_err_tracker.insert(attempts);
    }
}

fn reset(
    logs: &mut Vec<Log>,
    numbers: &mut Vec<usize>,
    settings: &Settings,
    err_tracker: &mut HashMap<ErrKey, usize>,
) {
    clear_numbers(logs, numbers, settings);
    err_tracker.clear();
}

#[derive(Debug, Clone, Copy)]
pub enum NumberType {
    Any,
    Odd,
    Even
}

#[derive(Debug, Clone)]
pub struct CurrentData<'a> {
    selected_numbers: &'a Vec<usize>,
    selected_numbers_set: OnceLock<HashSet<usize>>,
    selected_numbers_sorted: OnceLock<Vec<usize>>,
    settings: &'a Settings,
    shared_data: &'a SharedData,
}

impl<'a> CurrentData<'a> {

    pub fn new(selected_numbers: &'a Vec<usize>, settings: &'a Settings, shared_data: &'a SharedData) -> CurrentData<'a> {
        return CurrentData { selected_numbers, settings, shared_data, selected_numbers_set: OnceLock::new(), selected_numbers_sorted: OnceLock::new() };
    }

    pub fn from_current_data(current_data: &'a CurrentData, shared_data: &'a SharedData) -> CurrentData<'a> {
        let new_current_data = CurrentData { 
            selected_numbers: current_data.selected_numbers, 
            settings: current_data.settings, 
            shared_data, 
            selected_numbers_set: OnceLock::new(), 
            selected_numbers_sorted: OnceLock::new(), 
        };
        if current_data.selected_numbers_set.get().is_some() {
            let _ = new_current_data.selected_numbers_set.set(current_data.selected_numbers_set.get().unwrap().clone());
        }
        if current_data.selected_numbers_sorted.get().is_some() {
            let _ = new_current_data.selected_numbers_sorted.set(current_data.selected_numbers_sorted.get().unwrap().to_vec());
        }
        return new_current_data;
    }

    //Returns None when no number that fits could be found
    pub fn get_rand(&self, number_type: NumberType, from_pool_type: Option<&PoolType>, min_max: Option<(usize, usize)>) -> Option<usize> {
        let random = self.random()?;
        let (min, max) = min_max.unwrap_or_else(|| self.shared_data.min_max());
        let no_duplicates = self.shared_data.no_duplicates();
        let exclusion_pools = self.shared_data.exclusion_pools();
        for _ in 1..=30 {
            let val = if let Some(v) = &from_pool_type {
                match &v {
                    PoolType::Set(set) => {
                        *get_random_vec_item(random, set.numbers())?
                    }
                    PoolType::MinMax(pool_type_min, pool_type_max) => {
                        if pool_type_min > pool_type_max {
                            return None;
                        }
                        random.get_number(*pool_type_min, *pool_type_max)
                    }
                }
            } else {
                if min > max {
                    return None;
                }
                random.get_number(min, max)
            };
            if exclusion_pools.iter().any(|x| x.contains(val)) {
                continue;
            }
            if no_duplicates && self.selected_numbers_set().contains(&val) {
                continue;
            }
            match number_type {
                NumberType::Any => {},
                NumberType::Odd => {
                    if OddEven::is_even(val) {
                        continue;
                    }
                },
                NumberType::Even => {
                    if OddEven::is_odd(val) {
                        continue;
                    }
                }
            }

            return Some(val);
        }

        return None;
    }

    pub fn selected_numbers(&self) -> &'a Vec<usize> {
        return self.selected_numbers;
    }

    pub fn settings(&self) -> &'a Settings {
        return self.settings;
    }

    //Returns None when the settings have no random number generator
    pub fn random(&self) -> Option<&'a dyn RandomTrait> {
        return match self.settings.random() {
            Some(v) => Some(v.as_ref()),
            None => None,
        };
    }

    //The error for a rule to return when get_rand returns None
    pub fn no_number_err(&self) -> String {
        return if self.random().is_none() { NO_RANDOM_ERR } else { NO_NUMBER_FOUND_ERR }.to_owned();
    }

    pub fn shared_data(&self) -> &'a SharedData {
        return self.shared_data;
    }

    pub fn selected_numbers_set(&self) -> &HashSet<usize> {
        return self.selected_numbers_set.get_or_init(|| {
            self.selected_numbers.iter().copied().collect()
        });
    }

    pub fn selected_numbers_sorted(&self) -> &Vec<usize> {
        return self.selected_numbers_sorted.get_or_init(|| {
            let mut selected_numbers = self.selected_numbers.to_vec();
            selected_numbers.sort_unstable();
            return selected_numbers;
        });
    }  
}

pub fn random_numbers(settings: &Settings) -> RandomResult {
    let mut errors: Vec<Log> = settings.errors().iter().map(|x| Log::Error { msg: x.to_owned() }).collect();
    if settings.random().is_none() {
        errors.push(Log::Error { msg: NO_RANDOM_ERR.to_owned() });
    }
    let random: Arc<dyn RandomTrait + Send + Sync> = match settings.random() {
        Some(v) if errors.is_empty() => v.clone(),
        _ => {
            return RandomResult {
                status: RandomResultType::BadRequest,
                numbers: Vec::new(),
                attempts: 0,
                logs: errors,
                clear_err_tracker: HashSet::new(),
                settings: format!("{:#?}", settings),
                random: None
            };
        }
    };
    if settings.count() == 0 {
        return RandomResult {
            status: RandomResultType::Success,
            numbers: Vec::new(),
            attempts: 0,
            logs: Vec::new(),
            clear_err_tracker: HashSet::new(),
            settings: String::new(),
            random: Some(random)
        };
    }
    let mut numbers: Vec<usize> = Vec::new();
    let mut is_match_attempts: usize = 0;
    let mut err_tracker: HashMap<ErrKey, usize> = HashMap::new();
    let mut num_attempts = 1;
    let mut logs: Vec<Log> = Vec::new();
    let mut clear_err_tracker: HashSet<usize> = HashSet::new();
    //each rule is paired with its index in settings.expected_rules() so it can be found again after a shuffle
    let mut expected_rules: Vec<(usize, &dyn RuleTrait)> = settings.expected_rules().iter().map(|x| x.as_ref()).enumerate().collect();
    let mut rule_to_make_priority: Option<usize> = None;
    clear_numbers(&mut logs, &mut numbers, settings);
    for attempts in 1..=settings.max_tries() {
        logs.push(Log::Info {
            msg: format!("Attempt - {:?}", attempts),
        });
        num_attempts = attempts;
        let mut potential_numbers = Vec::new();
        let mut gen_type = String::from("");

        let current_data_numbers: Vec<usize> = numbers.to_vec();
        let current_data_shared_data = SharedData::new();
        let current_data_selected_numbers_sd = CurrentData::new(&current_data_numbers, settings, &current_data_shared_data);
        let mut shared_data = SharedData::new();
        //which rule set each custom key in this attempt
        let mut custom_key_rule_names: HashMap<String, String> = HashMap::new();
        shuffle_vec(random.as_ref(), &mut expected_rules);
        if let Some(v) = rule_to_make_priority.take() {
            if let Some(idx) = expected_rules.iter().position(|x| x.0 == v) {
                expected_rules.swap(0, idx);
            }
        }

        for (_, expected_rule) in &expected_rules {
            if let Some(actual_rule_shared_data) =
                expected_rule.share_data(&current_data_selected_numbers_sd)
            {
                let custom_keys: Vec<String> = actual_rule_shared_data.custom_keys().cloned().collect();
                if let Err(key) = shared_data.merge(actual_rule_shared_data) {
                    //two rules that set the same custom key is a mistake in the rules, so trying again will not help
                    logs.push(Log::Error {
                        msg: format!(
                            "Custom shared data key {:?} was set by both {} and {}",
                            key, custom_key_rule_names.get(&key).map(|x| x.as_str()).unwrap_or("another rule"), expected_rule.name()
                        ),
                    });
                    return RandomResult {
                        status: RandomResultType::BadRequest,
                        numbers: Vec::new(),
                        attempts,
                        logs,
                        clear_err_tracker,
                        settings: format!("{:#?}", settings),
                        random: None
                    };
                }
                for key in custom_keys {
                    custom_key_rule_names.insert(key, expected_rule.name());
                }
            }
        }

        let current_data_selected_numbers_gn = CurrentData::from_current_data(&current_data_selected_numbers_sd, &shared_data);
        for (idx, expected_rule) in &expected_rules {
            match expected_rule.get_numbers(&current_data_selected_numbers_gn) {
                Ok(Some(v)) => {
                    gen_type = expected_rule.name();
                    potential_numbers.extend(&v);
                    break;
                }
                Ok(None) => {}
                Err(e) => {
                    halt_from_error(
                        ErrKey::GetNumbers(*idx),
                        &format!("{} - {}", expected_rule.name(), e),
                        &mut logs,
                        &mut err_tracker,
                        attempts,
                        &mut numbers,
                        settings,
                        &mut clear_err_tracker,
                    );
                }
            }
        }

        //Check if potential_numbers are valid
        if !potential_numbers.is_empty() {
            let _temp_numbers = numbers
            .iter()
            .copied()
            .chain(potential_numbers.iter().copied())
            .collect::<Vec<usize>>();
            let current_data_with_potential_numbers = CurrentData::new(&_temp_numbers, settings, &shared_data);

            logs.push(Log::Info {
                msg: format!(
                    "GEN_TYPE - {}; P - {:?}; A&P - {:?}",
                    gen_type, potential_numbers, current_data_with_potential_numbers.selected_numbers
                ),
            });

            if current_data_with_potential_numbers.selected_numbers().len() > settings.count() {
                halt_from_error(
                    ErrKey::TooManyNumbers,
                    "Too Many Numbers Selected.",
                    &mut logs,
                    &mut err_tracker,
                    attempts,
                    &mut numbers,
                    settings,
                    &mut clear_err_tracker,
                );
                continue;
            }

            match is_within_range_check(&expected_rules, settings.exclude_rules(), settings, &current_data_with_potential_numbers, &mut logs) {
                Ok(_) => {
                    if current_data_with_potential_numbers.selected_numbers().len() == settings.count() {
                        is_match_attempts += 1;

                        match is_match_check(&expected_rules, settings.exclude_rules(), settings, &current_data_with_potential_numbers, &mut logs) {
                            Ok(_) => {
                                set_numbers(
                                    &mut numbers,
                                    settings,
                                    current_data_with_potential_numbers.selected_numbers(),
                                );
                                return RandomResult {
                                    status: RandomResultType::Success,
                                    numbers,
                                    attempts,
                                    logs,
                                    clear_err_tracker,
                                    settings: String::new(),
                                    random: Some(random)
                                };
                            },
                            Err(e) => {
                                halt_from_error(
                                    ErrKey::IsMatch(e.1),
                                    &format!("is_match_check failed. {}", e.0),
                                    &mut logs,
                                    &mut err_tracker,
                                    attempts,
                                    &mut numbers,
                                    settings,
                                    &mut clear_err_tracker,
                                );
                                if is_match_attempts - 1 == settings.max_is_match_attempts() {                        
                                    clear_numbers(&mut logs, &mut numbers, settings);
                                    is_match_attempts = 0;
                                    logs.push(Log::Info {
                                        msg: "Clear - Reached Is Match Attempts".to_string(),
                                    });
                                    clear_err_tracker.insert(attempts);
                                }
                            }                    
                        }
                    } else {
                        set_numbers(
                            &mut numbers,
                            settings,
                            current_data_with_potential_numbers.selected_numbers(),
                        );
                    }
                },
                Err(e) => {
                    //an exclude rule can not be made the priority because it does not get numbers
                    if let (IsWithinErrorType::MakePriority, RuleRef::Expected(idx)) = (&e.0, e.2) {
                        rule_to_make_priority = Some(idx);
                    }
                    halt_from_error(
                        ErrKey::IsWithinRange(e.2),
                        &e.1,
                        &mut logs,
                        &mut err_tracker,
                        attempts,
                        &mut numbers,
                        settings,
                        &mut clear_err_tracker,
                    );
                    continue;
                },
            }            
        }
    }

    return RandomResult {
        status: RandomResultType::Failed,
        numbers: Vec::new(),
        attempts: num_attempts,
        logs,
        clear_err_tracker,
        settings: format!("{:#?}", settings),
        random: None
    };
}


//Checks every rule in order and returns the error from the first rule that does not match, with which rule it was
fn is_match_check(
    expected_rules: &[(usize, &dyn RuleTrait)], 
    exclude_rules: &Option<Vec<Box<dyn ExcludeRuleTrait>>>, 
    _settings: &Settings, current_data: &CurrentData,
    logs: &mut Vec<Log>) -> std::result::Result<(), (String, RuleRef)> {
    for (idx, expected_rule) in expected_rules {
        if let Err(e) = expected_rule.is_match(current_data) {
            logs.push(Log::Info {
                msg: format!("Expected Rule - {} - {}", expected_rule.name(), expected_rule),
            });
            return Err((e, RuleRef::Expected(*idx)));
        }
    }
    if let Some(exc_rules) = exclude_rules {
        for (idx, exclude_rule) in exc_rules.iter().enumerate() {
            if let Err(e) = exclude_rule.is_excluded(current_data) {
                logs.push(Log::Info {
                    msg: format!("Exclude Rule - {} - {}", exclude_rule.exclude_name(), exclude_rule),
                });
                return Err((e, RuleRef::Exclude(idx)));
            }
        }
    }

    return Ok(());
}

//Checks every rule in order and returns the error from the first rule that is out of range, with which rule it was
fn is_within_range_check(
    expected_rules: &[(usize, &dyn RuleTrait)], 
    exclude_rules: &Option<Vec<Box<dyn ExcludeRuleTrait>>>, 
    _settings: &Settings, 
    current_data: &CurrentData,
    logs: &mut Vec<Log>) -> std::result::Result<(), (IsWithinErrorType, String, RuleRef)> {
    for (idx, expected_rule) in expected_rules {
        if let Err(e) = expected_rule.is_within_range(current_data) {
            logs.push(Log::Info {
                msg: format!("Expected Rule - {} - {}", expected_rule.name(), expected_rule),
            });
            return Err((e.0, e.1, RuleRef::Expected(*idx)));
        }
    }
    if let Some(exc_rules) = exclude_rules {
        for (idx, exclude_rule) in exc_rules.iter().enumerate() {
            if let Err(e) = exclude_rule.is_within_excluded_range(current_data) {
                logs.push(Log::Info {
                    msg: format!("Exclude Rule - {} - {}", exclude_rule.exclude_name(), exclude_rule),
                });
                return Err((e.0, e.1, RuleRef::Exclude(idx)));
            }
        }
    }

    return Ok(());
}
