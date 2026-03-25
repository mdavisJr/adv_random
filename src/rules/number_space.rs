
use crate::random::{CurrentData, NumberType};
use crate::rules::{MapAnyValue, RuleTrait, IsWithinErrorType};
use crate::settings::Settings;
use std::any::Any;
use std::{cmp, fmt};
use std::fmt::{Debug, Display, Formatter, Result};

use super::{ExcludeRuleTrait, is_excluded_helper};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OperatorType {
    Lt(usize),
    Lte(usize),
    Eq(usize),
    Gte(usize),
    Gt(usize),
    Between(usize, usize)
}

impl OperatorType {
    pub fn is_absolute_zero(&self) -> bool {
        return match *self {
            OperatorType::Lt(v) => v == 1,
            OperatorType::Lte(v) => v == 0,
            OperatorType::Eq(v) => v == 0,
            // NumberSpaceType::Gte(v) => *,               
            // NumberSpaceType::Gt(v) => *, 
            OperatorType::Between(lower_bound, upper_bound) => lower_bound == 0 && upper_bound == 0,
            _ => false
        };
    }

    pub fn get_missing_without_has(&self) -> usize {
        return self.get_missing(0);
    }

    pub fn get_missing(&self, value: usize) -> usize {
        return match *self {
            OperatorType::Lt(_v) => 0,
            OperatorType::Lte(_v) => 0,
            OperatorType::Eq(v) => v.saturating_sub(value),
            OperatorType::Gte(v) => v.saturating_sub(value),               
            OperatorType::Gt(v) => (v+1).saturating_sub(value), 
            OperatorType::Between(lower_bound, _upper_bound) => lower_bound.saturating_sub(value),
        };
    }

    pub fn is_within_range(&self, value: usize) -> bool {
        return match *self {
            OperatorType::Lt(v) => value < v,
            OperatorType::Lte(v) => value <= v,
            OperatorType::Eq(v) => value <= v,
            OperatorType::Gte(_v) => true,               
            OperatorType::Gt(_v) => true, 
            OperatorType::Between(_lower_bound, upper_bound) => value <= upper_bound 
        };
    }

    pub fn is_match(&self, value: usize) -> bool {
        return match *self {
            OperatorType::Lt(v) => value < v,
            OperatorType::Lte(v) => value <= v,
            OperatorType::Eq(v) => value == v,
            OperatorType::Gte(v) => value >= v,               
            OperatorType::Gt(v) => value > v, 
            OperatorType::Between(lower_bound, upper_bound) => value >= lower_bound && value <= upper_bound 
        };
    }

    #[allow(unused_assignments)]
    pub fn has(&self, num_spaces: &[usize]) -> Vec<usize> {
        let mut matches: Vec<usize> = Vec::new();
        let mut match_count = 0;
        let mut last_gap_was_match = false;
        for num_space in num_spaces {
            if self.is_match(*num_space) {
                if last_gap_was_match {
                    match_count += 1;
                } else {
                    match_count = 1;
                }
                last_gap_was_match = true;
            } else {
                if match_count > 0 {
                    matches.push(match_count);
                    match_count = 0;
                }
                last_gap_was_match = false;
            }            
        }
        if match_count > 0 {
            matches.push(match_count);
            match_count = 0;
        }

        if !matches.is_empty() {
            matches.sort();           
        }
        
        return matches; 
    }

    pub fn get_number(&self, current_data: &CurrentData, count: usize) -> Vec<usize> {
        let mut numbers: Vec<usize> = Vec::new();
        for _ in 0..5 {
            numbers = Vec::new();    
            let (min, max) = Settings::get_min_max(current_data.shared_data());
            numbers.push(
                if current_data.selected_numbers_sorted().is_empty() {            
                    current_data.get_rand(NumberType::Any, None, None)
                } else {
                    *current_data.selected_numbers_sorted().last().unwrap()
                }
            );
            for _ in 0..count*2 {
                let base_number = *numbers.last().unwrap();
                let (lower_b, upper_b) = match *self {//TODO check dereference 7;3 between(1-3)
                    OperatorType::Lt(v) => (Some(base_number + 1), Some(cmp::min(base_number + v - 1, max))),     //7+1=8;7+3-1=9 -- 8,9
                    OperatorType::Lte(v) => (Some(base_number + 1), Some(cmp::min(base_number + v, max))),        //7+1=8;7+3=10  -- 8,9,10
                    OperatorType::Eq(v) => (Some(base_number + v), None),                                                                        //7+3=10        -- 10
                    OperatorType::Gt(v) => (Some(base_number + v + 1), Some(max)),                                                                                                                                                                                                                                       
                    OperatorType::Gte(v) => (Some(base_number + v), Some(max)),                         
                    OperatorType::Between(lower_bound, upper_bound) => (Some(base_number + lower_bound), Some(base_number + upper_bound)),
                };

                let new_num: Option<usize> = match (lower_b, upper_b) {
                    (None, None) => None,
                    (None, Some(_)) => None,
                    (Some(v), None) => Some(v),
                    (Some(lb), Some(ub)) => {
                        if lb >= ub || lb < min || ub > max {
                            None
                        } else {
                            Some(current_data.get_rand(NumberType::Any, None, Some((lb, ub))))
                        }
                    }
                };

                if let Some(v) = new_num {
                    numbers.push(v);
                }

                if numbers.len() == count {
                    break;
                }
            }

            if numbers.len() == count {
                break;
            }
        }

        return numbers;
    }
}

