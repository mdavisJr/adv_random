use crate::rules::*;
use crate::seeded_random::SeededRandom;
use crate::settings::*;
use crate::uniform_check::check_uniform;
use std::collections::HashMap;

//The matching tickets and the wrong stops, from check_uniform
fn matches_and_wrong_stops(settings: &Settings) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let check = check_uniform(settings).unwrap();
    return (check.matching, check.wrong_stops);
}

//The rules, each with a range and no repeats unless it says otherwise, on 4 numbers from 1 to 10
fn rule_cases() -> Vec<(&'static str, SettingsBuilder)> {
    let b = || Settings::builder(4).range(1, 10).no_duplicates();
    vec![
        ("range only", b()),
        ("OddEven 2/2", b().rule(OddEven::new().odd(2).even(2))),
        ("OddEven 4/0", b().rule(OddEven::new().odd(4).even(0))),
        ("OddEven 1/3", b().rule(OddEven::new().odd(1).even(3))),
        ("Sequential 4 singles", b().rule(Sequential::new().singles(4))),
        ("Sequential 2 singles + 2 in a row", b().rule(Sequential::new().singles(2).in_a_row(2))),
        ("Sequential 2 + 2 in a row", b().rule(Sequential::new().in_a_row(2).in_a_row(2))),
        ("Sequential 1 single + 3 in a row", b().rule(Sequential::new().singles(1).in_a_row(3))),
        ("Sequential 4 in a row", b().rule(Sequential::new().in_a_row(4))),
        ("NumberPool exactly 2 of 1-3", b().rule(NumberPool::new().exactly(2, [1, 2, 3]))),
        ("NumberPool at least 1 of 9-10", b().rule(NumberPool::new().at_least(1, [9, 10]))),
        ("NumberPool at most 1 of 1-4", b().rule(NumberPool::new().at_most(1, [1, 2, 3, 4]))),
        ("NumberPool between 1 and 2 of 5-7", b().rule(NumberPool::new().between(1, 2, [5, 6, 7]))),
        ("NumberPool none of 5", b().rule(NumberPool::new().none_of([5]))),
        ("NumberPool exactly 1 of 1-2, at least 2 of 6-10", b().rule(NumberPool::new().exactly(1, [1, 2]).at_least(2, [6, 7, 8, 9, 10]))),
        ("NumberPool spread 2,1,1", b().rule(NumberPool::new().spread([2, 1, 1]).over(1..=3).over(4..=6).over(7..=10))),
        ("NumberPool spread 3,1,0", b().rule(NumberPool::new().spread([3, 1, 0]).over(1..=3).over(4..=6).over(7..=10))),
        ("NumberSpace 3 gaps under 3", b().rule(NumberSpace::new().run(OperatorType::Lt(3), 3))),
        ("NumberSpace 3 gaps of 3 or more", b().rule(NumberSpace::new().run(OperatorType::Gte(3), 3))),
        ("NumberSpace 3 gaps 2 to 4", b().rule(NumberSpace::new().run(OperatorType::Between(2, 4), 3))),
        ("NumberSpace 1 gap of 1 and 2 over 2", b().rule(NumberSpace::new().run(OperatorType::Eq(1), 1).run(OperatorType::Gt(2), 2))),
        ("OddEvenByIndex odd 0,2 even 1,3", b().rule(OddEvenByIndex::new().odd([0, 2]).even([1, 3]))),
        ("OddEvenByIndex odd 3", b().rule(OddEvenByIndex::new().odd([3]))),
        ("NumberPoolByIndex", b().rule(NumberPoolByIndex::new().at([0], [1, 2, 3]).at([1, 2], 4..=8))),
        ("NumberRange at indexes", Settings::builder(4).no_duplicates().rule(NumberRange::new().at([0], 1, 3).at([1, 2, 3], 4, 10))),
        ("ExcludeNumberSets", b().exclude(ExcludeNumberSets::new().numbers(&[1, 2, 3, 4]).numbers(&[2, 4, 6, 8]))),
        ("AnyOf OddEven 2/2 or 4 in a row", b().rule(AnyOf::new().rule(OddEven::new().odd(2).even(2)).rule(Sequential::new().in_a_row(4)))),
        (
            "OddEven + Sequential + NumberPool",
            b().rule(OddEven::new().odd(3).even(1))
                .rule(Sequential::new().singles(2).in_a_row(2))
                .rule(NumberPool::new().at_least(1, [8, 9, 10])),
        ),
        ("repeats allowed, OddEven 2/2", Settings::builder(4).range(1, 6).rule(OddEven::new().odd(2).even(2))),
    ]
}

