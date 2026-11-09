use crate::random::{CurrentData, NumberType, NO_RANDOM_ERR};
use crate::random_trait::get_random_vec_item;
use crate::rules::{SharedData, RuleTrait, IsWithinErrorType};
use std::any::Any;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Result};

use super::ExcludeRuleTrait;
use super::exclude_rule_trait::is_excluded_helper;

#[derive(Clone, Default)]
pub struct Sequential {
    not: usize,
    seq_counts: Vec<usize>,
    is_singles_set: bool,
    errors: Vec<String>,
}

impl Sequential {
    //No numbers yet.  Say what is needed with singles and in_a_row.
    pub fn new() -> Sequential {
        return Sequential::default();
    }

    //How many numbers are not next to another number
    pub fn singles(mut self, count: usize) -> Sequential {
        if self.is_singles_set {
            self.errors.push(format!("{} singles was set more than once", self.name()));
        }
        self.is_singles_set = true;
        self.not = count;
        return self;
    }

    //Adds one sequence of this many numbers in a row.  4, 5, 6 is 3 in a row.
    pub fn in_a_row(mut self, count: usize) -> Sequential {
        if count < 2 {
            self.errors.push(format!("{} in_a_row: {} must be 2 or more", self.name(), count));
        }
        //sorted because the counts from selected numbers are sorted before they are compared
        self.seq_counts.push(count);
        self.seq_counts.sort_unstable();
        return self;
    }

    //Adds a sequence for every count.  The same as calling in_a_row for each one.
    pub fn in_a_row_each(mut self, counts: &[usize]) -> Sequential {
        for count in counts {
            self = self.in_a_row(*count);
        }
        return self;
    }

    pub fn from_numbers(current_data: &CurrentData, seq_order_matters: bool) -> Sequential {
        let mut seq_counts: Vec<usize> = Vec::new();
        let mut seq_count: usize = 0;

        for (idx, _) in current_data.selected_numbers_sorted().iter().enumerate() {
            if idx > 0 {
                if (current_data.selected_numbers_sorted()[idx - 1] + 1) == current_data.selected_numbers_sorted()[idx] {
                    if seq_count == 0 {
                        seq_count += 2;
                    } else {
                        seq_count += 1;
                    }
                } else if seq_count > 0 {
                    seq_counts.push(seq_count);
                    seq_count = 0;
                }
            }
        }

        if seq_count > 0 {
            seq_counts.push(seq_count);
        }
        if !seq_order_matters {
            seq_counts.sort_unstable();
        }
        return Sequential {
            not: current_data.selected_numbers().len() - seq_counts.iter().copied().sum::<usize>(),
            seq_counts,
            is_singles_set: false,
            errors: Vec::new(),
        };
    }

    pub fn seq_counts(&self) -> &Vec<usize> {
        return &self.seq_counts;
    }

    pub fn not(&self) -> usize {
        return self.not;
    }

    pub fn needs_seq(&self, other: &Self, current_data: &CurrentData) -> usize {
        if other.not == current_data.settings().count() {
            return 0;
        }
        let mut other_seq_counts: Vec<usize> = other.seq_counts.clone();
        for seq_count in self.seq_counts() {
            if let Some(idx) = other_seq_counts.iter().position(|x| *x == *seq_count) {
                other_seq_counts.remove(idx);
            }
        }
        return match current_data.random() {
            Some(random) => get_random_vec_item(random, &other_seq_counts).copied().unwrap_or(0),
            None => other_seq_counts.first().copied().unwrap_or(0)
        };
    }
}

impl Display for Sequential {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut s = String::from("");
        if !self.seq_counts.is_empty() {
            s.push(',');
            for (idx, seq_count) in self.seq_counts.iter().enumerate() {
                s.push_str("SEQ");
                s.push_str(&(idx + 1).to_string());
                s.push(':');
                s.push_str(&seq_count.to_string());
                s.push(',');
            }
            s.pop();
        }
        write!(f, "{}", "NOT:".to_owned() + &self.not.to_string() + &s)
    }
}

impl Debug for Sequential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl RuleTrait for Sequential {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn share_data(
        &self,
        _current_data: &CurrentData,
    ) -> Option<SharedData> {
        None
    }