impl Display for OperatorType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            OperatorType::Gte(v) => write!(f, "Gte:{}", v),
            OperatorType::Eq(v) => write!(f, "Eq:{}", v),
            OperatorType::Lt(v) => write!(f, "Lt:{}", v),
            OperatorType::Lte(v) => write!(f, "Lte:{}", v),
            OperatorType::Gt(v) => write!(f, "Gt:{}", v),
            OperatorType::Between(lower_bound, upper_bound) => write!(f, "Gte:{}-Lte:{}", lower_bound, upper_bound),
        }
    }
}

#[derive(Clone, Debug)]
pub struct NumberSpaceItem {
    number_space_type: OperatorType,
    needs: Vec<usize>,
    has: Vec<usize>
}

impl NumberSpaceItem {
    pub fn new(number_space_type: &OperatorType, needs: usize) -> NumberSpaceItem {
        return NumberSpaceItem::new_full(number_space_type, &[needs], &Vec::new());
    }

    pub fn new_full(number_space_type: &OperatorType, needs: &[usize], has: &[usize]) -> NumberSpaceItem {
        return NumberSpaceItem {
            number_space_type: *number_space_type,
            needs: needs.to_vec(),
            has: has.to_vec()
        };
    }

    pub fn get_num_spaces(list: &[usize], is_sorted: bool) -> Vec<usize> {    
        if list.len() > 1 {
            let mut num_spaces:Vec<usize> = Vec::new();
            let sorted_list: Vec<usize> = if is_sorted { 
                list.to_vec()
            } else { 
                let mut v = list.to_vec(); 
                v.sort(); 
                v
            };
            for x in 1..sorted_list.len() {
                num_spaces.push(sorted_list[x] - sorted_list[x - 1]);
            }
            return num_spaces;
        }
        return Vec::new();
    }

    pub fn from_numbers(
        number_space_types: &[(OperatorType, Option<Vec<usize>>)],
        numbers: &[usize],
        is_sorted: bool
    ) -> Vec<NumberSpaceItem> {
        return NumberSpaceItem::from_number_spaces(number_space_types, &NumberSpaceItem::get_num_spaces(numbers, is_sorted));
    }

    pub fn from_number_spaces(
        number_space_types: &[(OperatorType, Option<Vec<usize>>)],
        number_spaces: &[usize]
    ) -> Vec<NumberSpaceItem> {
        let mut number_space_items: Vec<NumberSpaceItem> = Vec::new();
        for number_space_type in number_space_types {            
            let has_count = number_space_type.0.has(number_spaces);
            number_space_items.push(
                NumberSpaceItem {
                    number_space_type: number_space_type.0,
                    needs: match &number_space_type.1 {
                        Some(v) => v.clone(),
                        None => has_count.clone()
                    },
                    has: has_count.clone()
                },
            );
        }
        
        return number_space_items;
    }

    pub fn number_space_type(&self) -> &OperatorType {
        return &self.number_space_type;
    }

    pub fn needs(&self) -> &Vec<usize> {
        return &self.needs;
    }

    pub fn has(&self) -> &Vec<usize> {
        return &self.has;
    }

    pub fn missing(&self) -> Vec<usize> {
        let mut missing_list: Vec<usize> = Vec::new();
        for i in 0..self.needs.len() {
            if i < self.has.len() {
                let missing = self.needs[i].saturating_sub(self.has[i]);
                missing_list.push(missing);
            } else {
                missing_list.push(self.needs[i]);
            }
        }
        
        return missing_list;
    }

