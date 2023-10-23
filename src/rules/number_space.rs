
use crate::random::CurrentData;
use crate::random_trait::get_random_trait;
use crate::rules::{MapAnyValue, RuleTrait, IsWithinErrorType};
use crate::settings::Settings;
use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Result};

use super::{ExcludeRuleTrait, is_excluded_helper};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NumberSpaceType {
    Lt(usize),
    Lte(usize),
    Eq(usize),
    Gte(usize),
    Gt(usize),
    Between(usize, usize)
}

impl NumberSpaceType {
    pub fn is_match(&self, num_space: usize) -> bool {
        return match *self {//TODO check dereference 
            NumberSpaceType::Lt(v) => num_space < v,
            NumberSpaceType::Lte(v) => num_space <= v,
            NumberSpaceType::Eq(v) => num_space == v,
            NumberSpaceType::Gte(v) => num_space >= v,               
            NumberSpaceType::Gt(v) => num_space > v, 
            NumberSpaceType::Between(lower_bound, upper_bound) => num_space >= lower_bound && num_space <= upper_bound 
        };
    }

    pub fn has(&self, num_spaces: &[usize]) -> usize {
        let mut matches: usize = 0;
        for num_space in num_spaces {
            if self.is_match(*num_space) {
                matches += 1;
            }
        }
        return matches;
    }

    pub fn get_number(&self, number_space_base: usize, max: usize) -> usize {
        return match *self {//TODO check dereference 7;3 between(1-3)
            NumberSpaceType::Lt(v) => get_random_trait().get_number(number_space_base + 1, number_space_base + v - 1),     //7+1=8;7+3-1=9 -- 8,9
            NumberSpaceType::Lte(v) => get_random_trait().get_number(number_space_base + 1, number_space_base + v),        //7+1=8;7+3=10  -- 8,9,10
            NumberSpaceType::Eq(v) => number_space_base + v,                                                                        //7+3=10        -- 10
            NumberSpaceType::Gt(v) => {
                if (number_space_base + v + 1) <= max { 
                    get_random_trait().get_number(number_space_base + v + 1, max)
                } else {
                    0
                }
            },                                                                                                                                                                                                                                                 
            NumberSpaceType::Gte(v) => {
                if (number_space_base + v) <= max { 
                    get_random_trait().get_number(number_space_base + v, max)
                } else {
                    0
                }
            },                               
            NumberSpaceType::Between(lower_bound, upper_bound) => get_random_trait().get_number(number_space_base + lower_bound, number_space_base + upper_bound), //7+1=8;7+3=10  -- 8,9,10
        };
    }
}

impl Display for NumberSpaceType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            NumberSpaceType::Gte(v) => write!(f, "Gte:{}", v),
            NumberSpaceType::Eq(v) => write!(f, "Eq:{}", v),
            NumberSpaceType::Lt(v) => write!(f, "Lt:{}", v),
            NumberSpaceType::Lte(v) => write!(f, "Lte:{}", v),
            NumberSpaceType::Gt(v) => write!(f, "Gt:{}", v),
            NumberSpaceType::Between(lower_bound, upper_bound) => write!(f, "Gte:{}-Lte:{}", lower_bound, upper_bound),
        }
    }
}

#[derive(Clone, Debug)]
pub struct NumberSpaceItem {
    number_space_type: NumberSpaceType,
    needs: usize,
    has: usize,
    missing: usize,
}

impl NumberSpaceItem {
    pub fn new(number_space_type: &NumberSpaceType, needs: usize) -> NumberSpaceItem {
        return NumberSpaceItem::new_full(number_space_type, needs, 0, 0);
    }

