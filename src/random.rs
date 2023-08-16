use once_cell::sync::OnceCell;

use crate::random_trait::shuffle_vec;
use crate::rules::{
    IsWithinErrorType, MapAnyValue, RuleTrait,
};
use crate::settings::Settings;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Debug, Formatter, Result};
use std::hash::Hash;

const ERR_TRACKER_MULTIPLIER: usize = 6;

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
    clear_err_tracker: Vec<usize>
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
    pub fn string(&self, shuffle: bool) -> std::result::Result<String, String> {
        match self.status {
            RandomResultType::Success => {
                let mut random_string = String::new();
                let mut numbers: Vec<usize> = self.numbers.clone();
                if shuffle {
                    shuffle_vec(&mut numbers);
                    //numbers.shuffle(&mut thread_rng()); TODO remove
                }
                for digit in numbers {
                    random_string.push(char::from_u32(digit as u32).unwrap());
                }
                return Ok(random_string);
            },
            _ => Err("Error! Please Check Logs!".to_string())
        }
    }
    pub fn attempts(&self) -> usize {
        return self.attempts;
    }
    pub fn logs(&self) -> &Vec<Log> {
        return &self.logs;
    }
    pub fn clear_err_tracker(&self) -> &Vec<usize> {
        return &self.clear_err_tracker;
    }
}

impl fmt::Display for RandomResult {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "\n\nStatus - {:?}\nNumbers - {:?}\nAttempts - {}\nClear Errs - {:?}\n{}",
            self.status(),
            self.numbers(),
            self.attempts,
            self.clear_err_tracker(),
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
    numbers.clear();
    logs.push(Log::Info {
        msg: format!("CLEAR - {:?}", numbers),
    });
}

#[derive(Debug, Clone)]
pub enum Log {
    Error { msg: String },
    Info { msg: String },
}

impl fmt::Display for Log {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Log::Error { msg } => write!(fmt, "Error : {}", msg.to_owned())?,
            Log::Info { msg } => write!(fmt, "Info  : {}", msg.to_owned())?,
        }
        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
pub fn halt_from_error(
    err: &str,
    logs: &mut Vec<Log>,
    err_tracker: &mut HashMap<String, usize>,
    attempts: usize,
    numbers: &mut Vec<usize>,
    settings: &Settings,
    clear_err_tracker: &mut Vec<usize>,
) {
    logs.push(Log::Error {
        msg: err.to_owned(),
    });
    let count = *err_tracker
        .entry(err.to_owned())
        .and_modify(|x| *x += 1)
        .or_insert(1);
    if count > (ERR_TRACKER_MULTIPLIER * settings.count()) {
        logs.push(Log::Error {
            msg: format!("Reset Because Of Too Many Same Errors - {}", err),
        });
        reset(logs, numbers, settings, err_tracker);
        clear_err_tracker.push(attempts);
    }
}

fn reset(
    logs: &mut Vec<Log>,
    numbers: &mut Vec<usize>,
    settings: &Settings,
    err_tracker: &mut HashMap<String, usize>,
) {
    clear_numbers(logs, numbers, settings);
    err_tracker.clear();
}

pub struct CurrentData<'a> {
    selected_numbers: &'a Vec<usize>,
    selected_numbers_set: OnceCell<HashSet<usize>>,
    selected_numbers_sorted: OnceCell<Vec<usize>>,
    settings: &'a Settings,
    shared_data: &'a HashMap<String, HashMap<String, MapAnyValue>>,
}

impl<'a> CurrentData<'a> {