//No rule stops a partial ticket that numbers could still be added to so it matches.  Goes through every
//ticket of every case.
#[test]
fn rules_only_stop_tickets_that_can_not_match() {
    let mut problems: Vec<String> = Vec::new();
    for (name, builder) in rule_cases() {
        let settings = builder.uniform().build();
        if !settings.errors().is_empty() {
            problems.push(format!("{}: settings errors {:?}", name, settings.errors()));
            continue;
        }
        let (matching, wrong) = matches_and_wrong_stops(&settings);
        println!("{:<50} {:>5} matching tickets, {} wrong stops", name, matching.len(), wrong.len());
        if !wrong.is_empty() {
            problems.push(format!(
                "{}: {} partial tickets stopped that could match, such as {:?}",
                name,
                wrong.len(),
                &wrong[..wrong.len().min(3)]
            ));
        }
    }
    assert!(problems.is_empty(), "\n{}", problems.join("\n"));
}

//Small cases, quick enough for every test run: 3 numbers from 1 to 6 or 8
fn small_cases() -> Vec<(&'static str, SettingsBuilder)> {
    let b = |max: usize| Settings::builder(3).range(1, max).no_duplicates();
    vec![
        ("small: OddEven 2/1", b(6).rule(OddEven::new().odd(2).even(1))),
        ("small: Sequential 3 in a row", b(6).rule(Sequential::new().in_a_row(3))),
        ("small: Sequential 1 single + 2 in a row", b(6).rule(Sequential::new().singles(1).in_a_row(2))),
        ("small: NumberSpace 2 gaps under 3", b(8).rule(NumberSpace::new().run(OperatorType::Lt(3), 2))),
        ("small: NumberPool spread 2,1", b(6).rule(NumberPool::new().spread([2, 1]).over(1..=3).over(4..=6))),
        ("small: AnyOf", b(6).rule(AnyOf::new().rule(OddEven::new().odd(3).even(0)).rule(Sequential::new().in_a_row(3)))),
        ("small: ExcludeNumberSets", b(6).exclude(ExcludeNumberSets::new().numbers(&[1, 2, 3]).numbers(&[4, 5, 6]))),
        ("small: repeats allowed, OddEven 2/1", Settings::builder(3).range(1, 4).rule(OddEven::new().odd(2).even(1))),
        (
            "small: pools without a range",
            Settings::builder(3).no_duplicates().rule(NumberPool::new().exactly(2, [1, 2, 3, 4]).exactly(1, [8, 9])),
        ),
        (
            "small: pools by index without a range",
            Settings::builder(3).rule(NumberPoolByIndex::new().at([0], [1, 2, 3]).at([1, 2], [5, 6, 7])),
        ),
    ]
}

//Every matching ticket comes up about as often as every other, in the small cases
#[test]
fn uniform_makes_every_matching_ticket_equally_likely() {
    assert_eq!(equally_likely_problems(small_cases(), 200, 150), small_cases().len());
}

//The same for every case of 4 numbers from 1 to 10 with up to 2,000 matching tickets
#[ignore = "slow: about 15 seconds with --release; run it with --release --ignored"]
#[test]
fn uniform_makes_every_matching_ticket_equally_likely_in_every_case() {
    equally_likely_problems(rule_cases(), 2_000, 200);
}

