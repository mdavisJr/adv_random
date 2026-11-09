//Checks rules for uniform settings: that can_still_match only says no when no numbers added could make the
//numbers match.  A rule that says no too soon makes uniform settings leave tickets out, so a custom rule's
//can_still_match should only be used when these find no wrong stops for it.
//
//  let settings = Settings::builder(4).range(1, 10).no_duplicates().rule(MyRule::new()).uniform().build();
//  let check = check_uniform(&settings).unwrap();
//  assert!(check.wrong_stops.is_empty(), "{:?}", check.wrong_stops);
//
//check_uniform goes through every ticket the settings can draw, so it is for small ranges and counts: 4
//numbers from 1 to 10 are 5,040 tickets.  sample_uniform checks random tickets instead, for settings of any
//size.

use crate::random::{uniform_can_go_on, uniform_matches, uniform_ranges, Choices, NO_RANDOM_ERR};
use crate::settings::Settings;
use std::collections::HashSet;

//What check_uniform or sample_uniform found
pub struct UniformCheck {
    //The tickets that uniform generation can give, in the order it draws the numbers: every one for
    //check_uniform, the ones drawn for sample_uniform
    pub matching: Vec<Vec<usize>>,
    //Partial or full tickets that a rule stopped, although numbers added to them could make them match
    pub wrong_stops: Vec<Vec<usize>>,
}

//Goes through every ticket the uniform settings can draw.  Err when the settings can not be uniform.
pub fn check_uniform(settings: &Settings) -> Result<UniformCheck, String> {
    let (choices, no_duplicates) = uniform_ranges(settings)?;
    let mut matching: Vec<Vec<usize>> = Vec::new();
    let mut stopped: Vec<Vec<usize>> = Vec::new();
    //every full ticket that matches, whether or not something stopped it on the way
    let mut all_matching: Vec<Vec<usize>> = Vec::new();
    for_each_drawn(&choices, no_duplicates, &mut |numbers| {
        let can_go_on = uniform_can_go_on(settings, numbers);
        if !can_go_on {
            stopped.push(numbers.to_vec());
        }
        if numbers.len() == choices.len() && uniform_matches(settings, numbers) {
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

//Draws tickets random tickets as uniform generation does, but without stopping any early, and for each one
//that matches, asks can_still_match about every start of it.  Every no is a wrong stop.  The random number
//generator of the settings draws them, so a seeded one checks the same tickets every time.  A wrong stop it
//finds is certain.  Finding none means no wrong stop was in the tickets that matched, so draw enough of them.
pub fn sample_uniform(settings: &Settings, tickets: usize) -> Result<UniformCheck, String> {
    let (choices, no_duplicates) = uniform_ranges(settings)?;
    let random = settings.random().ok_or_else(|| NO_RANDOM_ERR.to_owned())?;
    let mut matching: Vec<Vec<usize>> = Vec::new();
    let mut wrong_stops: HashSet<Vec<usize>> = HashSet::new();
    let mut numbers: Vec<usize> = Vec::with_capacity(choices.len());
    'tickets: for _ in 0..tickets {
        numbers.clear();
        for index_choices in &choices {
            match index_choices.draw(random.as_ref(), &numbers, no_duplicates) {
                Some(number) => numbers.push(number),
                None => continue 'tickets,
            }
        }
        if uniform_matches(settings, &numbers) {
            for len in 1..=numbers.len() {
                if !uniform_can_go_on(settings, &numbers[..len]) {
                    wrong_stops.insert(numbers[..len].to_vec());
                }
            }
            matching.push(numbers.clone());
        }
    }
    let mut wrong_stops: Vec<Vec<usize>> = wrong_stops.into_iter().collect();
    wrong_stops.sort();
    return Ok(UniformCheck { matching, wrong_stops });
}

//Every partial and full ticket, as uniform generation draws them: in index order, from each index's
//choices, without repeats when the rules say so
fn for_each_drawn(choices: &[Choices], no_duplicates: bool, f: &mut dyn FnMut(&[usize])) {
    fn go(choices: &[Choices], no_duplicates: bool, numbers: &mut Vec<usize>, f: &mut dyn FnMut(&[usize])) {
        if !numbers.is_empty() {
            f(numbers);
        }
        if numbers.len() == choices.len() {
            return;
        }
        for number in choices[numbers.len()].numbers() {
            if no_duplicates && numbers.contains(&number) {
                continue;
            }
            numbers.push(number);
            go(choices, no_duplicates, numbers, f);
            numbers.pop();
        }
    }
    go(choices, no_duplicates, &mut Vec::new(), f);
}