    pub fn new_full(number_space_type: &NumberSpaceType, needs: usize, has: usize, missing: usize) -> NumberSpaceItem {
        return NumberSpaceItem {
            number_space_type: number_space_type.clone(),
            needs,
            has,
            missing,
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

    pub fn number_space_type(&self) -> &NumberSpaceType {
        return &self.number_space_type;
    }

    pub fn needs(&self) -> usize {
        return self.needs;
    }

    pub fn has(&self) -> usize {
        return self.has;
    }

    pub fn missing(&self) -> usize {
        return self.missing;
    }
}

#[derive(Clone)]
pub struct NumberSpace {
    number_space_items: Vec<NumberSpaceItem>
}

impl NumberSpace {
    pub fn new(number_space_items: &[NumberSpaceItem]) -> NumberSpace {
        return NumberSpace { number_space_items: number_space_items.to_vec() };
    }

    pub fn number_space_items(&self) -> &Vec<NumberSpaceItem> {
        return &self.number_space_items;
    }

    fn from_numbers_2(
        number_space_items: &Vec<NumberSpaceItem>,
        numbers: &[usize],
        is_sorted: bool
    ) -> NumberSpace {
        return NumberSpace::from_numbers(
            &number_space_items
                .iter()
                .map(|x| NumberSpaceItem::new(&x.number_space_type.clone(), x.needs))
                .collect::<Vec<NumberSpaceItem>>(),
                numbers,
            false,
            is_sorted
        );
    }

    pub fn from_numbers(
        number_pool_items: &[NumberSpaceItem],
        numbers: &[usize],
        set_needs_eq_match: bool,
        is_sorted: bool
    ) -> NumberSpace {
        let mut number_space_items: Vec<NumberSpaceItem> = Vec::new();
        for number_pool_item in number_pool_items {            
            let match_count = number_pool_item.number_space_type.has(&NumberSpaceItem::get_num_spaces(numbers, is_sorted));
            let missing = if match_count > number_pool_item.needs {
                0
            } else {
                number_pool_item.needs - match_count
            };
            number_space_items.push(
                NumberSpaceItem {
                    number_space_type: number_pool_item.number_space_type.clone(),
                    needs: if set_needs_eq_match {
                        match_count
                    } else {
                        number_pool_item.needs
                    },
                    has: match_count,
                    missing,
                },
            );
        }
        return NumberSpace {
            number_space_items,
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
                s.push_str(&number_space_item.has.to_string());
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
    ) -> Option<HashMap<String, MapAnyValue>> {
        None
    }

    fn get_numbers(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<Vec<usize>, String> {
        if current_data.selected_numbers().len() > 1 {
        } else {

        }

        let other_number_pool =
        NumberSpace::from_numbers_2(&self.number_space_items, &current_data.selected_numbers_sorted(), true);
        let mut numbers: Vec<usize> = Vec::new();
        for number_space_item in &other_number_pool.number_space_items {
            if number_space_item.missing > 0 {
                let number_space_base: usize = if !numbers.is_empty() {
                    *numbers.last().unwrap()
                } else if current_data.selected_numbers_sorted().len() > 0 {
                    *current_data.selected_numbers_sorted().last().unwrap()
                } else {
                    let (min, max) = Settings::get_min_max("NumberRange", current_data.shared_data());
                    get_random_trait().get_number(min, max)
                };
                numbers.push(number_space_item
                    .number_space_type
                    .get_number(
                        number_space_base,
                        Settings::get_min_max("NumberRange", current_data.shared_data()).1
                    )
                );
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

        let other_number_space = NumberSpace::from_numbers_2(&self.number_space_items, &current_data.selected_numbers_sorted(), true);

        let mut total_missing: usize = 0;
        for number_space_item in &other_number_space.number_space_items {
            if number_space_item.has > number_space_item.needs {
                return Err((IsWithinErrorType::Regular, format!(
                    "Too many from pool {:?}, \"needs\" is {} and \"has\" {} from this pool",
                    number_space_item.number_space_type, number_space_item.needs, number_space_item.has
                )));
            }
            if number_space_item.needs > 0 {
                total_missing += number_space_item.missing;
            }
        }
        let len_remaining = if current_data.selected_numbers().len() > current_data.settings().count() {
            0
        } else {
            current_data.settings().count() - current_data.selected_numbers().len()
        };
        if total_missing > 0 && total_missing > len_remaining {
            return Err((IsWithinErrorType::MakePriority, format!(
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
            NumberSpace::from_numbers_2(&self.number_space_items, current_data.selected_numbers_sorted(), true);
        for number_space_item in other_number_space.number_space_items {
            if number_space_item.has != number_space_item.needs {
                return Err(format!(
                    "Expected--Pool:{:?}--Needs:{}. Actual Count:{}",
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