//Checks every case with up to max_tickets matching tickets, drawing per_ticket for each one.  Panics with
//the problems, and returns how many cases were tested.
fn equally_likely_problems(cases: Vec<(&'static str, SettingsBuilder)>, max_tickets: usize, per_ticket: usize) -> usize {
    let mut problems: Vec<String> = Vec::new();
    let mut tested = 0;
    for (name, builder) in cases {
        let settings = builder.uniform().random(SeededRandom::new(7)).build();
        if !settings.errors().is_empty() {
            continue;
        }
        let (matching, _) = matches_and_wrong_stops(&settings);
        if matching.is_empty() || matching.len() > max_tickets {
            continue;
        }
        tested += 1;
        let draws = matching.len() * per_ticket;
        let mut counts: HashMap<Vec<usize>, usize> = HashMap::new();
        for _ in 0..draws {
            *counts.entry(settings.generate().unwrap().into_numbers()).or_default() += 1;
        }
        let unknown = counts.keys().filter(|x| !matching.contains(x)).count();
        let p = 1.0 / matching.len() as f64;
        let z: Vec<f64> = matching
            .iter()
            .map(|x| (counts.get(x).copied().unwrap_or(0) as f64 - draws as f64 * p) / (draws as f64 * p * (1.0 - p)).sqrt())
            .collect();
        let worst = z.iter().map(|x| x.abs()).fold(0.0, f64::max);
        let mean_square = z.iter().map(|x| x * x).sum::<f64>() / z.len() as f64;
        let allowed = 6.0 * (2.0 / z.len() as f64).sqrt() + 0.05;
        println!("{:<50} {:>5} tickets, worst {:.1} sd, spread {:.3}", name, matching.len(), worst, mean_square);
        if unknown > 0 || worst > 6.0 || (mean_square - 1.0).abs() > allowed {
            problems.push(format!(
                "{}: {} tickets that don't match, worst {:.1} sd, spread {:.3} (allowed 1 ± {:.3})",
                name, unknown, worst, mean_square, allowed
            ));
        }
    }
    assert!(problems.is_empty(), "\n{}", problems.join("\n"));
    return tested;
}

//When nothing matches it stops at max_draws, with the logs cut short and how often each rule stopped a ticket
#[test]
fn uniform_stops_at_max_draws_and_counts_what_stopped_it() {
    let settings = Settings::builder(4)
        .range(1, 10)
        .no_duplicates()
        .rule(OddEven::new().odd(4).even(0))
        .rule(Sequential::new().in_a_row(4))
        .uniform()
        .max_draws(20_000)
        .build();
    let error = settings.generate().unwrap_err();
    assert_eq!(error.attempts(), 20_000);
    assert!(error.logs().len() <= 10_001, "{}", error.logs().len());
    let last = error.logs().last().unwrap().to_string();
    assert!(last.contains("more entries were not kept") && last.contains("Sequential") && last.contains("OddEven"), "{}", last);
    println!("{}", last);
}

//Uniform logs keep one number for every draw, and their text says every number drawn so far in the attempt
#[test]
fn uniform_logs_say_every_number_drawn_in_the_attempt() {
    let settings = Settings::builder(5)
        .range(1, 39)
        .no_duplicates()
        .rule(OddEven::new().odd(4).even(1))
        .rule(NumberPool::new().exactly(3, 1..=13))
        .random(SeededRandom::new(2))
        .uniform()
        .build();
    let numbers = settings.generate().unwrap();
    let list = |text: &str| -> Vec<usize> {
        let text = text.trim().trim_start_matches('[').trim_end_matches(']');
        if text.is_empty() {
            Vec::new()
        } else {
            text.split(", ").map(|x| x.parse().unwrap()).collect()
        }
    };
    let mut drawn: Vec<usize> = Vec::new();
    let (mut attempts, mut draws, mut clears) = (0, 0, 0);
    for log in numbers.logs() {
        let text = log.to_string();
        if text.starts_with("Info  : Attempt - ") {
            attempts += 1;
            drawn.clear();
        } else if let Some(rest) = text.strip_prefix("Info  : GEN_TYPE - RandomNumber; P - ") {
            let (p, all) = rest.split_once("; A&P - ").unwrap();
            drawn.extend(list(p));
            assert_eq!(list(all), drawn, "{}", text);
            draws += 1;
        } else if let Some(rest) = text.strip_prefix("Info  : CLEAR - ") {
            assert_eq!(list(rest), drawn, "{}", text);
            clears += 1;
        }
    }
    assert_eq!(attempts, numbers.attempts());
    assert_eq!(clears, attempts - 1);
    assert!(draws > attempts, "{} draws in {} attempts", draws, attempts);
    // The last attempt's numbers are the ones made
    assert_eq!(&drawn, numbers.numbers());
}

//check_uniform refuses settings with too many tickets, and max_draws(0) is a mistake
#[test]
fn check_uniform_is_for_small_settings_and_max_draws_is_at_least_1() {
    let big = Settings::builder(6).range(1, 49).no_duplicates().uniform().build();
    let error = check_uniform(&big).err().unwrap();
    assert!(error.contains("Use sample_uniform"), "{}", error);
    let settings = Settings::builder(3).range(1, 9).uniform().max_draws(0).build();
    assert_eq!(settings.errors(), ["max_draws must be at least 1"]);
    assert_eq!(settings.generate().unwrap_err().error_type(), crate::random::RandomErrorType::BadRequest);
}

//Uniform without repeats needs enough different numbers
#[test]
fn uniform_needs_enough_numbers_without_repeats() {
    let error = Settings::builder(5).range(1, 3).no_duplicates().uniform().build().generate().unwrap_err();
    assert_eq!(error.attempts(), 0);
    assert!(error.messages().iter().any(|x| x.contains("needs 5 different numbers, and the ranges only have 3")), "{:?}", error.messages());
    // With repeats, 5 from 1 to 3 is fine
    assert_eq!(Settings::builder(5).range(1, 3).uniform().build().generate().unwrap().numbers().len(), 5);
}

//AnyOf works without uniform too, and needs rules
#[test]
fn any_of_works_without_uniform_and_needs_rules() {
    for _ in 0..200 {
        let numbers = Settings::builder(4)
            .range(1, 10)
            .no_duplicates()
            .rule(AnyOf::new().rule(OddEven::new().odd(4).even(0)).rule(OddEven::new().odd(0).even(4)))
            .build()
            .generate()
            .unwrap()
            .into_numbers();
        let odd = numbers.iter().filter(|x| *x % 2 == 1).count();
        assert!(odd == 4 || odd == 0, "{:?}", numbers);
    }
    let settings = Settings::builder(4).range(1, 10).rule(AnyOf::new()).build();
    assert!(settings.errors().iter().any(|x| x == "AnyOf has no rules"), "{:?}", settings.errors());
}

//Without uniform, nothing changes; uniform needs a range
#[test]
fn uniform_needs_a_range_and_is_off_by_default() {
    assert!(!Settings::builder(3).range(1, 9).build().uniform());
    let error = Settings::builder(3).rule(OddEven::new().odd(2).even(1)).uniform().build().generate().unwrap_err();
    assert!(error.messages().iter().any(|x| x.contains("Uniform needs a range")), "{:?}", error.messages());
}

//Pools give the numbers to draw when every number has to come from them, so settings like passwords work
//without a range
#[test]
fn uniform_draws_from_pools_every_number_comes_from() {
    // The README's 15 characters: 11 capital letters, 2 of "365" and 2 of "#*"
    let settings = Settings::builder(15)
        .rule(NumberPool::new().exactly(11, "ABCDEFGHIJKLMNOPQRSTUVWXYZ").exactly(2, "365").exactly(2, "#*"))
        .uniform()
        .build();
    for _ in 0..50 {
        let text = settings.generate().unwrap().string().unwrap();
        assert_eq!(text.len(), 15);
        assert_eq!(text.chars().filter(|x| x.is_ascii_uppercase()).count(), 11, "{}", text);
        assert_eq!(text.chars().filter(|x| "365".contains(*x)).count(), 2, "{}", text);
        assert_eq!(text.chars().filter(|x| "#*".contains(*x)).count(), 2, "{}", text);
    }
    // The README's license number: 2 letters and 7 digits, but not two that are taken
    let settings = Settings::builder(9)
        .rule(ExcludeNumberSets::new().string("AB1234567").string("CB1234567"))
        .rule(NumberPoolByIndex::new().at([0, 1], "ABCDEFGHIJKLMNOPQRSTUVWXYZ").at(2..=8, "1234567890"))
        .uniform()
        .build();
    for _ in 0..50 {
        let text = settings.generate().unwrap().string().unwrap();
        assert!(text[..2].chars().all(|x| x.is_ascii_uppercase()) && text[2..].chars().all(|x| x.is_ascii_digit()), "{}", text);
    }
    // Pools that share numbers can count a number twice, so they don't say where every number comes from
    let error =
        Settings::builder(4).rule(NumberPool::new().exactly(2, [1, 2, 3]).exactly(2, [3, 4, 5])).uniform().build().generate().unwrap_err();
    assert!(error.messages().iter().any(|x| x.contains("Uniform needs a range")), "{:?}", error.messages());
}

//sample_uniform finds a can_still_match that says no too soon, and none in the built-in rules on settings
//too big to go through every ticket
#[test]
fn sample_uniform_finds_wrong_stops() {
    use crate::random::CurrentData;
    use crate::uniform_check::sample_uniform;
    // Says the numbers can't match once the first one is odd, but every ticket matches
    #[derive(Clone, Debug)]
    struct OddFirstStops;
    impl std::fmt::Display for OddFirstStops {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "OddFirstStops")
        }
    }
    impl RuleTrait for OddFirstStops {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
        fn share_data(&self, _c: &CurrentData) -> Option<SharedData> {
            None
        }
        fn get_numbers(&self, _c: &CurrentData) -> Result<Option<Vec<usize>>, String> {
            Ok(None)
        }
        fn is_within_range(&self, _c: &CurrentData) -> Result<(), (IsWithinErrorType, String)> {
            Ok(())
        }
        fn is_match(&self, _c: &CurrentData) -> Result<(), String> {
            Ok(())
        }
        fn check_count(&self, _count: usize) -> Result<bool, String> {
            Ok(true)
        }
        fn name(&self) -> String {
            "OddFirstStops".into()
        }
        fn can_still_match(&self, c: &CurrentData) -> bool {
            c.selected_numbers()[0].is_multiple_of(2)
        }
    }
    let settings = Settings::builder(6).range(1, 40).no_duplicates().rule(OddFirstStops).random(SeededRandom::new(3)).uniform().build();
    let check = sample_uniform(&settings, 200).unwrap();
    assert_eq!(check.matching.len(), 200);
    assert!(
        !check.wrong_stops.is_empty() && check.wrong_stops.iter().all(|x| x[0] % 2 == 1),
        "{:?}",
        &check.wrong_stops[..3.min(check.wrong_stops.len())]
    );

    let b = || Settings::builder(6).range(1, 40).no_duplicates().random(SeededRandom::new(5));
    for (name, builder) in [
        ("OddEven 4/2", b().rule(OddEven::new().odd(4).even(2))),
        ("NumberPool", b().rule(NumberPool::new().exactly(2, 1..=10).at_least(1, [38, 39, 40]).none_of([20]))),
        ("NumberPool spread", b().rule(NumberPool::new().spread([3, 2, 1]).over(1..=13).over(14..=26).over(27..=40))),
        ("OddEvenByIndex", b().rule(OddEvenByIndex::new().odd([0, 2, 4]).even([1]))),
        ("AnyOf", b().rule(AnyOf::new().rule(OddEven::new().odd(6).even(0)).rule(NumberPool::new().at_least(3, 1..=10)))),
    ] {
        let check = sample_uniform(&builder.uniform().build(), 20_000).unwrap();
        assert!(check.matching.len() > 50, "{}: only {} matched", name, check.matching.len());
        assert!(check.wrong_stops.is_empty(), "{}: {:?}", name, &check.wrong_stops[..3.min(check.wrong_stops.len())]);
    }
}
