
use crate::random::{CurrentData, NumberType};
use crate::random_trait::get_random_vec_item;
use crate::rules::{SharedData, RuleTrait, IsWithinErrorType};
use std::any::Any;
use std::{cmp, fmt};
use std::fmt::{Debug, Display, Formatter, Result};

use super::{ExcludeRuleTrait, is_excluded_helper};

//More types can be added, so a match on this needs a _ arm
#[non_exhaustive]
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
    //Returns an error when nothing can ever match this
    pub fn check(&self) -> std::result::Result<(), String> {
        return match *self {
            OperatorType::Lt(0) => Err(String::from("Lt(0) never matches because nothing is less than 0")),
            OperatorType::Between(lower_bound, upper_bound) if lower_bound > upper_bound => {
                Err(format!("Between lower: {} is greater than upper: {}", lower_bound, upper_bound))
            },
            _ => Ok(())
        };
    }

    pub fn is_absolute_zero(&self) -> bool {
        return match *self {
            OperatorType::Lt(v) => v == 1,
            OperatorType::Lte(v) => v == 0,
            OperatorType::Eq(v) => v == 0,
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

    //Returns true when the space between any two different numbers matches
    pub fn is_match_for_all_different_numbers(&self) -> bool {
        return match *self {
            OperatorType::Gte(v) => v <= 1,
            OperatorType::Gt(v) => v == 0,
            OperatorType::Between(lower_bound, upper_bound) => lower_bound <= 1 && upper_bound == usize::MAX,
            _ => false
        };
    }

    //Returns how many of the sorted numbers are not next to a space that matches, and if the highest number is one of them
    pub fn lone_numbers(&self, sorted_numbers: &[usize]) -> (usize, bool) {
        let mut lone_count = 0;
        let mut highest_is_lone = false;
        for i in 0..sorted_numbers.len() {
            let lower_matches = i > 0 && self.is_match(sorted_numbers[i] - sorted_numbers[i - 1]);
            let higher_matches = i + 1 < sorted_numbers.len() && self.is_match(sorted_numbers[i + 1] - sorted_numbers[i]);
            if !lower_matches && !higher_matches {
                lone_count += 1;
                highest_is_lone = i + 1 == sorted_numbers.len();
            }
        }
        return (lone_count, highest_is_lone);
    }

    //Returns the new numbers needed to add "count" spaces after the highest selected number
    pub fn get_number(&self, current_data: &CurrentData, count: usize) -> Vec<usize> {
        return self.get_run(current_data, count, false);
    }

    //Returns the numbers for a new run of "count" spaces that starts at a random number
    pub fn get_new_run(&self, current_data: &CurrentData, count: usize) -> Vec<usize> {
        return self.get_run(current_data, count, true);
    }

    fn get_run(&self, current_data: &CurrentData, count: usize, new_run: bool) -> Vec<usize> {
        let (min, max) = current_data.shared_data().min_max();
        let highest_selected_number = if new_run { None } else { current_data.selected_numbers_sorted().last() };
        let mut numbers: Vec<usize> = Vec::new();
        for _ in 0..5 {
            numbers = Vec::new();
            let (mut base_number, needed) = match highest_selected_number {
                Some(v) => (*v, count),
                None => {
                    //the starting number is new too
                    let start = match self.get_start_number(current_data, count, min, max) {
                        Some(v) => v,
                        None => continue
                    };
                    numbers.push(start);
                    (start, count + 1)
                }
            };
            while numbers.len() < needed {
                match self.get_next_number(current_data, base_number, min, max) {
                    Some(v) => {
                        numbers.push(v);
                        base_number = v;
                    }
                    None => break
                }
            }

            if numbers.len() == needed {
                break;
            }
        }

        return numbers;
    }

    //Returns the smallest space that the numbers this creates are apart
    fn smallest_space(&self) -> usize {
        return match *self {
            OperatorType::Lt(_) | OperatorType::Lte(_) => 1,
            OperatorType::Eq(v) | OperatorType::Gte(v) => v,
            OperatorType::Gt(v) => v.saturating_add(1),
            OperatorType::Between(lower_bound, _) => lower_bound,
        };
    }

    //Returns the largest space that does not match Gte or Gt.  Any larger space matches them, so a number that is not
    //part of a run has to be this close to another number.  None for the other types, and when every space matches.
    fn largest_separator(&self) -> Option<usize> {
        return match *self {
            OperatorType::Gte(v) => v.checked_sub(1),
            OperatorType::Gt(v) => Some(v),
            _ => None
        }.filter(|v| *v >= 1);
    }

    //Returns the numbers for a new run of "count" spaces just above the highest or just below the lowest selected number,
    //with a space between them that does not match.  None when this type does not need it or there is no room.
    fn get_separated_run(&self, current_data: &CurrentData, count: usize) -> Option<Vec<usize>> {
        let largest_separator = self.largest_separator()?;
        let random = current_data.random()?;
        let lowest = *current_data.selected_numbers_sorted().first()?;
        let highest = *current_data.selected_numbers_sorted().last()?;
        let (min, max) = current_data.shared_data().min_max();
        let room = self.smallest_space().checked_mul(count)?;
        //how far from the highest and lowest numbers the run can start and still fit
        let above = max.checked_sub(room).and_then(|v| v.checked_sub(highest)).map(|v| cmp::min(v, largest_separator)).filter(|v| *v >= 1);
        let below = lowest.checked_sub(min).and_then(|v| v.checked_sub(room)).map(|v| cmp::min(v, largest_separator)).filter(|v| *v >= 1);
        let go_above = match (above, below) {
            (Some(_), Some(_)) => random.get_bool(),
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (None, None) => return None
        };
        let mut base_number = if go_above {
            highest + random.get_number(1, above?)
        } else {
            lowest - random.get_number(1, below?)
        };
        let mut numbers: Vec<usize> = vec![base_number];
        for _ in 0..count {
            base_number = if go_above {
                self.get_next_number(current_data, base_number, min, max)?
            } else {
                self.get_previous_number(current_data, base_number, min)?
            };
            numbers.push(base_number);
        }
        return Some(numbers);
    }

    //Returns a number just above or just below a selected number, with a space between them that does not match.
    //None when this type does not need it.
    fn get_separated_number(&self, current_data: &CurrentData) -> Option<usize> {
        let largest_separator = self.largest_separator()?;
        let random = current_data.random()?;
        let (min, max) = current_data.shared_data().min_max();
        let selected_number = *get_random_vec_item(random, current_data.selected_numbers())?;
        let separator = random.get_number(1, largest_separator);
        let number = if random.get_bool() { selected_number.checked_add(separator)? } else { selected_number.checked_sub(separator)? };
        return Some(number).filter(|v| *v >= min && *v <= max);
    }

    //Gte and Gt only.  The other types are created going up from the lowest number.
    fn get_previous_number(&self, current_data: &CurrentData, base_number: usize, min: usize) -> Option<usize> {
        let ub = match *self {
            OperatorType::Gte(v) => base_number.checked_sub(v)?,
            OperatorType::Gt(v) => base_number.checked_sub(v)?.checked_sub(1)?,
            _ => return None
        };
        if min > ub {
            return None;
        }
        if min == ub {
            return Some(ub);
        }
        return current_data.get_rand(NumberType::Any, None, Some((min, ub)));
    }

    //Returns a random number that leaves room for "count" spaces before max, when there is room
    fn get_start_number(&self, current_data: &CurrentData, count: usize, min: usize, max: usize) -> Option<usize> {
        let highest_start = self.smallest_space().checked_mul(count).and_then(|v| max.checked_sub(v)).filter(|v| *v >= min);
        return match highest_start {
            Some(v) => current_data.get_rand(NumberType::Any, None, Some((min, v))),
            None => current_data.get_rand(NumberType::Any, None, None)
        };
    }

    fn get_next_number(&self, current_data: &CurrentData, base_number: usize, min: usize, max: usize) -> Option<usize> {
        let (lower_b, upper_b) = match *self {                                                             //base 7, v 3
            OperatorType::Lt(v) => (base_number.checked_add(1), base_number.checked_add(v)?.checked_sub(1)),       //8,9
            OperatorType::Lte(v) => (base_number.checked_add(1), base_number.checked_add(v)),                      //8,9,10
            OperatorType::Eq(v) => (base_number.checked_add(v), base_number.checked_add(v)),                       //10
            OperatorType::Gt(v) => (base_number.checked_add(v)?.checked_add(1), Some(max)),                        //11..=max
            OperatorType::Gte(v) => (base_number.checked_add(v), Some(max)),                                       //10..=max
            OperatorType::Between(lower_bound, upper_bound) => (base_number.checked_add(lower_bound), Some(base_number.saturating_add(upper_bound))),
        };
        let lb = cmp::max(lower_b?, min);
        let ub = cmp::min(upper_b?, max);
        if lb > ub {
            return None;
        }
        if lb == ub {
            return Some(lb);
        }
        return current_data.get_rand(NumberType::Any, None, Some((lb, ub)));
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
    //No runs yet.  Say what is needed with needs, and what a set of numbers has with has.
    pub fn new(number_space_type: OperatorType) -> NumberSpaceItem {
        return NumberSpaceItem {
            number_space_type,
            needs: Vec::new(),
            has: Vec::new()
        };
    }

    //The runs that are needed.  Each count is how many spaces in a row match the type.
    pub fn needs(mut self, counts: &[usize]) -> NumberSpaceItem {
        //sorted because the counts from selected numbers are sorted before they are compared
        self.needs = counts.to_vec();
        self.needs.sort_unstable();
        return self;
    }

    //The runs that a set of numbers has
    pub fn has(mut self, counts: &[usize]) -> NumberSpaceItem {
        self.has = counts.to_vec();
        return self;
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

    //The runs that are needed
    pub fn needed(&self) -> &Vec<usize> {
        return &self.needs;
    }

    //The runs that the numbers have
    pub fn found(&self) -> &Vec<usize> {
        return &self.has;
    }

    pub fn missing(&self) -> Vec<usize> {
        //needs and has are both sorted, so the longest of each are paired with each other
        let mut missing_list: Vec<usize> = Vec::new();
        for i in 0..self.needs.len() {
            let from_end = self.needs.len() - i;
            if from_end <= self.has.len() {
                let missing = self.needs[i].saturating_sub(self.has[self.has.len() - from_end]);
                missing_list.push(missing);
            } else {
                missing_list.push(self.needs[i]);
            }
        }
        
        return missing_list;
    }

    pub fn has_too_many(&self) -> bool {
        let mut needs_desc = self.needs.clone();
        needs_desc.sort_unstable_by(|a, b| b.cmp(a));
        //has is sorted ascending, so compare largest to largest
        return self.has.len() > needs_desc.len() || self.has.iter().rev().zip(needs_desc.iter()).any(|(has, needs)| has > needs);
    }

    //Returns how many numbers the runs in needs take.  A run of 3 spaces takes 4 numbers.
    pub fn numbers_needed(&self) -> usize {
        return self.needs.iter().filter(|x| **x > 0).map(|x| x.saturating_add(1)).fold(0, usize::saturating_add);
    }

    pub fn process_missing_count(&self) -> usize {
        return self.process_missing().map(|x| x.0).unwrap_or(0);
    }

    //Returns how many spaces the longest missing run needs, and true when none of that run is there yet
    pub fn process_missing(&self) -> Option<(usize, bool)> {
        //missing is in the same order as needs, and only the last has.len() of them are paired with a run that is there
        let new_runs = self.needs.len().saturating_sub(self.has.len());
        for (i, missing) in self.missing().into_iter().enumerate().rev() {
            if missing > 0 {
                return Some((missing, i < new_runs));
            }
        }

        return None;
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
    errors: Vec<String>,
}

impl Default for NumberSpace {
    fn default() -> NumberSpace {
        return NumberSpace::new();
    }
}

impl NumberSpace {
    //No spaces are needed yet.  Say what is needed with run.
    pub fn new() -> NumberSpace {
        return NumberSpace {
            number_space_items: Vec::new(),
            process_number_space_items: ProcessNumberSpaceItems::AllPerRequest,
            errors: Vec::new(),
        };
    }

    //Adds one run of this many spaces in a row that match the type.  10 numbers have 9 spaces between them, so
    //run(OperatorType::Lt(3), 9) is 10 numbers that are all less than 3 apart.
    pub fn run(mut self, number_space_type: OperatorType, count: usize) -> NumberSpace {
        if count == 0 {
            self.errors.push(format!("{} run of {:?}: 0 must be 1 or more", self.name(), number_space_type));
        }
        if let Err(e) = number_space_type.check() {
            self.errors.push(format!("{} {}", self.name(), e));
        }
        //the runs of one type are checked together, so they are kept in one item
        match self.number_space_items.iter_mut().find(|x| x.number_space_type == number_space_type) {
            Some(number_space_item) => {
                number_space_item.needs.push(count);
                number_space_item.needs.sort_unstable();
            },
            None => self.number_space_items.push(NumberSpaceItem::new(number_space_type).needs(&[count]))
        }
        return self;
    }

    //Adds a run for every count.  The same as calling run for each one.
    pub fn runs(mut self, number_space_type: OperatorType, counts: &[usize]) -> NumberSpace {
        for count in counts {
            self = self.run(number_space_type, *count);
        }
        return self;
    }

    //Adds an item that is already created, such as one that says what a set of numbers has
    pub fn item(mut self, number_space_item: NumberSpaceItem) -> NumberSpace {
        if let Err(e) = number_space_item.number_space_type.check() {
            self.errors.push(format!("{} {}", self.name(), e));
        }
        self.number_space_items.push(number_space_item);
        return self;
    }

    //Adds every item.  The same as calling item for each one.
    pub fn items(mut self, number_space_items: impl IntoIterator<Item = NumberSpaceItem>) -> NumberSpace {
        for number_space_item in number_space_items {
            self = self.item(number_space_item);
        }
        return self;
    }

    //Says if the runs of every type are created in the same request, or one type per request
    pub fn process(mut self, process_number_space_items: ProcessNumberSpaceItems) -> NumberSpace {
        self.process_number_space_items = process_number_space_items;
        return self;
    }

    #[deprecated(since = "3.0.0", note = "use NumberSpace::new().item(...).process(...)")]
    pub fn new_with_process_number_space_items(
        number_space_items: &[NumberSpaceItem], 
        process_number_space_items: ProcessNumberSpaceItems
    ) -> NumberSpace {
        let mut number_space = NumberSpace::new().process(process_number_space_items);
        for number_space_item in number_space_items {
            number_space = number_space.item(number_space_item.clone());
        }
        return number_space;
    }

    pub fn number_space_items(&self) -> &Vec<NumberSpaceItem> {
        return &self.number_space_items;
    }

    //Checks the runs when no two numbers can be the same.  A type that matches the space between any two different
    //numbers then has one run with every number in it, so any other runs can never be found.
    pub fn check_no_duplicates(&self, count: usize) -> std::result::Result<(), String> {
        for number_space_item in &self.number_space_items {
            if number_space_item.number_space_type.is_match_for_all_different_numbers() {
                let runs: Vec<usize> = number_space_item.needs.iter().copied().filter(|x| *x > 0).collect();
                let only_possible_runs: Vec<usize> = if count >= 2 { vec![count - 1] } else { Vec::new() };
                if runs != only_possible_runs {
                    return Err(format!(
                        "NumberSpace {:?} with runs {:?} is not possible with NoDuplicate. Every space between different numbers matches, so the only possible runs for count: {} are {:?}",
                        number_space_item.number_space_type, runs, count, only_possible_runs
                    ));
                }
            }
        }
        return Ok(());
    }

    pub fn from_numbers(
        number_space: &NumberSpace,
        numbers: &[usize],
        is_sorted: bool
    ) -> NumberSpace {
        return NumberSpace {
            number_space_items: NumberSpaceItem::from_numbers(&number_space.number_space_items.iter().map(|x| (x.number_space_type, Some(x.needs.clone()))).collect::<Vec<_>>(), numbers, is_sorted),
            process_number_space_items: number_space.process_number_space_items,
            errors: Vec::new(),
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
    ) -> Option<SharedData> {
        None
    }

    fn get_numbers(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<Option<Vec<usize>>, String> {
        if current_data.random().is_none() {
            return Err(current_data.no_number_err());
        }
        let other_number_pool =
        NumberSpace::from_numbers(self, current_data.selected_numbers_sorted(), true);
        let mut numbers: Vec<usize> = Vec::new();
        let mut is_missing_a_run = false;
        for number_space_item in &other_number_pool.number_space_items {
            if let Some((count, is_new_run)) = number_space_item.process_missing() {
                is_missing_a_run = true;
                //The rest of a run continues from the highest selected number.  So does a new run when that number is not
                //part of a run yet.  Any other new run starts somewhere else, because continuing would only make a run longer.
                let (_, highest_is_lone) = number_space_item.number_space_type.lone_numbers(current_data.selected_numbers_sorted());
                let start_new_run = is_new_run && !current_data.selected_numbers().is_empty() && !highest_is_lone;
                if start_new_run {
                    //Gte and Gt match any large space, so their new runs have to be right next to the selected numbers
                    match number_space_item.number_space_type.get_separated_run(current_data, count) {
                        Some(v) => numbers.extend(v),
                        None => numbers.extend(number_space_item.number_space_type.get_new_run(current_data, count))
                    }
                } else {
                    numbers.extend(number_space_item.number_space_type.get_number(current_data, count));
                }
                if self.process_number_space_items == ProcessNumberSpaceItems::OnePerRequest && !numbers.is_empty() { break; }
            }
        }
        if !numbers.is_empty() {
            return Ok(Some(numbers));
        }

        //Every run is there.  The rest of the numbers can not make a run longer, so for Gte and Gt they have to be
        //right next to a selected number.
        if !is_missing_a_run && !current_data.selected_numbers().is_empty() && current_data.selected_numbers().len() < current_data.settings().count() {
            for number_space_item in &other_number_pool.number_space_items {
                if let Some(number) = number_space_item.number_space_type.get_separated_number(current_data) {
                    return Ok(Some(vec![number]));
                }
            }
        }

        return Ok(None);
    }

    fn is_within_range(
        &self,
        current_data: &CurrentData
    ) -> std::result::Result<(), (IsWithinErrorType, String)> {

        let other_number_space = NumberSpace::from_numbers(self, current_data.selected_numbers_sorted(), true);

        let mut total_missing: usize = 0;
        for number_space_item in &other_number_space.number_space_items {
            if number_space_item.has_too_many() {
                return Err((IsWithinErrorType::Regular, format!(
                    "Too many from pool {:?}, \"needs\" is {:?} and \"has\" {:?} from this pool",
                    number_space_item.number_space_type, number_space_item.needs, number_space_item.has
                )));
            }
            if number_space_item.needs.iter().sum::<usize>() > 0 {
                total_missing += number_space_item.missing().iter().sum::<usize>();

                //Every number that is not in a run uses up one of the spare numbers.  The highest number does not count
                //because a new run can continue from it.
                let spare_numbers = current_data.settings().count().saturating_sub(number_space_item.numbers_needed());
                let (lone_count, highest_is_lone) = number_space_item.number_space_type.lone_numbers(current_data.selected_numbers_sorted());
                let lone_count = if highest_is_lone { lone_count - 1 } else { lone_count };
                if lone_count > spare_numbers {
                    return Err((IsWithinErrorType::Regular, format!(
                        "Too many numbers that are not part of a run for {:?}, there are {} and only {} can be spared",
                        number_space_item.number_space_type, lone_count, spare_numbers
                    )));
                }
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
        count: usize,
    ) -> std::result::Result<bool, String> {
        for number_space_item in &self.number_space_items {
            let numbers_needed = number_space_item.numbers_needed();
            if numbers_needed > count {
                return Err(format!(
                    "{} {:?} with runs {:?} takes {} numbers, which is greater than count: {}",
                    self.name(), number_space_item.number_space_type, number_space_item.needs, numbers_needed, count
                ));
            }
        }
        return Ok(true);
    }

    fn validate(&self, count: usize) -> Vec<String> {
        let mut errors = self.errors.clone();
        if let Err(e) = self.check_count(count) {
            errors.push(e);
        }
        return errors;
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

    fn validate_exclude(&self) -> Vec<String> {
        return self.errors.clone();
    }
}