    pub fn new(selected_numbers: &'a Vec<usize>, settings: &'a Settings, shared_data: &'a HashMap<String, HashMap<String, MapAnyValue>>) -> CurrentData<'a> {
        return CurrentData { selected_numbers, settings, shared_data, selected_numbers_set: OnceCell::new(), selected_numbers_sorted: OnceCell::new() };
    }

    pub fn from_current_data(current_data: &'a CurrentData, shared_data: &'a HashMap<String, HashMap<String, MapAnyValue>>) -> CurrentData<'a> {
        let new_current_data = CurrentData { 
            selected_numbers: current_data.selected_numbers, 
            settings: current_data.settings, 
            shared_data, 
            selected_numbers_set: OnceCell::new(), 
            selected_numbers_sorted: OnceCell::new(), 
        };
        if current_data.selected_numbers_set.get().is_some() {
            let _ = new_current_data.selected_numbers_set.set(current_data.selected_numbers_set.get().unwrap().clone());
        }
        if current_data.selected_numbers_sorted.get().is_some() {
            let _ = new_current_data.selected_numbers_sorted.set(current_data.selected_numbers_sorted.get().unwrap().to_vec());
        }
        return new_current_data;
    }

    pub fn selected_numbers(&self) -> &'a Vec<usize> {
        return self.selected_numbers;
    }

    pub fn settings(&self) -> &'a Settings {
        return self.settings;
    }

    pub fn shared_data(&self) -> &'a HashMap<String, HashMap<String, MapAnyValue>> {
        return self.shared_data;
    }

    pub fn selected_numbers_set(&self) -> &HashSet<usize> {
        return self.selected_numbers_set.get_or_init(|| {
            self.selected_numbers.iter().copied().collect()
        });
    }

    pub fn selected_numbers_sorted(&self) -> &Vec<usize> {
        return self.selected_numbers_sorted.get_or_init(|| {
            let mut t = self.selected_numbers.iter().copied().collect::<Vec<usize>>();
            t.sort_unstable();
            return t;
        });
    }  
}

