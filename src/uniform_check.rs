//Checks rules for uniform settings: that can_still_match only says no when no numbers added could make the
//numbers match.  A rule that says no too soon makes uniform settings leave tickets out, so a custom rule's
//can_still_match should only be used when this finds no wrong stops for it.
//
//  let settings = Settings::builder(4).range(1, 10).no_duplicates().rule(MyRule::new()).uniform().build();
//  let check = check_uniform(&settings).unwrap();
//  assert!(check.wrong_stops.is_empty(), "{:?}", check.wrong_stops);
//
//It goes through every ticket the settings can draw, so it is for small ranges and counts: 4 numbers from
//1 to 10 are 5,040 tickets.

use crate::random::{uniform_can_go_on, uniform_matches, uniform_ranges};
use crate::settings::Settings;
use std::collections::HashSet;

//What check_uniform found
pub struct UniformCheck {
    //Every ticket that uniform generation can give, in the order it draws the numbers
    pub matching: Vec<Vec<usize>>,
    //Partial or full tickets that a rule stopped, although numbers added to them could make them match
    pub wrong_stops: Vec<Vec<usize>>,
}

//Goes through every ticket the uniform settings can draw.  Err when the settings can not be uniform.
pub fn check_uniform(settings: &Settings) -> Result<UniformCheck, String> {
    let (ranges, no_duplicates) = uniform_ranges(settings)?;
    let mut matching: Vec<Vec<usize>> = Vec::new();
    let mut stopped: Vec<Vec<usize>> = Vec::new();
    //every full ticket that matches, whether or not something stopped it on the way
    let mut all_matching: Vec<Vec<usize>> = Vec::new();
    for_each_drawn(&ranges, no_duplicates, &mut |numbers| {
        let can_go_on = uniform_can_go_on(settings, numbers);
        if !can_go_on {
            stopped.push(numbers.to_vec());
        }
        if numbers.len() == ranges.len() && uniform_matches(settings, numbers) {
            all_matching.push(numbers.to_vec());
            if can_go_on {
                matching.push(numbers.to_vec());
            }
        }
    });
    let starts: HashSet<&[usize]> = all_matching.iter().flat_map(|x| (1..=x.len()).map(move |len| &x[..len])).collect();
    let wrong_stops: Vec<Vec<usize>> = stopped.into_iter().filter(|x| starts.contains(x.as_slice())).collect();
    return Ok(UniformCheck { matching, wrong_stops });
}

//Every partial and full ticket, as uniform generation draws them: in index order, from each index's range,
//without repeats when the rules say so
fn for_each_drawn(ranges: &[(usize, usize)], no_duplicates: bool, f: &mut dyn FnMut(&[usize])) {
    fn go(ranges: &[(usize, usize)], no_duplicates: bool, numbers: &mut Vec<usize>, f: &mut dyn FnMut(&[usize])) {
        if !numbers.is_empty() {
            f(numbers);
        }
        if numbers.len() == ranges.len() {
            return;
        }
        let (min, max) = ranges[numbers.len()];
        for number in min..=max {
            if no_duplicates && numbers.contains(&number) {
                continue;
            }
            numbers.push(number);
            go(ranges, no_duplicates, numbers, f);
            numbers.pop();
        }
    }
    go(ranges, no_duplicates, &mut Vec::new(), f);
}
