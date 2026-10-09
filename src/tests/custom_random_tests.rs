use crate::random::{random_numbers, RandomResultType};
#[cfg(not(feature="rand"))]
use crate::random::{CurrentData, NumberType};
use crate::random_trait::RandomTrait;
use crate::rules::NumberRange;
#[cfg(not(feature="rand"))]
use crate::rules::{NumberPool, NumberPoolItem, NumberSpace, NumberSpaceItem, OddEven, OperatorType, PoolType, RandomNumber, RuleTrait, Sequential, SharedData};
#[cfg(feature="rand")]
use crate::rules::{NoDuplicate, NumberPool, OddEven};
#[cfg(feature="rand")]
use crate::seeded_random::SeededRandom;
use crate::settings::Settings;

#[test]
fn custom_random() {
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NumberRange::all(1, 20))
    ], 10).with_random(TestNoDefaultRandom{}));
    match random_result.numbers() {
        Ok(numbers) => {
            assert!(numbers.len() == 10);
            assert!(numbers.iter().all(|x| *x == 1), "{:?}", numbers);
        },
        _ => panic!("{}", random_result)
    }
    assert_eq!(random_result.string(true).unwrap().len(), 10);
}

#[test]
#[cfg(not(feature="rand"))]
fn no_random_is_bad_request() {
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NumberRange::all(1, 20))
    ], 10));
    assert_eq!(random_result.status(), RandomResultType::BadRequest);
    assert!(random_result.numbers().is_err());
    assert!(random_result.string(true).is_err());
    assert_eq!(random_result.logs().len(), 1);
}

#[test]
#[cfg(not(feature="rand"))]
fn no_random_rules_return_errors() {
    //rules called directly without a random number generator return errors instead of panicking
    let settings = Settings::new(&[], 5);
    let shared_data = SharedData::new();
    let selected = vec![];
    let current_data = CurrentData::new(&selected, &settings, &shared_data);
    assert!(current_data.random().is_none());
    assert_eq!(current_data.get_rand(NumberType::Any, None, None), None);
    assert!(RandomNumber{}.get_numbers(&current_data).is_err());
    assert!(OddEven::new(3, 2).get_numbers(&current_data).is_err());
    assert!(Sequential::new(3, &[2]).get_numbers(&current_data).is_err());
    assert!(NumberPool::new(&[NumberPoolItem::new("a", &PoolType::MinMax(1, 5), OperatorType::Eq(1))]).get_numbers(&current_data).is_err());
    assert!(NumberSpace::new(&[NumberSpaceItem::new(&OperatorType::Lte(3), 4)]).get_numbers(&current_data).unwrap().is_none());
}

#[test]
#[cfg(feature="rand")]
fn seeded_random_is_repeatable() {
    let run = |seed: u64| -> (Vec<usize>, String) {
        let random_result = random_numbers(&Settings::new(&[
            Box::new(NoDuplicate{}),
            Box::new(NumberRange::all(1, 100)),
            Box::new(OddEven::new(5, 5))
        ], 10).with_random(SeededRandom::new(seed)));
        assert_eq!(random_result.status(), RandomResultType::Success, "{}", random_result);
        return (random_result.numbers().unwrap().clone(), random_result.string(true).unwrap());
    };
    assert_eq!(run(42), run(42));
    assert_ne!(run(42).0, run(43).0);
}

#[test]
#[cfg(feature="rand")]
fn seeded_random_password_is_repeatable() {
    let run = |seed: u64| -> String {
        let random = SeededRandom::new(seed);
        let random_result = random_numbers(&Settings::new(&[
            Box::new(NumberPool::alphanumeric_with_random(&random, 12, true))
        ], 12).with_random(random));
        assert_eq!(random_result.status(), RandomResultType::Success, "{}", random_result);
        return random_result.string(true).unwrap();
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