pub fn random_numbers(settings: &Settings) -> RandomResult {
    let mut numbers: Vec<usize> = Vec::new();
    let max_tries = 500;
    let mut err_tracker: HashMap<String, usize> = HashMap::new();
    let mut num_attempts = 1;
    let mut logs: Vec<Log> = Vec::new();
    let mut clear_err_tracker: Vec<usize> = Vec::new();
    let mut expected_rules: Vec<Box<dyn RuleTrait>> = settings.expected_rules().clone();
    let mut key_to_make_priority: Option<String> = None;
    clear_numbers(&mut logs, &mut numbers, settings);
    for attempts in 1..=max_tries {
        logs.push(Log::Info {
            msg: format!("Attempt - {:?}", attempts),
        });
        num_attempts = attempts;
        let mut potential_numbers = Vec::new();
        let mut gen_type = String::from("");

        let current_data_numbers: Vec<usize> = numbers.iter().copied().collect();
        let current_data_shared_data: HashMap<String, HashMap<String, MapAnyValue>> = HashMap::new();
        let current_data_selected_numbers_sd = CurrentData::new(&current_data_numbers, settings, &current_data_shared_data);
        let mut shared_data: HashMap<String, HashMap<String, MapAnyValue>> = HashMap::new();
        shuffle_vec(&mut expected_rules);
        //expected_rules.shuffle(&mut thread_rng()); TODO remove
        if let Some(v) = key_to_make_priority {
            let idx = expected_rules
                .iter()
                .position(|x| x.name() == v)
                .unwrap();
            expected_rules.swap(0, idx);
            key_to_make_priority = None;
        }

        for expected_rule in &expected_rules {
            if let Some(actual_rule_shared_data) =
                expected_rule.share_data(&current_data_selected_numbers_sd)
            {
                shared_data.insert(expected_rule.name(), actual_rule_shared_data);
            }
        }

        let current_data_selected_numbers_gn = CurrentData::from_current_data(&current_data_selected_numbers_sd, &shared_data);
        for expected_rule in &expected_rules {
            match expected_rule.get_numbers(&current_data_selected_numbers_gn) {
                Ok(v) => {
                    gen_type = expected_rule.name();
                    potential_numbers.extend(&v);
                    break;
                }
                Err(e) => {
                    if e != "Skip" {
                        halt_from_error(
                            &e,
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
        }

        //Check if potential_numbers are valid
        if !potential_numbers.is_empty() {
            let selected_and_potential_numbers = numbers
                .iter()
                .copied()
                .chain(potential_numbers.iter().copied())
                .collect::<Vec<usize>>();
            let current_data_with_potential_numbers = CurrentData::new(&selected_and_potential_numbers, settings, &shared_data);

            logs.push(Log::Info {
                msg: format!(
                    "GEN_TYPE - {}; P - {:?}; A&P - {:?}",
                    gen_type, potential_numbers, selected_and_potential_numbers
                ),
            });

            if selected_and_potential_numbers.len() > settings.count() {
                halt_from_error(
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

            let mut is_within_range_err: std::result::Result<(), String> = Ok(());
            for expected_rule in &expected_rules {
                if let Err(e) = expected_rule.is_within_range(
                    &current_data_with_potential_numbers,
                ) {
                    if e.0 == IsWithinErrorType::MakePriority {
                        key_to_make_priority = Some(expected_rule.name());
                    }
                    is_within_range_err = Err(format!("Check Type: {}. Error Type: {:?}. Potential Rule Stack Not Within Range - {}", expected_rule.name(), e.0, e.1));
                }
            }
            
            if let Ok(()) = is_within_range_err {
                if let Some(exclude_rules) = settings.exclude_rules() {
                    for exclude_rule in exclude_rules {
                        if let Err(e) = exclude_rule.is_within_excluded_range(
                            &current_data_with_potential_numbers,
                        ) {
                            if e.0 == IsWithinErrorType::MakePriority {
                                key_to_make_priority = Some(exclude_rule.exclude_name());
                            }
                            is_within_range_err = Err(format!("Check Type: {}. Error Type: {:?}. Potential Rule Stack Not Within Range - {}", exclude_rule.exclude_name(), e.0, e.1));
                        }
                    }
                }
            }

            match is_within_range_err {
                Ok(()) => {
                    set_numbers(
                        &mut numbers,
                        settings,
                        &selected_and_potential_numbers,
                    );
                }
                Err(e) => {
                    halt_from_error(
                        &e,
                        &mut logs,
                        &mut err_tracker,
                        attempts,
                        &mut numbers,
                        settings,
                        &mut clear_err_tracker,
                    );
                }
            }

            if numbers.len() == settings.count() {
                let mut no_errors = true;
                for expected_rule in &expected_rules {
                    if let Err(e) = expected_rule.is_match(
                        &current_data_with_potential_numbers,
                    ) {
                        no_errors = false;
                        halt_from_error(
                            &e,
                            &mut logs,
                            &mut err_tracker,
                            attempts,
                            &mut numbers,
                            settings,
                            &mut clear_err_tracker,
                        );
                        clear_numbers(&mut logs, &mut numbers, settings);
                        break;
                    }
                }
                match settings.exclude_rules() {
                    Some(exclude_rules) => {
                        for exclude_rule in exclude_rules {
                            if let Err(e) = exclude_rule.is_excluded(
                                &current_data_with_potential_numbers,
                            ) {
                                no_errors = false;
                                halt_from_error(
                                    &e,
                                    &mut logs,
                                    &mut err_tracker,
                                    attempts,
                                    &mut numbers,
                                    settings,
                                    &mut clear_err_tracker,
                                );
                                clear_numbers(&mut logs, &mut numbers, settings);
                                break;
                            }
                        }
                    },
                    None => {},
                }
                if no_errors {
                    return RandomResult {
                        status: RandomResultType::Success,
                        numbers,
                        attempts,
                        logs,
                        clear_err_tracker
                    };
                }
            }
        }
    }

    return RandomResult {
        status: RandomResultType::Failed,
        numbers: Vec::new(),
        attempts: num_attempts,
        logs,
        clear_err_tracker
    };
}