    fn get_numbers(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<Option<Vec<usize>>, String> {
        let act_seq = Sequential::from_numbers(current_data, false);
        let seq_count_needed = act_seq.needs_seq(self, current_data);
        if seq_count_needed > 0 {
            let random = current_data.random().ok_or_else(|| NO_RANDOM_ERR.to_owned())?;
            //A selected number that is not part of a sequence can be extended into one.  None starts a new sequence.
            let mut starts: Vec<Option<usize>> = current_data.selected_numbers_sorted().iter().copied()
                .filter(|x| {
                    let has_lower = x.checked_sub(1).is_some_and(|v| current_data.is_selected(v));
                    let has_higher = x.checked_add(1).is_some_and(|v| current_data.is_selected(v));
                    !has_lower && !has_higher
                })
                .map(Some)
                .collect();
            starts.push(None);
            let (first, last) = match get_random_vec_item(random, &starts).copied().flatten() {
                Some(num) => (num.checked_add(1), num.checked_add(seq_count_needed - 1)),
                None => {
                    //leave room for the whole sequence before max
                    let (min, max) = current_data.shared_data().min_max();
                    let max_start = max.checked_sub(seq_count_needed - 1).filter(|v| *v >= min)
                        .ok_or_else(|| format!("Sequence of {} numbers does not fit between min: {} and max: {}", seq_count_needed, min, max))?;
                    let num = current_data.get_rand(NumberType::Any, None, Some((min, max_start))).ok_or_else(|| current_data.no_number_err())?;
                    (Some(num), num.checked_add(seq_count_needed - 1))
                }
            };
            return match (first, last) {
                (Some(first), Some(last)) => Ok(Some((first..=last).collect())),
                _ => Err(format!("Sequence of {} numbers does not fit after the selected number", seq_count_needed))
            };
        }
        return Ok(None);
    }

    fn is_within_range(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<(), (IsWithinErrorType, String)> {
        let other = Sequential::from_numbers(current_data, false);
        if (other.not > self.not) || (other.seq_counts.len() > self.seq_counts.len()) {
            return Err((IsWithinErrorType::Regular, format!(
                "Expected Not: {} and Seq_Counts: {:?}.  Actual Not: {} and Seq_Counts: {:?}.",
                self.not, self.seq_counts, other.not, other.seq_counts
            )));
        }
        for (idx, (a, b)) in self
            .seq_counts
            .iter()
            .zip(other.seq_counts.iter())
            .enumerate()
        {
            if a > b {
                return Err((IsWithinErrorType::Regular, format!(
                    "Expected Not: {} and Seq_Counts: {:?}.  Actual Not: {} and Seq_Counts: {:?}. IDX:{}",
                    self.not, self.seq_counts, other.not, other.seq_counts, idx
                )));
            }
        }
        //a sequence never gets shorter, so the longest sequences have to fit in the longest expected sequences
        if self.seq_counts.iter().rev().zip(other.seq_counts.iter().rev()).any(|(a, b)| b > a) {
            return Err((IsWithinErrorType::Regular, format!(
                "Sequence is too long. Expected Not: {} and Seq_Counts: {:?}.  Actual Not: {} and Seq_Counts: {:?}.",
                self.not, self.seq_counts, other.not, other.seq_counts
            )));
        }
        return Ok(());
    }

    fn is_match(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<(), String> {
        let other = Sequential::from_numbers(current_data, false);
        if self.not == other.not && self.seq_counts == other.seq_counts {
            return Ok(());
        }
        return Err(format!(
            "Expected Not: {} and Seq_Counts: {:?}.  Actual Not: {} and Seq_Counts: {:?}.",
            self.not, self.seq_counts, other.not, other.seq_counts
        ));
    }

    fn name(&self) -> String {
        return String::from("Sequential");
    }

    fn check_count(
        &self,
        count: usize,
    ) -> std::result::Result<bool, String> {
        let this_count:usize = self.not + self.seq_counts.iter().sum::<usize>();
        if this_count == count {
            return Ok(true);
        }
        return Err(format!("{} count: {} must equal count: {} ", self.name(), this_count, count));
    }

    fn validate(&self, count: usize) -> Vec<String> {
        let mut errors = self.errors.clone();
        if let Err(e) = self.check_count(count) {
            errors.push(e);
        }
        return errors;
    }
}

impl ExcludeRuleTrait for Sequential {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_excluded(
        &self,
        current_data: &CurrentData,
    ) -> std::result::Result<(), String> {
        return is_excluded_helper(&self.is_match(current_data), &self.to_string());
    }

    fn is_within_excluded_range(
        &self,
        _current_data: &CurrentData,
    ) -> std::result::Result<(), (IsWithinErrorType, String)> {
        return Ok(());
    }

    fn exclude_name(&self) -> String {
        return self.name();
    }

    fn validate_exclude(&self) -> Vec<String> {
        return self.errors.clone();
    }
}