    pub fn process_missing_count(&self) -> usize {
        let missing_list = self.missing();
        for missing in missing_list {
            if missing > 0 {
                return missing;
            }
        }

        return 0;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ProcessNumberSpaceItems {
    AllPerRequest,
    OnePerRequest
}

#[derive(Clone)]
pub struct NumberSpace {
    number_space_items: Vec<NumberSpaceItem>,
    process_number_space_items: ProcessNumberSpaceItems,
}

impl NumberSpace {
    pub fn new(number_space_items: &[NumberSpaceItem]) -> NumberSpace {
        return NumberSpace::new_with_process_number_space_items(number_space_items, ProcessNumberSpaceItems::AllPerRequest);
    }

    pub fn new_with_process_number_space_items(
        number_space_items: &[NumberSpaceItem], 
        process_number_space_items: ProcessNumberSpaceItems
    ) -> NumberSpace {
        return NumberSpace { 
            number_space_items: number_space_items.to_vec(),
            process_number_space_items
        };
    }

    pub fn number_space_items(&self) -> &Vec<NumberSpaceItem> {
        return &self.number_space_items;
    }

    pub fn from_numbers(
        number_space: &NumberSpace,
        numbers: &[usize],
        is_sorted: bool
    ) -> NumberSpace {
        return NumberSpace {
            number_space_items: NumberSpaceItem::from_numbers(&number_space.number_space_items.iter().map(|x| (x.number_space_type, Some(x.needs.clone()))).collect::<Vec<_>>(), numbers, is_sorted),
            process_number_space_items: number_space.process_number_space_items
        };
    }
}

impl Display for NumberSpace {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut s = String::from("");
        if !self.number_space_items.is_empty() {
            for number_space_item in &self.number_space_items {
                s.push_str(&format!("{}", number_space_item.number_space_type));
                s.push('=');
                if !number_space_item.has.is_empty() {
                    for item in &number_space_item.has {
                        s.push_str(item.to_string().as_str());
                        s.push(',');
                    }
                    s.pop();
                }
                s.push(';');
            }
            s.pop();
        }
        write!(f,"{}",s)
    }
}

impl Debug for NumberSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl RuleTrait for NumberSpace {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn share_data(
        &self,
        _current_data: &CurrentData,
    ) -> Option<Vec<(String, MapAnyValue)>> {
        None
    }

    fn get_numbers(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<Vec<usize>, String> {
        let other_number_pool =
        NumberSpace::from_numbers(self, current_data.selected_numbers_sorted(), true);
        let mut numbers: Vec<usize> = Vec::new();
        for number_space_item in &other_number_pool.number_space_items {
            if number_space_item.process_missing_count() > 0 {
                numbers.extend(number_space_item.number_space_type.get_number(current_data, number_space_item.process_missing_count()));                    
                if self.process_number_space_items == ProcessNumberSpaceItems::OnePerRequest && !numbers.is_empty() { break; }
            }
        }
        if !numbers.is_empty() {
            return Ok(numbers);
        }
        
        return Err(String::from("Skip"));
    }

    fn is_within_range(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<(), (IsWithinErrorType, String)> {

        let other_number_space = NumberSpace::from_numbers(self, current_data.selected_numbers_sorted(), true);

        let mut total_missing: usize = 0;
        for number_space_item in &other_number_space.number_space_items {
            if number_space_item.has > number_space_item.needs {
                return Err((IsWithinErrorType::Regular, format!(
                    "Too many from pool {:?}, \"needs\" is {:?} and \"has\" {:?} from this pool",
                    number_space_item.number_space_type, number_space_item.needs, number_space_item.has
                )));
            }
            if number_space_item.needs.iter().sum::<usize>() > 0 {
                total_missing += number_space_item.missing().iter().sum::<usize>();
            }
        }
        let len_remaining = if current_data.selected_numbers().len() > current_data.settings().count() {
            0
        } else {
            current_data.settings().count() - current_data.selected_numbers().len()
        };
        if total_missing > 0 && total_missing > len_remaining {
            return Err((IsWithinErrorType::Regular, format!(
                "Need to pull from number pool, \"missing\" {} and there are {} numbers left to pick",
                total_missing, len_remaining
            )));
        }
        return Ok(());
    }

    fn is_match(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<(), String> {
        let other_number_space =
        NumberSpace::from_numbers(self, current_data.selected_numbers_sorted(), true);
        for number_space_item in other_number_space.number_space_items {
            if number_space_item.has != number_space_item.needs {
                return Err(format!(
                    "Expected--Pool:{:?}--Needs:{:?}. Actual Count:{:?}",
                    number_space_item.number_space_type, number_space_item.needs, number_space_item.has
                ));
            }
        }
        return Ok(());
    }

    fn name(&self) -> String {
        return String::from("NumberSpace");
    }

    fn check_count(
        &self,
        _count: usize,
    ) -> std::result::Result<bool, String> {
        return Ok(true);
    }
}

impl ExcludeRuleTrait for NumberSpace {
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
}
