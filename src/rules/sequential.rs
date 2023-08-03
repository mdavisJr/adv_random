use crate::random::CurrentData;
use crate::random_trait::get_random_vec_item;
use crate::rules::{MapAnyValue, RuleTrait, IsWithinErrorType};
use crate::settings::Settings;
use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Clone)]
pub struct Sequential {
    not: usize,
    seq_counts: Vec<usize>,
}

impl Sequential {
    pub fn new(not: usize, seq_counts: &[usize]) -> Sequential {
        return Sequential {
            not,
            seq_counts: seq_counts.to_vec(),
        };
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
        };
    }

    pub fn seq_counts(&self) -> &Vec<usize> {
        return &self.seq_counts;
    }

    pub fn not(&self) -> usize {
        return self.not;
    }

    pub fn needs_seq(&self, other: &Self, settings: &Settings) -> usize {
        if other.not == settings.count() {
            return 0;
        }
        let mut other_seq_counts: Vec<usize> = other.seq_counts.clone();
        for seq_count in self.seq_counts() {
            if let Some(idx) = other_seq_counts.iter().position(|x| *x == *seq_count) {
                other_seq_counts.remove(idx);
            }
        }
        if !other_seq_counts.is_empty() {
            return *get_random_vec_item(&other_seq_counts);
        }
        return 0;
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
        write!(f, "{}", self.to_string())
    }
}

impl RuleTrait for Sequential {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn share_data(
        &self,
        _current_data: &CurrentData,
    ) -> Option<HashMap<String, MapAnyValue>> {
        None
    }

    fn get_numbers(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<Vec<usize>, String> {
        let act_seq = Sequential::from_numbers(current_data, false);
        let seq_count_needed = act_seq.needs_seq(self, current_data.settings());
        if seq_count_needed > 0 {
            let range = if current_data.selected_numbers().is_empty() {
                let num = current_data.settings().get_number_within_number_range(current_data).unwrap()[0];
                num..=(num + seq_count_needed - 1)
            } else {
                let num = *get_random_vec_item(&current_data.selected_numbers());
                (num + 1)..=(num + seq_count_needed - 1)
            };
            let mut seq_digits: Vec<usize> = Vec::new();
            for seq_digit in range {
                seq_digits.push(seq_digit);
            }
            return Ok(seq_digits);
        }
        return Err(String::from("Skip"));
    }

    fn is_within_range(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<(), (IsWithinErrorType, String)> {
        let other = Sequential::from_numbers(current_data, false);
        if other.not > self.not {
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
        if this_count <= count {
            return Ok(true);
        }
        return Err(format!("{} count: {} is greater than count: {} ", self.name(), this_count, count));
    }
}
