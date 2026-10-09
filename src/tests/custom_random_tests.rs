#[cfg(not(feature="rand"))]
use crate::random::RandomErrorType;
#[cfg(not(feature="rand"))]
use crate::random::{CurrentData, NumberType, NO_RANDOM_ERR};
use crate::random_trait::RandomTrait;
use crate::rules::NumberRange;
#[cfg(not(feature="rand"))]
use crate::rules::{NumberPool, NumberPoolByIndex, NumberPoolItem, NumberPoolItemByIndex, NumberSpace, NumberSpaceItem, OddEven, OddEvenByIndex, OperatorType, PoolType, RandomNumber, RuleTrait, Sequential, SharedData};
#[cfg(not(feature="rand"))]
use std::collections::HashSet;
#[cfg(feature="rand")]
use crate::rules::{NoDuplicate, NumberPool, OddEven};
#[cfg(feature="rand")]
use crate::seeded_random::SeededRandom;
use crate::settings::Settings;

#[test]
fn custom_random() {
    let random_result = Settings::builder(10)
        .rule(NumberRange::all(1, 20))
        .random(TestNoDefaultRandom{})
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let numbers = random_numbers.numbers();
            assert!(numbers.len() == 10);
            assert!(numbers.iter().all(|x| *x == 1), "{:?}", numbers);
            assert_eq!(random_numbers.shuffled_string().unwrap().len(), 10);
        },
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
#[cfg(not(feature="rand"))]
fn no_random_is_bad_request() {
    let random_result = Settings::builder(10)
        .rule(NumberRange::all(1, 20))
        .build()
        .generate();
    let error = random_result.unwrap_err();
    assert_eq!(error.error_type(), RandomErrorType::BadRequest);
    assert_eq!(error.messages(), &vec![NO_RANDOM_ERR.to_owned()]);
    assert_eq!(error.logs().len(), 1);
}

#[test]
#[cfg(not(feature="rand"))]
fn no_random_rules_return_errors() {
    //rules called directly without a random number generator return errors instead of panicking
    let settings = Settings::builder(5).build();
    let shared_data = SharedData::new();
    let selected = vec![];
    let current_data = CurrentData::new(&selected, &settings, &shared_data);
    assert!(current_data.random().is_none());
    assert_eq!(current_data.get_rand(NumberType::Any, None, None), None);
    let rules: Vec<Box<dyn RuleTrait>> = vec![
        Box::new(RandomNumber{}),
        Box::new(OddEven::new(3, 2)),
        Box::new(OddEvenByIndex::new(&[0], &[1])),
        Box::new(Sequential::new(3, &[2])),
        Box::new(NumberPool::new(&[NumberPoolItem::new("a", &PoolType::MinMax(1, 5), OperatorType::Eq(1))])),
        Box::new(NumberPoolByIndex::new(vec![NumberPoolItemByIndex::new("a", &PoolType::MinMax(1, 5), &HashSet::from_iter([0]))])),
        Box::new(NumberSpace::new(&[NumberSpaceItem::new(&OperatorType::Lte(3), 4)])),
    ];
    for rule in rules {
        //the error says there is no generator, not that no number fits
        assert_eq!(rule.get_numbers(&current_data), Err(NO_RANDOM_ERR.to_owned()), "{}", rule.name());
    }
}

#[test]
#[cfg(feature="rand")]
fn seeded_random_is_repeatable() {
    let run = |seed: u64| -> (Vec<usize>, String) {
        let random_result = Settings::builder(10)
            .rule(NoDuplicate)
            .rule(NumberRange::all(1, 100))
            .rule(OddEven::new(5, 5))
            .random(SeededRandom::new(seed))
            .build()
            .generate();
        assert!(random_result.is_ok(), "{:?}", random_result);
        return (random_result.as_ref().unwrap().numbers().clone(), random_result.as_ref().unwrap().shuffled_string().unwrap());
    };
    assert_eq!(run(42), run(42));
    assert_ne!(run(42).0, run(43).0);
}

#[test]
#[cfg(feature="rand")]
fn seeded_random_password_is_repeatable() {
    let run = |seed: u64| -> String {
        let random = SeededRandom::new(seed);
        let random_result = Settings::builder(12)
            .rule(NumberPool::alphanumeric_with_random(&random, 12, true))
            .random(random)
            .build()
            .generate();
        assert!(random_result.is_ok(), "{:?}", random_result);
        return random_result.as_ref().unwrap().shuffled_string().unwrap();
    };
    for seed in 0..20 {
        assert_eq!(run(seed), run(seed));
    }
}


//A random number generator does not have to implement Clone
#[derive(Debug)]
pub struct TestNoDefaultRandom {}



impl RandomTrait for TestNoDefaultRandom {
    fn get_number(&self, _min: usize, _max: usize) -> usize {
        return 1;
    }

    fn get_bool(&self) -> bool {
        return true;
    }
}