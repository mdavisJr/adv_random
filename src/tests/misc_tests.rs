use crate::default_random::DefaultRandom;
use crate::random_trait::{get_random_vec_item, RandomTrait};
use crate::rules::RuleTrait;
use crate::rules::*;
use crate::random::*;
use crate::settings::*;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::sync::{LazyLock, Mutex};

const MAX_TRIES: usize = 2000;
static NUMBER_RANGE: LazyLock<Mutex<NumberRange>> = LazyLock::new(|| { Mutex::new(NumberRange::all(1, 100)) });

#[test]
fn odd_even_with_order_2() {
    let random_result = random_numbers(&Settings::builder(5)
        .rule(OddEvenByIndex::new(&[0, 2, 4], &[1,3]))
        .build());
    match random_result.numbers() {
        Ok(s) => println!("{:?}", s),
        _ => panic!("{}", random_result)
    }
}

#[test]
fn sequential_1() {
    let data: Vec<(&str, Sequential, &[usize])> = vec![
        ("NOT:5", Sequential::new(5, &[]), &[1, 15, 10, 25, 8]),
        ("NOT:2,SEQ1:3", Sequential::new(2, &[3]), &[1, 2, 10, 6, 3]),
        ("NOT:3,SEQ1:2", Sequential::new(3, &[2]), &[1, 2, 26, 6, 8]),
        (
            "NOT:0,SEQ1:2,SEQ2:2",
            Sequential::new(0, &[2, 2]),
            &[1, 2, 4, 5],
        ),
        (
            "NOT:1,SEQ1:2,SEQ2:2",
            Sequential::new(1, &[2, 2]),
            &[1, 2, 4, 7, 8],
        ),
        ("NOT:0,SEQ1:5", Sequential::new(0, &[5]), &[1, 2, 3, 4, 5]),
        (
            "NOT:3,SEQ1:2",
            Sequential::new(3, &[2]),
            &[1, 10, 15, 18, 17],
        ),
        (
            "NOT:3,SEQ1:3",
            Sequential::new(3, &[3]),
            &[1, 2, 3, 6, 8, 10],
        ),
    ];
    for (main_idx, item) in data.iter().enumerate() {
        let current_data_numbers: Vec<usize> = item.2.to_vec();
        let current_data_settings: Settings = Settings::builder(item.2.len()).build();
        let current_data_shared_data: SharedData = SharedData::new();
        let current_data = CurrentData::new(&current_data_numbers, &current_data_settings, &current_data_shared_data);
        let seq_numbers = [item.1.clone(), Sequential::from_numbers(&current_data, false)];
        for (idx, seq_number) in seq_numbers.iter().enumerate() {
            assert_eq!(
                item.0,
                seq_number.to_string(),
                "MAIN IDX: {}, FROM_NUMBERS: {}; SEQ_NUM: {:?}",
                main_idx,
                idx == 1,
                seq_number
            );
            
            assert_eq!(
                Ok(()),
                seq_number.is_within_range(&current_data),
                "MAIN IDX: {}, FROM_NUMBERS: {}; SEQ_NUM: {:?}",
                main_idx,
                idx == 1,
                seq_number
            );
            assert_eq!(
                Ok(()),
                seq_number.is_match(&current_data),
                "MAIN IDX: {}, FROM_NUMBERS: {}; SEQ_NUM: {:?}",
                main_idx,
                idx == 1,
                seq_number
            );
            let rand_result = random_numbers(&Settings::builder(item.2.len())
                .rule(seq_number.clone())
                .rule(NUMBER_RANGE.lock().unwrap().clone())
                .build());
            assert_eq!(
                RandomResultType::Success,
                rand_result.status(),
                "MAIN IDX: {}, FROM_NUMBERS: {}; SEQ_NUM: {:?} \n\n{:?}",
                main_idx,
                idx == 1,
                seq_number,
                rand_result
            );
            assert_eq!(
                Ok(()),
                Sequential::from_numbers(&current_data, false).is_match(&current_data),
                "MAIN IDX: {}, FROM_NUMBERS: {}; SEQ_NUM: {:?} \n\n{:?}",
                main_idx,
                idx == 1,
                seq_number,
                rand_result
            );
        }
    }
}

#[test]
fn number_range_1() {
    type IsWithinRangeResult = std::result::Result<(), (IsWithinErrorType, String)>;
    type IsMatchResult = std::result::Result<(), String>;
    let data: Vec<(&str, NumberRange, &[usize], IsWithinRangeResult, IsMatchResult)> = vec![
        ("[(0, (1, 100))]", NumberRange::all(1, 100), &[1, 99, 4, 6, 7], Ok(()), Ok(())),
        ("[(0, (1, 100)), (1, (1, 100)), (2, (1, 100)), (3, (1, 100)), (4, (1, 100))]", NumberRange::from_map(&[(&[0,1,2,3,4], 1 , 100)]), &[1, 99, 4, 6, 7], Ok(()), Ok(())),
        ("[(0, (1, 1)), (1, (99, 99)), (2, (4, 4)), (3, (6, 6)), (4, (7, 7))]", NumberRange::from_map(&[(&[0], 1,1), (&[1], 99,99), (&[2], 4,4), (&[3], 6,6), (&[4], 7,7)]), &[1, 99, 4, 6, 7], Ok(()), Ok(())),

        ("[(0, (1, 100))]", NumberRange::all(1, 100), &[1, 99, 4, 6, 101], Err((IsWithinErrorType::Regular, "Exclude: false - Selected number 101 at index 4 is not within range of min: 1 and max: 100. Numbers:[1, 99, 4, 6, 101].  Map Index:0".to_owned())), Err("Exclude: false - Selected number 101 at index 4 is not within range of min: 1 and max: 100. Numbers:[1, 99, 4, 6, 101].  Map Index:0".to_owned())),
        ("[(0, (1, 100)), (1, (1, 100)), (2, (1, 100)), (3, (1, 100)), (4, (1, 100))]", NumberRange::from_map(&[(&[0,1,2,3,4], 1 , 100)]), &[1, 100, 4, 6, 7], Ok(()), Ok(())),
        ("[(0, (1, 1)), (1, (99, 99)), (2, (4, 4)), (3, (6, 6)), (4, (7, 7))]", NumberRange::from_map(&[(&[0], 1,1), (&[1], 99,99), (&[2], 4,4), (&[3], 6,6), (&[4], 7,7)]), &[1, 99, 5, 6, 7], Err((IsWithinErrorType::Regular, "Exclude: false - Selected number 5 at index 2 is not within range of min: 4 and max: 4. Numbers:[1, 99, 5, 6, 7].  Map Index:2".to_owned())), Err("Exclude: false - Selected number 5 at index 2 is not within range of min: 4 and max: 4. Numbers:[1, 99, 5, 6, 7].  Map Index:2".to_owned())),
    ];
    for (main_idx, item) in data.iter().enumerate() {
        let current_data_numbers: Vec<usize> = item.2.to_vec();
        let current_data_settings: Settings = Settings::builder(item.2.len()).build();
        let current_data_shared_data: SharedData = SharedData::new();
        let current_data = CurrentData::new(&current_data_numbers, &current_data_settings, &current_data_shared_data);
        assert_eq!(
            item.0,
            item.1.to_string(),
            "MAIN IDX: {}, NUM_RANGE: {:?}",
            main_idx,
            item.1
        );
        assert_eq!(
            item.3,
            item.1.is_within_range(&current_data),
            "MAIN IDX: {},  NUM_RANGE: {:?}",
            main_idx,
            item.1
        );
        assert_eq!(
            item.4,
            item.1.is_match(&current_data),
            "MAIN IDX: {},  NUM_RANGE: {:?}",
            main_idx,
            item.1
        );
        let rand_result = random_numbers(&Settings::builder(item.2.len()).rule(item.1.clone()).build());
        assert_eq!(
            RandomResultType::Success,
            rand_result.status(),
            "MAIN IDX: {}, NUM_RANGE: {:?} \n\n{:?}",
            main_idx,
            item.1,
            rand_result
        );
        assert_eq!(
            Ok(()),
            item.1.is_match(&CurrentData::new(rand_result.numbers().unwrap(), &Settings::builder(item.2.len()).build(), &SharedData::new())),
            "MAIN IDX: {}, NUM_RANGE: {:?} \n\n{:?}",
            main_idx,
            item.1,
            rand_result
        );
    }
}

#[test]
fn number_range_2() {
    let rules: Vec<Box<dyn RuleTrait>> = vec![Box::new(NumberRange::all(1, 2))];
    let settings = Settings::builder(10).boxed_rules(rules.clone()).build();
    for _ in 0..MAX_TRIES {
        let random_result = random_numbers(&settings);
        assert_eq!(
            RandomResultType::Success,
            random_result.status(),
            "{:?}",
            random_result
        );
        assert_eq!(random_result.numbers().unwrap().len(), 10);
        for number in random_result.numbers().unwrap() {
            assert!(*number == 1 || *number == 2);
        }
    }
}

#[test]
fn number_range_3() {
    let rules: Vec<Box<dyn RuleTrait>> = vec![Box::new(NumberRange::all(1, 1))];
    let settings = Settings::builder(10).boxed_rules(rules.clone()).build();
    for _ in 0..MAX_TRIES {
        let random_result = random_numbers(&settings);
        assert_eq!(
            RandomResultType::Success,
            random_result.status(),
            "{:?}",
            random_result
        );
        assert_eq!(random_result.numbers().unwrap().len(), 10);
        for number in random_result.numbers().unwrap() {
            assert_eq!(*number, 1);
        }
    }
}

#[test]
fn number_range_4() {
    let rules: Vec<Box<dyn RuleTrait>> = vec![Box::new(NumberRange::all(100, 100))];
    let settings = Settings::builder(10).boxed_rules(rules.clone()).build();
    for _ in 0..MAX_TRIES {
        let random_result = random_numbers(&settings);
        assert_eq!(
            RandomResultType::Success,
            random_result.status(),
            "{:?}",
            random_result
        );
        assert_eq!(random_result.numbers().unwrap().len(), 10);
        for number in random_result.numbers().unwrap() {
            assert_eq!(*number, 100);
        }
    }
}

#[test]
fn number_range_5() {
    let rules: Vec<Box<dyn RuleTrait>> = vec![
        Box::new(NumberRange::all(1, 2)),
        Box::new(NoDuplicate {}),
    ];
    let settings = Settings::builder(2).boxed_rules(rules.clone()).build();
    for _ in 0..MAX_TRIES {
        let random_result = random_numbers(&settings);
        assert_eq!(
            RandomResultType::Success,
            random_result.status(),
            "{:?}",
            random_result
        );
        assert_eq!(random_result.numbers().unwrap().len(), 2);
        for number in random_result.numbers().unwrap() {
            assert!(*number == 1 || *number == 2);
        }
    }
}

#[test]
fn odd_even_1() {
    let data: Vec<(&str, OddEven, &[usize])> = vec![
        ("ODD:3,EVEN:2", OddEven::new(3, 2), &[1, 15, 10, 25, 8]),
        ("ODD:3,EVEN:3", OddEven::new(3, 3), &[4, 2, 10, 5, 11, 15]),
        ("ODD:2,EVEN:3", OddEven::new(2, 3), &[1, 9, 26, 6, 8]),
        (
            "ODD:5,EVEN:5",
            OddEven::new(5, 5),
            &[1, 2, 4, 5, 7, 21, 23, 8, 10, 14],
        ),
    ];
    for (main_idx, item) in data.iter().enumerate() {
        let odd_evens = [item.1, OddEven::from_numbers(item.2)];
        for (idx, odd_even) in odd_evens.iter().enumerate() {
            let current_data_numbers: Vec<usize> = item.2.to_vec();
        let current_data_settings: Settings = Settings::builder(item.2.len()).build();
        let current_data_shared_data: SharedData = SharedData::new();
        let current_data = CurrentData::new(&current_data_numbers, &current_data_settings, &current_data_shared_data);
            assert_eq!(
                item.0,
                odd_even.to_string(),
                "MAIN IDX: {}, FROM_NUMBERS: {}; ODD_EVEN: {:?}",
                main_idx,
                idx == 1,
                odd_even
            );
            assert_eq!(
                Ok(()),
                odd_even.is_within_range(&current_data),
                "MAIN IDX: {}, FROM_NUMBERS: {}; ODD_EVEN: {:?}",
                main_idx,
                idx == 1,
                odd_even
            );
            assert_eq!(
                Ok(()),
                odd_even.is_match(&current_data),
                "MAIN IDX: {}, FROM_NUMBERS: {}; ODD_EVEN: {:?}",
                main_idx,
                idx == 1,
                odd_even
            );
            let rand_result = random_numbers(&Settings::builder(item.2.len())
                .rule(*odd_even)
                .rule(NUMBER_RANGE.lock().unwrap().clone())
                .build());
            assert_eq!(
                RandomResultType::Success,
                rand_result.status(),
                "MAIN IDX: {}, FROM_NUMBERS: {}; ODD_EVEN: {:?} \n\n{:?}",
                main_idx,
                idx == 1,
                odd_even,
                rand_result
            );
            assert_eq!(
                Ok(()),
                OddEven::from_numbers(rand_result.numbers().unwrap()).is_match(&current_data),
                "MAIN IDX: {}, FROM_NUMBERS: {}; ODD_EVEN: {:?} \n\n{:?}",
                main_idx,
                idx == 1,
                odd_even,
                rand_result
            );
        }
    }
}

#[test]
fn odd_even_2() {
    let rules: Vec<Box<dyn RuleTrait>> =
        vec![Box::new(OddEven::new(5, 5)), Box::new(NUMBER_RANGE.lock().unwrap().clone())];
    let settings = Settings::builder(10).boxed_rules(rules.clone()).build();
    for _ in 0..MAX_TRIES {
        let random_result = random_numbers(&settings);
        assert_eq!(
            RandomResultType::Success,
            random_result.status(),
            "{:?}",
            random_result
        );
        assert_eq!(random_result.numbers().unwrap().len(), 10);
        let odd_even = OddEven::from_numbers(random_result.numbers().unwrap());
        let mut odd: usize = 0;
        let mut even: usize = 0;
        random_result.numbers().unwrap().iter().for_each(|x| {
            if x % 2_usize == 0 {
                even += 1
            } else {
                odd += 1
            }
        });
        assert_eq!(odd_even.odd(), 5);
        assert_eq!(odd_even.even(), 5);
        assert_eq!(odd, 5);
        assert_eq!(even, 5);
    }
}

#[test]
fn random_number() {
    let data: Vec<NumberRange> = vec![
        NumberRange::from_map(&[(&[0], 1, 1), (&[1], 99, 99), (&[2], 4, 4), (&[3], 6, 6), (&[4], 7, 7)]),
        NumberRange::from_map(&[(&[0], 20,20), (&[1], 55,55), (&[2], 4,10), (&[3], 71,75), (&[4], 47,47)]),
    ];
    for (main_idx, item) in data.iter().enumerate() {
        let rand_result = random_numbers(&Settings::builder(item.len()).rule(item.clone()).build());
        assert_eq!(
            RandomResultType::Success,
            rand_result.status(),
            "MAIN IDX: {} \n\n{:?}",
            main_idx,
            rand_result
        );
        assert_eq!(5, rand_result.attempts(), "MAIN IDX: {}",
            main_idx,
        );
        assert_eq!(
            Ok(()),
            item.is_match(&CurrentData::new(rand_result.numbers().unwrap(), &Settings::builder(item.len()).build(), &SharedData::new())),
            "MAIN IDX: {}, NUM_RANGE: {:?} \n\n{:?}",
            main_idx,
            item,
            rand_result
        );
    }
}

#[test]
fn number_pool_1() {
    let exclude_set: HashSet<usize> = HashSet::from_iter([23]);
    let all_set: HashSet<usize> = HashSet::from_iter([1, 4]);
    let some_set: HashSet<usize> = HashSet::from_iter([17, 18, 19, 20, 21]);
    let rules_1: Vec<Box<dyn RuleTrait>> = vec![
        Box::new(NumberPool::new(&[
            NumberPoolItem::new("exclude_set", &PoolType::set(exclude_set.clone()), OperatorType::Eq(0)),
            NumberPoolItem::new("all_set", &PoolType::set(all_set.clone()), OperatorType::Eq(all_set.len())),
            NumberPoolItem::new("some_set", &PoolType::set(some_set.clone()), OperatorType::Eq(3)),
        ])),
        Box::new(NUMBER_RANGE.lock().unwrap().clone()),
        Box::new(NoDuplicate),
    ];
    let settings_1 = Settings::builder(10).boxed_rules(rules_1.clone()).build();
    let rules_2: Vec<Box<dyn RuleTrait>> = vec![
        Box::new(NumberPool::new(&[
            NumberPoolItem::new("exclude_set", &PoolType::MinMax(23, 23), OperatorType::Eq(0)),
            NumberPoolItem::new("all_set", &PoolType::set(all_set.clone()), OperatorType::Eq(all_set.len())),
            NumberPoolItem::new("some_set", &PoolType::MinMax(17, 21), OperatorType::Eq(3)),
        ])),
        Box::new(NUMBER_RANGE.lock().unwrap().clone()),
        Box::new(NoDuplicate),
    ];
    let settings_2 = Settings::builder(10).boxed_rules(rules_2.clone()).build();
    for settings in [settings_1, settings_2] {
        for _ in 0..MAX_TRIES {
            let random_result = random_numbers(&settings);
            assert_eq!(
                RandomResultType::Success,
                random_result.status(),
                "{:?}",
                random_result
            );
            assert_eq!(random_result.numbers().unwrap().len(), 10);
            let numbers_set: HashSet<usize> = random_result.numbers().unwrap().iter().copied().collect();

            assert!(!numbers_set.contains(&23));
            assert_eq!(
                all_set
                    .intersection(&numbers_set)
                    .collect::<HashSet<&usize>>(),
                all_set.iter().collect()
            );
            assert_eq!(some_set.intersection(&numbers_set).count(), 3);
        }
    }
}

#[test]
fn number_pool_2() {
    let exclude_set: HashSet<usize> = (11..=100).collect();
    let all_set: HashSet<usize> = HashSet::from_iter([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let rules_1: Vec<Box<dyn RuleTrait>> = vec![
        Box::new(NumberPool::new(&[
            NumberPoolItem::new("exclude_set", &PoolType::set(exclude_set.clone()), OperatorType::Eq(0)),
            NumberPoolItem::new("all_set", &PoolType::set(all_set.clone()), OperatorType::Eq(all_set.len())),
        ])),
        Box::new(NUMBER_RANGE.lock().unwrap().clone()),
        Box::new(NoDuplicate),
    ];
    let settings_1 = Settings::builder(10).boxed_rules(rules_1.clone()).build();
    let rules_2: Vec<Box<dyn RuleTrait>> = vec![
        Box::new(NumberPool::new(&[
            NumberPoolItem::new("exclude_set", &PoolType::MinMax(11, 100), OperatorType::Eq(0)),
            NumberPoolItem::new("all_set", &PoolType::MinMax(1, 10), OperatorType::Eq(all_set.len())),
        ])),
        Box::new(NUMBER_RANGE.lock().unwrap().clone()),
        Box::new(NoDuplicate),
    ];
    let settings_2 = Settings::builder(10).boxed_rules(rules_2.clone()).build();
    for settings in [settings_1, settings_2] {
        for _ in 0..MAX_TRIES {
            let random_result = random_numbers(&settings);
            assert_eq!(
                RandomResultType::Success,
                random_result.status(),
                "{:?}",
                random_result
            );
            assert_eq!(random_result.numbers().unwrap().len(), 10);
            let numbers_set: HashSet<usize> = random_result.numbers().unwrap().iter().copied().collect();

            assert_eq!(
                all_set
                    .intersection(&numbers_set)
                    .collect::<HashSet<&usize>>(),
                all_set.iter().collect()
            );
        }
    }
}

#[test]
fn current_data_1() {
    let current_data_numbers_1: Vec<usize> = vec![5,4,3,2,1];
    let current_data_settings_1: Settings = Settings::builder(current_data_numbers_1.len()).build();
    let current_data_shared_data_1: SharedData = SharedData::new();
    let current_data_1 = CurrentData::new(&current_data_numbers_1, &current_data_settings_1, &current_data_shared_data_1);

    let current_data_numbers_2: Vec<usize> = vec![25,24,23,22,21];
    let current_data_settings_2: Settings = Settings::builder(current_data_numbers_2.len()).build();
    let current_data_shared_data_2: SharedData = SharedData::new();
    let current_data_2 = CurrentData::new(&current_data_numbers_2, &current_data_settings_2, &current_data_shared_data_2);

    let current_data_numbers_3: Vec<usize> = vec![35,35,33,32,31];
    let current_data_settings_3: Settings = Settings::builder(current_data_numbers_3.len()).build();
    let current_data_shared_data_3: SharedData = SharedData::new();
    let current_data_3 = CurrentData::new(&current_data_numbers_3, &current_data_settings_3, &current_data_shared_data_3);


    assert_eq!(current_data_1.selected_numbers().clone(), current_data_numbers_1);
    assert_eq!(current_data_1.selected_numbers_set().len(), current_data_numbers_1.len());
    assert!(current_data_numbers_1.iter().all(|x| current_data_1.selected_numbers_set().contains(x)), "expected:{:?} - actual:{:?}", current_data_numbers_1, current_data_1.selected_numbers_set());
    assert_eq!(current_data_1.selected_numbers_sorted().clone(), current_data_numbers_1.iter().copied().rev().collect::<Vec<usize>>());

    assert_eq!(current_data_2.selected_numbers().clone(), current_data_numbers_2);
    assert_eq!(current_data_2.selected_numbers_set().len(), current_data_numbers_2.len());
    assert!(current_data_numbers_2.iter().all(|x| current_data_2.selected_numbers_set().contains(x)), "expected:{:?} - actual:{:?}", current_data_numbers_2, current_data_2.selected_numbers_set());
    assert_eq!(current_data_2.selected_numbers_sorted().clone(), current_data_numbers_2.iter().copied().rev().collect::<Vec<usize>>());

    assert_eq!(current_data_3.selected_numbers().clone(), current_data_numbers_3);
    assert_eq!(current_data_3.selected_numbers_set().len(), current_data_numbers_3.len() - 1);
    assert!(current_data_numbers_3.iter().all(|x| current_data_3.selected_numbers_set().contains(x)), "expected:{:?} - actual:{:?}", current_data_numbers_3, current_data_3.selected_numbers_set());
    assert_eq!(current_data_3.selected_numbers_sorted().clone(), current_data_numbers_3.iter().copied().rev().collect::<Vec<usize>>());

    assert_ne!(current_data_1.selected_numbers_sorted().clone(), current_data_2.selected_numbers_sorted().clone());
    assert_ne!(current_data_2.selected_numbers_sorted().clone(), current_data_3.selected_numbers_sorted().clone());
    assert_ne!(current_data_1.selected_numbers_sorted().clone(), current_data_3.selected_numbers_sorted().clone());
    assert_eq!(format!("{:?}", current_data_1.selected_numbers_set().intersection(current_data_2.selected_numbers_set())), "[]");
    assert_eq!(format!("{:?}", current_data_2.selected_numbers_set().intersection(current_data_3.selected_numbers_set())), "[]");
    assert_eq!(format!("{:?}", current_data_1.selected_numbers_set().intersection(current_data_3.selected_numbers_set())), "[]");
}


#[test]
fn get_num_spaces_1() {
    let list: Vec<usize> = vec![4, 2, 1, 30, 10, 14, 12, 8, 5]; //1, 2, 4, 5, 8, 10, 12, 14, 30 
    let gaps = NumberSpaceItem::get_num_spaces(&list, false);
    assert_eq!(gaps, vec![1, 2, 1, 3, 2, 2, 2, 16], "{:?}", gaps);
}

#[test]
fn get_num_spaces_2() {
    let list: Vec<usize> = vec![1, 2, 4, 5, 8, 10, 12, 14, 30]; //1, 2, 4, 5, 8, 10, 12, 14, 30 
    let gaps = NumberSpaceItem::get_num_spaces(&list, true);
    assert_eq!(gaps, vec![1, 2, 1, 3, 2, 2, 2, 16], "{:?}", gaps);
}

#[test]
fn sequential_is_within_range() {
    let rule = Sequential::new(5, &[]);
    match rule.is_within_range(&CurrentData::new(&vec![5, 2, 7, 6/*20, 35, 4, 11, 12*/], &Settings::builder(5).build(), &SharedData::new())) {
        Ok(_) => panic!("Should Have Thrown Error."),
        Err(e) => assert_eq!("Expected Not: 5 and Seq_Counts: [].  Actual Not: 1 and Seq_Counts: [3].", e.1)
    }
}


#[test]
fn even_number_1() {
    let expected_rules = &Vec::new();
    let settings = &Settings::builder(10).build();
    let shared_data = &SharedData::new().with_min_max(1, 20);
    let current_data = &CurrentData::new(expected_rules, settings, shared_data);
    for _ in 0..2000 {
        let even_number = current_data.get_rand(NumberType::Even,None, None).unwrap();
        assert!(even_number.is_multiple_of(2), "{}", even_number);
        assert!((1..=20).contains(&even_number), "{}", even_number);
    }
}

#[test]
fn odd_number_1() {
    let expected_rules = &Vec::new();
    let settings = &Settings::builder(10).build();
    let shared_data = &SharedData::new().with_min_max(1, 20);
    let current_data = &CurrentData::new(expected_rules, settings, shared_data);
    for _ in 0..2000 {
        let odd_number = current_data.get_rand(NumberType::Odd,None, None).unwrap();
        assert!(odd_number % 2 == 1, "{}", odd_number);
        assert!((1..=20).contains(&odd_number), "{}", odd_number);
    }
}

#[test]
fn random_trait_get_number_1() {
    let mut set = HashSet::new();
    let min = 1;
    let max = 200;
    for _ in 0..20000 {
        let number = DefaultRandom{}.get_number(min, max);
        set.insert(number);
        assert!(number >= min && number <= max, "{}", number);
    }
    assert!(set.len() == max);    
}

#[test]
fn random_trait_get_bool_1() {
    let mut set = HashSet::new();
    for _ in 0..100 {
        let bool_value = DefaultRandom{}.get_bool();
        set.insert(bool_value);
    }
    assert!(set.len() == 2);    
}

#[test]
fn number_space_item_from_number_1() {
    //Create 10 random numbers between 1 and 20 (min and max numbers are inclusive)
    let number_space = NumberSpaceItem::from_numbers(&[
        (OperatorType::Between(1, 3), None),
        (OperatorType::Between(4, 6), None),
        (OperatorType::Between(7, 9), None),

    ], &[25, 33, 31, 26, 18], false);
    assert!("[NumberSpaceItem { number_space_type: Between(1, 3), needs: [1, 1], has: [1, 1] }, NumberSpaceItem { number_space_type: Between(4, 6), needs: [1], has: [1] }, NumberSpaceItem { number_space_type: Between(7, 9), needs: [1], has: [1] }]" == format!("{:?}", number_space), "{:?}", number_space);
}

#[test]
fn number_space_item_from_number_2() {
    //Create 10 random numbers between 1 and 20 (min and max numbers are inclusive)
    let number_space = NumberSpaceItem::from_numbers(&[
        (OperatorType::Between(4, 6), None),

    ], &[7, 12, 16, 20, 25], false);
    assert!("[NumberSpaceItem { number_space_type: Between(4, 6), needs: [4], has: [4] }]" == format!("{:?}", number_space), "{:?}", number_space);
}

#[test]
fn number_space_item_from_number_3() {
    //Create 10 random numbers between 1 and 20 (min and max numbers are inclusive)
    let number_space = NumberSpaceItem::from_numbers(&[
        (OperatorType::Between(1, 3), None),
        (OperatorType::Between(19, 21), None),
        (OperatorType::Between(7, 9), None),
    ], &[31, 22, 24, 3, 39], false);
    assert!("[NumberSpaceItem { number_space_type: Between(1, 3), needs: [1], has: [1] }, NumberSpaceItem { number_space_type: Between(19, 21), needs: [1], has: [1] }, NumberSpaceItem { number_space_type: Between(7, 9), needs: [2], has: [2] }]" == format!("{:?}", number_space), "{:?}", number_space);
}


#[test]
fn settings_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Settings>();
}

#[test]
fn number_space_item_has_too_many() {
    let lte = OperatorType::Lte(3);
    //longest run of 5 is more than the longest needed run of 3
    assert!(NumberSpaceItem::new_full(&lte, &[2, 3], &[1, 5]).has_too_many());
    //more runs than needed
    assert!(NumberSpaceItem::new_full(&lte, &[2], &[1, 1]).has_too_many());
    assert!(!NumberSpaceItem::new_full(&lte, &[3, 2], &[1, 3]).has_too_many());
    assert!(!NumberSpaceItem::new_full(&lte, &[2, 3], &[2]).has_too_many());
    assert!(!NumberSpaceItem::new_full(&lte, &[2, 3], &[]).has_too_many());
}

#[test]
fn number_pool_counts_duplicates() {
    let digits = PoolType::MinMax(48, 57);
    let rule = NumberPool::new(&[NumberPoolItem::new("digits", &digits, OperatorType::Eq(2))]);
    let settings = Settings::builder(4).build();
    let shared_data = SharedData::new();
    //48 is selected three times, so there are three numbers from the pool, not one
    let numbers = vec![48, 48, 48, 70];
    assert!(rule.is_match(&CurrentData::new(&numbers, &settings, &shared_data)).is_err());
    assert!(rule.is_within_range(&CurrentData::new(&numbers, &settings, &shared_data)).is_err());
    let numbers = vec![48, 48, 70, 71];
    assert!(rule.is_match(&CurrentData::new(&numbers, &settings, &shared_data)).is_ok());

    for _ in 0..MAX_TRIES {
        let random_result = random_numbers(&Settings::builder(8)
            .rule(NumberRange::all(40, 70))
            .rule(NumberPool::new(&[NumberPoolItem::new("digits", &digits, OperatorType::Eq(2))]))
            .build());
        let numbers = random_result.numbers().unwrap();
        assert_eq!(numbers.iter().filter(|x| digits.contains(**x)).count(), 2, "{:?}", numbers);
    }
}

#[test]
fn sequential_seq_counts_any_order() {
    for seq_counts in [[2, 3], [3, 2]] {
        for _ in 0..100 {
            let random_result = random_numbers(&Settings::builder(10)
                .rule(NoDuplicate)
                .rule(NumberRange::all(1, 100))
                .rule(Sequential::new(5, &seq_counts))
                .build());
            assert_eq!(RandomResultType::Success, random_result.status(), "{:?}", random_result);
        }
    }
}

#[test]
fn number_space_single_value_spaces() {
    //Lte(1), Between(4, 4) and Eq(3) only allow one possible higher number
    for operator_type in [OperatorType::Lte(1), OperatorType::Between(4, 4), OperatorType::Eq(3)] {
        for _ in 0..100 {
            let random_result = random_numbers(&Settings::builder(10)
                .rule(NumberRange::all(1, 100))
                .rule(NumberSpace::new(&[NumberSpaceItem::new(&operator_type, 9)]))
                .build());
            assert_eq!(RandomResultType::Success, random_result.status(), "{:?}", random_result);
            let mut sorted_numbers = random_result.numbers().unwrap().to_vec();
            sorted_numbers.sort_unstable();
            assert!(sorted_numbers.windows(2).all(|x| operator_type.is_match(x[1] - x[0])), "{:?}", sorted_numbers);
        }
    }
}

#[test]
fn operator_type_get_number() {
    let settings = Settings::builder(10).build();
    let shared_data = SharedData::new().with_min_max(1, 100);

    //the highest selected number is the base and is not returned again
    let selected = vec![10, 40];
    let numbers = OperatorType::Lte(3).get_number(&CurrentData::new(&selected, &settings, &shared_data), 3);
    assert_eq!(numbers.len(), 3);
    assert!(numbers[0] > 40 && numbers[0] <= 43, "{:?}", numbers);
    assert!(numbers.windows(2).all(|x| x[1] > x[0] && x[1] - x[0] <= 3), "{:?}", numbers);

    //nothing selected, so the starting number is returned with the three spaces
    let selected = vec![];
    let numbers = OperatorType::Between(2, 2).get_number(&CurrentData::new(&selected, &settings, &shared_data), 3);
    assert!(numbers.len() <= 4);
    assert!(numbers.windows(2).all(|x| x[1] - x[0] == 2), "{:?}", numbers);

    //never goes past max
    let selected = vec![99];
    let current_data = CurrentData::new(&selected, &settings, &shared_data);
    assert_eq!(OperatorType::Between(1, 5).get_number(&current_data, 1), vec![100]);
    assert_eq!(OperatorType::Eq(1).get_number(&current_data, 1), vec![100]);
    assert_eq!(OperatorType::Eq(2).get_number(&current_data, 1), Vec::<usize>::new());
    assert_eq!(OperatorType::Gt(1).get_number(&current_data, 1), Vec::<usize>::new());
}

#[test]
fn number_space_needs_any_order() {
    let lte = OperatorType::Lte(3);
    assert_eq!(NumberSpaceItem::new_full(&lte, &[3, 2], &[]).needs(), &vec![2, 3]);
}

#[test]
fn string_with_invalid_character() {
    //0xD800 to 0xDFFF are surrogates, which are not valid characters
    let random_result = random_numbers(&Settings::builder(3).rule(NumberRange::all(0xD800, 0xDFFF)).build());
    assert_eq!(RandomResultType::Success, random_result.status());
    assert!(random_result.string().is_err());
    assert!(random_result.shuffled_string().is_err());

    //no number range, so the numbers are almost always too large to be characters
    let random_result = random_numbers(&Settings::builder(20).build());
    assert!(random_result.string().is_err());

    let random_result = random_numbers(&Settings::builder(3).rule(NumberRange::all(97, 122)).build());
    assert_eq!(random_result.string().unwrap().len(), 3);
}

#[test]
fn check_count_must_equal_count() {
    assert!(OddEven::new(5, 5).check_count(10).is_ok());
    assert!(OddEven::new(1, 1).check_count(5).is_err());
    assert!(OddEven::new(5, 5).check_count(9).is_err());
    assert!(Sequential::new(1, &[2, 2]).check_count(5).is_ok());
    assert!(Sequential::new(1, &[2]).check_count(5).is_err());
    assert!(Sequential::new(5, &[2]).check_count(5).is_err());
}

#[test]
fn settings_odd_even_count_mismatch() {
    let settings = Settings::builder(5).rule(OddEven::new(1, 1)).build();
    assert_eq!(settings.errors().len(), 1);
    assert!(settings.errors()[0].contains("must add up to count"), "{:?}", settings.errors());
    let random_result = random_numbers(&settings);
    assert_eq!(RandomResultType::BadRequest, random_result.status());
    assert_eq!(random_result.attempts(), 0);
    assert!(random_result.numbers().is_err());
    assert!(random_result.logs()[0].to_string().contains("must add up to count"));
}

#[test]
fn no_duplicate_full_range() {
    //every number from 1 to 50 has to be picked, so the last picks only have a few numbers left to choose from
    for _ in 0..50 {
        let random_result = random_numbers(&Settings::builder(50)
            .rule(NoDuplicate)
            .rule(NumberRange::all(1, 50))
            .build());
        assert_eq!(RandomResultType::Success, random_result.status());
        assert_eq!(random_result.numbers().unwrap().iter().collect::<HashSet<_>>().len(), 50);
    }
}

#[test]
fn get_rand_number_not_found() {
    let settings = Settings::builder(10).build();
    let selected = vec![];
    //11 is the only number in range and it is odd
    let shared_data = SharedData::new().with_min_max(11, 11);
    let current_data = CurrentData::new(&selected, &settings, &shared_data);
    assert_eq!(current_data.get_rand(NumberType::Even, None, None), None);
    assert_eq!(current_data.get_rand(NumberType::Odd, None, None), Some(11));

    //min is greater than max
    assert_eq!(current_data.get_rand(NumberType::Any, None, Some((20, 1))), None);
    assert_eq!(current_data.get_rand(NumberType::Any, Some(&PoolType::MinMax(20, 1)), None), None);
    assert_eq!(current_data.get_rand(NumberType::Any, Some(&PoolType::set([])), None), None);
}

#[test]
fn random_numbers_number_not_found() {
    //every number from 1 to 10 is excluded, so there is nothing left to pick
    let random_result = random_numbers(&Settings::builder(3)
        .rule(NumberRange::all(1, 10))
        .rule(NumberPool::new(&[NumberPoolItem::new("exclude", &PoolType::MinMax(1, 10), OperatorType::Eq(0))]))
        .build());
    assert_eq!(RandomResultType::Failed, random_result.status());
    assert!(random_result.logs().iter().any(|x| x.to_string().contains(NO_NUMBER_FOUND_ERR)));
}

#[test]
fn empty_and_large_values() {
    let random = DefaultRandom{};
    let empty: Vec<usize> = Vec::new();
    assert_eq!(get_random_vec_item(&random, &empty), None);
    assert_eq!(get_random_vec_item(&random, &[7]), Some(&7));

    assert_eq!(PoolType::MinMax(0, usize::MAX).len(), usize::MAX);
    assert_eq!(PoolType::MinMax(20, 1).len(), 0);
    assert!(PoolType::MinMax(20, 1).is_empty());
    assert_eq!(PoolType::MinMax(5, 5).len(), 1);

    //no number range, so the numbers go up to usize::MAX
    let settings = Settings::builder(3).rule(Sequential::new(0, &[3])).build();
    let selected = vec![usize::MAX - 1];
    let shared_data = SharedData::new();
    for _ in 0..50 {
        //extending usize::MAX - 1 into a sequence of 3 does not fit, starting a new sequence does
        match Sequential::new(0, &[3]).get_numbers(&CurrentData::new(&selected, &settings, &shared_data)) {
            Ok(numbers) => assert_eq!(numbers.unwrap().len(), 3),
            Err(e) => assert!(e.contains("does not fit"), "{}", e)
        }
    }
}

#[test]
fn count_zero() {
    let random_result = random_numbers(&Settings::builder(0).rule(NumberRange::all(1, 20)).build());
    assert_eq!(RandomResultType::Success, random_result.status());
    assert!(random_result.numbers().unwrap().is_empty());
    assert_eq!(random_result.shuffled_string().unwrap(), "");
}

#[test]
fn number_range_min_greater_than_max() {
    let settings = Settings::builder(3).rule(NumberRange::all(20, 1)).build();
    assert_eq!(settings.errors(), ["NumberRange min: 20 is greater than max: 1"]);
    assert_eq!(RandomResultType::BadRequest, random_numbers(&settings).status());
}

#[test]
fn settings_with_more_than_one_error() {
    let settings = Settings::builder(5)
        .rule(NumberRange::all(20, 1))
        .rule(OddEven::new(1, 1))
        .rule(Sequential::new(1, &[2]))
        .build();
    assert_eq!(settings.errors().len(), 3);
    let random_result = random_numbers(&settings);
    assert_eq!(RandomResultType::BadRequest, random_result.status());
    assert_eq!(random_result.logs().len(), 3);

    assert!(Settings::builder(5).rule(NumberRange::all(1, 20)).build().errors().is_empty());
}

#[test]
fn number_pool_alphanumeric_count_too_small() {
    for (count, include_special_char) in [(1, false), (2, true)] {
        let settings = Settings::builder(count).rule(NumberPool::alphanumeric(count, include_special_char)).build();
        assert_eq!(settings.errors().len(), 1);
        assert!(settings.errors()[0].contains("or more"), "{:?}", settings.errors());
        assert_eq!(RandomResultType::BadRequest, random_numbers(&settings).status());
    }
    for (count, include_special_char) in [(2, false), (3, true)] {
        let random_result = random_numbers(&Settings::builder(count).rule(NumberPool::alphanumeric(count, include_special_char)).build());
        assert_eq!(random_result.shuffled_string().unwrap().len(), count);
    }
}

#[test]
fn number_range_min_greater_than_max_by_index() {
    assert!(NumberRange::from_map(&[(&[0, 1], 100, 999), (&[2], 1000, 9999)]).check_count(3).is_ok());
    assert!(NumberRange::from_map(&[(&[0, 1], 100, 999), (&[2], 9999, 1000)]).check_count(3).is_err());
}

#[test]
fn sequential_more_than_one_sequence() {
    //(min, max, not, seq_counts)
    let data: Vec<(usize, usize, usize, Vec<usize>)> = vec![
        (1, 10, 1, vec![2, 2]),
        (1, 10, 0, vec![2, 3]),
        (1, 100, 0, vec![2, 2, 2, 2]),
        (1, 20, 2, vec![3, 3]),
        (1, 30, 4, vec![2, 3, 4]),
    ];
    for (min, max, not, seq_counts) in data {
        let count = not + seq_counts.iter().sum::<usize>();
        for _ in 0..200 {
            let random_result = random_numbers(&Settings::builder(count)
                .rule(NoDuplicate)
                .rule(NumberRange::all(min, max))
                .rule(Sequential::new(not, &seq_counts))
                .build());
            assert_eq!(RandomResultType::Success, random_result.status(), "{:?}", random_result);
            let numbers = random_result.numbers().unwrap();
            assert!(numbers.iter().all(|x| *x >= min && *x <= max), "{:?}", numbers);
            let actual = Sequential::from_numbers(&CurrentData::new(numbers, &Settings::builder(count).build(), &SharedData::new()), false);
            assert_eq!(actual.not(), not, "{:?}", numbers);
            assert_eq!(actual.seq_counts(), &seq_counts, "{:?}", numbers);
        }
    }
}

#[test]
fn sequential_is_within_range_sequence_too_long() {
    let rule = Sequential::new(1, &[2, 2]);
    let settings = Settings::builder(5).build();
    let shared_data = SharedData::new();
    //a sequence of 3 can never become a sequence of 2
    let numbers = vec![5, 6, 7];
    assert!(rule.is_within_range(&CurrentData::new(&numbers, &settings, &shared_data)).is_err());
    let numbers = vec![5, 6];
    assert!(rule.is_within_range(&CurrentData::new(&numbers, &settings, &shared_data)).is_ok());
    let numbers = vec![5, 6, 9];
    assert!(rule.is_within_range(&CurrentData::new(&numbers, &settings, &shared_data)).is_ok());
}

#[test]
fn number_space_item_missing() {
    let lte = OperatorType::Lte(3);
    //the run of 3 that is already there is the run of 3 that is needed, so the run of 2 is missing
    let item = NumberSpaceItem::new_full(&lte, &[2, 3], &[3]);
    assert_eq!(item.missing(), vec![2, 0]);
    assert_eq!(item.process_missing_count(), 2);

    let item = NumberSpaceItem::new_full(&lte, &[1, 4], &[1]);
    assert_eq!(item.missing(), vec![1, 3]);
    assert_eq!(item.process_missing_count(), 3);

    let item = NumberSpaceItem::new_full(&lte, &[2, 3], &[]);
    assert_eq!(item.missing(), vec![2, 3]);
    assert_eq!(item.process_missing_count(), 3);

    let item = NumberSpaceItem::new_full(&lte, &[2, 3], &[2, 3]);
    assert_eq!(item.missing(), vec![0, 0]);
    assert_eq!(item.process_missing_count(), 0);
}

#[test]
fn pool_type_set() {
    let pool = PoolType::set([21, 4, 1, 4, 17]);
    //sorted with no duplicates
    match &pool {
        PoolType::Set(set) => assert_eq!(set.numbers(), &[1, 4, 17, 21]),
        _ => panic!("{:?}", pool)
    }
    assert_eq!(pool.len(), 4);
    assert!(pool.contains(17));
    assert!(!pool.contains(18));
    assert_eq!(pool.has(&[4, 4, 5, 21]), 3);
    assert_eq!(pool.difference(&HashSet::from_iter([4, 99])), vec![1, 17, 21]);
    assert!(PoolType::set([]).is_empty());

    //the other ways to create a set give the same numbers
    let from_chars = PoolType::from_chars(&['b', 'a', 'b']);
    let from_hash_set = PoolType::new(&HashSet::from_iter(['a', 'b']));
    let hash_set: HashSet<usize> = HashSet::from_iter([97, 98]);
    let from_set = PoolType::set(hash_set);
    for pool in [from_chars, from_hash_set, from_set] {
        match &pool {
            PoolType::Set(set) => assert_eq!(set.numbers(), &[97, 98]),
            _ => panic!("{:?}", pool)
        }
    }
}

#[test]
fn number_space_item_process_missing() {
    let eq = OperatorType::Eq(3);
    //nothing is there yet, so the longest run is a new run
    assert_eq!(NumberSpaceItem::new_full(&eq, &[2, 2, 3], &[]).process_missing(), Some((3, true)));
    //the run of 3 is there, so the next run is a new run of 2
    assert_eq!(NumberSpaceItem::new_full(&eq, &[2, 2, 3], &[3]).process_missing(), Some((2, true)));
    //the run of 1 is the start of the run of 3, so the rest of it is not a new run
    assert_eq!(NumberSpaceItem::new_full(&eq, &[2, 2, 3], &[1]).process_missing(), Some((2, false)));
    assert_eq!(NumberSpaceItem::new_full(&eq, &[2, 2, 3], &[2, 2, 3]).process_missing(), None);
    assert_eq!(NumberSpaceItem::new_full(&eq, &[2, 2, 3], &[2, 2, 3]).process_missing_count(), 0);
}

#[test]
fn operator_type_get_new_run() {
    let settings = Settings::builder(10).build();
    let shared_data = SharedData::new().with_min_max(1, 100);
    let selected = vec![10, 40, 98];
    let current_data = CurrentData::new(&selected, &settings, &shared_data);
    for _ in 0..200 {
        //a new run does not continue from 98, so there is always room for it
        let numbers = OperatorType::Eq(3).get_new_run(&current_data, 2);
        assert_eq!(numbers.len(), 3, "{:?}", numbers);
        assert!(numbers.windows(2).all(|x| x[1] - x[0] == 3), "{:?}", numbers);
        assert!(numbers.iter().all(|x| *x >= 1 && *x <= 100), "{:?}", numbers);
    }
    //continuing from 98 does not fit
    assert!(OperatorType::Eq(3).get_number(&current_data, 2).is_empty());
}

#[test]
fn number_space_more_than_one_run() {
    for operator_type in [OperatorType::Eq(3), OperatorType::Eq(1), OperatorType::Lte(3), OperatorType::Between(3, 6)] {
        let mut success_count = 0;
        for _ in 0..100 {
            let random_result = random_numbers(&Settings::builder(12)
                .rule(NumberRange::all(1, 100))
                .rule(NumberSpace::new(&[NumberSpaceItem::new_full(&operator_type, &[2, 2, 3], &[])]))
                .build());
            if let Ok(numbers) = random_result.numbers() {
                success_count += 1;
                let mut sorted_numbers = numbers.to_vec();
                sorted_numbers.sort_unstable();
                let spaces: Vec<usize> = sorted_numbers.windows(2).map(|x| x[1] - x[0]).collect();
                assert_eq!(operator_type.has(&spaces), vec![2, 2, 3], "{:?}", sorted_numbers);
            }
        }
        assert!(success_count >= 95, "{:?} {}", operator_type, success_count);
    }
}

#[test]
fn shared_data_merge() {
    let mut shared_data = SharedData::new();
    assert_eq!(shared_data.min_max(), (usize::MIN, usize::MAX));
    assert!(!shared_data.no_duplicates());
    assert!(shared_data.exclusion_pools().is_empty());

    shared_data.merge(SharedData::new().with_min_max(1, 20)).unwrap();
    shared_data.merge(SharedData::new().with_no_duplicates()).unwrap();
    shared_data.merge(SharedData::new().with_exclusion_pools(vec![PoolType::MinMax(5, 6)])).unwrap();
    shared_data.merge(SharedData::new().with_exclusion_pools(vec![PoolType::set([9])])).unwrap();
    assert_eq!(shared_data.min_max(), (1, 20));
    assert!(shared_data.no_duplicates());
    assert_eq!(shared_data.exclusion_pools().len(), 2);

    //the last min and max shared are the ones used, and everything else is kept
    shared_data.merge(SharedData::new().with_min_max(3, 8)).unwrap();
    assert_eq!(shared_data.min_max(), (3, 8));
    assert!(shared_data.no_duplicates());
    assert_eq!(shared_data.exclusion_pools().len(), 2);
}

#[test]
fn operator_type_lone_numbers() {
    let eq = OperatorType::Eq(3);
    assert_eq!(eq.lone_numbers(&[]), (0, false));
    assert_eq!(eq.lone_numbers(&[10]), (1, true));
    //10, 13 and 16 are a run.  40 and 90 are not next to a space of 3.
    assert_eq!(eq.lone_numbers(&[10, 13, 16, 40, 90]), (2, true));
    assert_eq!(eq.lone_numbers(&[5, 10, 13, 16]), (1, false));
    //every space of 3 or more matches Gte(3), so only 11 is not part of a run
    assert_eq!(OperatorType::Gte(3).lone_numbers(&[10, 11, 40, 90]), (1, false));
}

#[test]
fn number_space_takes_more_numbers_than_count() {
    //3 runs of 2 spaces take 9 numbers
    let number_space = NumberSpace::new(&[NumberSpaceItem::new_full(&OperatorType::Eq(3), &[2, 2, 2], &[])]);
    assert!(number_space.check_count(9).is_ok());
    assert!(number_space.check_count(8).is_err());

    let settings = Settings::builder(8).rule(NumberRange::all(1, 100)).rule(number_space).build();
    assert_eq!(settings.errors().len(), 1);
    assert_eq!(RandomResultType::BadRequest, random_numbers(&settings).status());
}

#[test]
fn number_space_is_within_range_lone_numbers() {
    let shared_data = SharedData::new();
    let rule = NumberSpace::new(&[NumberSpaceItem::new_full(&OperatorType::Eq(3), &[2, 2, 2], &[])]);

    //count 9 has no spare numbers, so 50 can not be there unless a run can continue from it
    let settings = Settings::builder(9).build();
    let numbers = vec![50, 60, 63, 66];
    assert!(rule.is_within_range(&CurrentData::new(&numbers, &settings, &shared_data)).is_err());
    let numbers = vec![10, 13, 16, 50];
    assert!(rule.is_within_range(&CurrentData::new(&numbers, &settings, &shared_data)).is_ok());

    //count 10 has one spare number
    let settings = Settings::builder(10).build();
    let numbers = vec![50, 60, 63, 66];
    assert!(rule.is_within_range(&CurrentData::new(&numbers, &settings, &shared_data)).is_ok());
    let numbers = vec![40, 50, 60, 63, 66];
    assert!(rule.is_within_range(&CurrentData::new(&numbers, &settings, &shared_data)).is_err());
}

#[test]
fn number_space_no_spare_numbers() {
    //(operator type, runs, count).  Every number has to be part of a run, or right next to one for Gte and Gt.
    let data: Vec<(OperatorType, Vec<usize>, usize)> = vec![
        (OperatorType::Eq(3), vec![2, 2, 2], 9),
        (OperatorType::Eq(1), vec![2, 2, 2], 9),
        (OperatorType::Lt(3), vec![2, 2, 2], 9),
        (OperatorType::Between(3, 6), vec![2, 2, 2], 9),
        (OperatorType::Gte(3), vec![2, 2, 2], 9),
        (OperatorType::Gt(3), vec![2, 2, 2], 9),
        (OperatorType::Gte(3), vec![1, 1, 1, 1], 10),
        (OperatorType::Gt(3), vec![1, 1, 1, 1], 10),
        (OperatorType::Gte(3), vec![4], 10),
    ];
    for (operator_type, runs, count) in data {
        let mut success_count = 0;
        for _ in 0..50 {
            let random_result = random_numbers(&Settings::builder(count)
                .rule(NoDuplicate)
                .rule(NumberRange::all(1, 100))
                .rule(NumberSpace::new(&[NumberSpaceItem::new_full(&operator_type, &runs, &[])]))
                .build());
            if let Ok(numbers) = random_result.numbers() {
                success_count += 1;
                let mut sorted_numbers = numbers.to_vec();
                sorted_numbers.sort_unstable();
                let spaces: Vec<usize> = sorted_numbers.windows(2).map(|x| x[1] - x[0]).collect();
                assert_eq!(operator_type.has(&spaces), runs, "{:?}", sorted_numbers);
            }
        }
        assert!(success_count >= 48, "{:?} {:?} {}", operator_type, runs, success_count);
    }
}

#[test]
fn shared_data_custom() {
    let mut shared_data = SharedData::new();
    assert_eq!(shared_data.custom("multiple"), None);

    shared_data.merge(SharedData::new()
        .with_custom("multiple", CustomValue::Usize(5))
        .with_custom("label", CustomValue::String("abc".to_owned()))).unwrap();
    shared_data.merge(SharedData::new().with_custom("flag", CustomValue::Bool(true))).unwrap();
    assert_eq!(shared_data.custom("multiple"), Some(&CustomValue::Usize(5)));
    assert_eq!(shared_data.custom("label"), Some(&CustomValue::String("abc".to_owned())));
    assert_eq!(shared_data.custom("flag"), Some(&CustomValue::Bool(true)));
    assert_eq!(shared_data.custom_keys().count(), 3);

    //"multiple" is already there, so nothing from this one is added
    let other = SharedData::new().with_min_max(1, 9).with_custom("multiple", CustomValue::Usize(7)).with_custom("new", CustomValue::Bool(false));
    assert_eq!(shared_data.merge(other), Err("multiple".to_owned()));
    assert_eq!(shared_data.custom("multiple"), Some(&CustomValue::Usize(5)));
    assert_eq!(shared_data.custom("new"), None);
    assert_eq!(shared_data.min_max(), (usize::MIN, usize::MAX));
}

//Shares a custom value.  It does not get any numbers.
#[derive(Clone, Debug)]
struct ShareMultiple {
    name: String,
    multiple: usize,
}

impl std::fmt::Display for ShareMultiple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.name, self.multiple)
    }
}

impl RuleTrait for ShareMultiple {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn share_data(&self, _current_data: &CurrentData) -> Option<SharedData> {
        return Some(SharedData::new().with_custom("multiple", CustomValue::Usize(self.multiple)));
    }

    fn get_numbers(&self, _current_data: &CurrentData) -> std::result::Result<Option<Vec<usize>>, String> {
        return Ok(None);
    }

    fn is_within_range(&self, _current_data: &CurrentData) -> std::result::Result<(), (IsWithinErrorType, String)> {
        return Ok(());
    }

    fn is_match(&self, _current_data: &CurrentData) -> std::result::Result<(), String> {
        return Ok(());
    }

    fn check_count(&self, _count: usize) -> std::result::Result<bool, String> {
        return Ok(true);
    }

    fn name(&self) -> String {
        return self.name.to_owned();
    }
}

//Only allows numbers that are a multiple of the custom value that another rule shared
#[derive(Clone, Debug)]
struct MultipleOfShared {}

impl MultipleOfShared {
    fn multiple(current_data: &CurrentData) -> usize {
        return match current_data.shared_data().custom("multiple") {
            Some(CustomValue::Usize(v)) => *v,
            _ => 1
        };
    }
}

impl std::fmt::Display for MultipleOfShared {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MultipleOfShared")
    }
}

impl RuleTrait for MultipleOfShared {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn share_data(&self, _current_data: &CurrentData) -> Option<SharedData> {
        return None;
    }

    fn get_numbers(&self, current_data: &CurrentData) -> std::result::Result<Option<Vec<usize>>, String> {
        let multiple = MultipleOfShared::multiple(current_data);
        let number = current_data.get_rand(NumberType::Any, None, None).ok_or_else(|| current_data.no_number_err())?;
        return Ok(Some(vec![number - (number % multiple)]));
    }

    fn is_within_range(&self, current_data: &CurrentData) -> std::result::Result<(), (IsWithinErrorType, String)> {
        let multiple = MultipleOfShared::multiple(current_data);
        if current_data.selected_numbers().iter().any(|x| !x.is_multiple_of(multiple)) {
            return Err((IsWithinErrorType::MakePriority, format!("Not every number is a multiple of {}", multiple)));
        }
        return Ok(());
    }

    fn is_match(&self, current_data: &CurrentData) -> std::result::Result<(), String> {
        return self.is_within_range(current_data).map_err(|x| x.1);
    }

    fn check_count(&self, _count: usize) -> std::result::Result<bool, String> {
        return Ok(true);
    }

    fn name(&self) -> String {
        return String::from("MultipleOfShared");
    }
}

#[test]
fn custom_rules_share_custom_value() {
    for _ in 0..50 {
        let random_result = random_numbers(&Settings::builder(6)
            .rule(NumberRange::all(10, 100))
            .rule(ShareMultiple { name: "ShareMultiple".to_owned(), multiple: 5 })
            .rule(MultipleOfShared {})
            .build());
        assert_eq!(RandomResultType::Success, random_result.status(), "{:?}", random_result);
        let numbers = random_result.numbers().unwrap();
        assert_eq!(numbers.len(), 6);
        assert!(numbers.iter().all(|x| x.is_multiple_of(5) && *x >= 10 && *x <= 100), "{:?}", numbers);
    }
}

#[test]
fn custom_rules_set_same_custom_key() {
    let random_result = random_numbers(&Settings::builder(6)
        .rule(NumberRange::all(10, 100))
        .rule(ShareMultiple { name: "RuleA".to_owned(), multiple: 5 })
        .rule(ShareMultiple { name: "RuleB".to_owned(), multiple: 7 })
        .rule(MultipleOfShared {})
        .build());
    assert_eq!(RandomResultType::BadRequest, random_result.status());
    assert_eq!(random_result.attempts(), 1);
    assert!(random_result.numbers().is_err());
    let error = random_result.logs().last().unwrap().to_string();
    assert!(error.contains("Custom shared data key \"multiple\" was set by both"), "{}", error);
    assert!(error.contains("RuleA") && error.contains("RuleB"), "{}", error);
}

#[test]
fn number_space_not_possible_with_no_duplicate() {
    let settings = |operator_type: OperatorType, runs: &[usize], count: usize, no_duplicate: bool| -> Settings {
        let mut rules: Vec<Box<dyn RuleTrait>> = vec![
            Box::new(NumberRange::all(1, 100)),
            Box::new(NumberSpace::new(&[NumberSpaceItem::new_full(&operator_type, runs, &[])])),
        ];
        if no_duplicate {
            rules.push(Box::new(NoDuplicate));
        }
        return Settings::builder(count).boxed_rules(rules.clone()).build();
    };

    //the space between any two different numbers matches Gte(1), Gte(0) and Gt(0), so with no duplicates there is
    //always one run of 7 for count 8
    for operator_type in [OperatorType::Gte(1), OperatorType::Gte(0), OperatorType::Gt(0)] {
        for runs in [vec![2, 2], vec![4], vec![6]] {
            let settings = settings(operator_type, &runs, 8, true);
            assert_eq!(settings.errors().len(), 1, "{:?} {:?}", operator_type, runs);
            assert!(settings.errors()[0].contains("is not possible with NoDuplicate"), "{:?}", settings.errors());
            let random_result = random_numbers(&settings);
            assert_eq!(RandomResultType::BadRequest, random_result.status());
            assert_eq!(random_result.attempts(), 0);
        }
    }

    //one run of 7 is possible
    let settings_1 = settings(OperatorType::Gte(1), &[7], 8, true);
    assert!(settings_1.errors().is_empty());
    assert_eq!(RandomResultType::Success, random_numbers(&settings_1).status());

    //possible when duplicates are allowed, and for types that do not match every space
    assert!(settings(OperatorType::Gte(1), &[2, 2], 8, false).errors().is_empty());
    assert!(settings(OperatorType::Gte(2), &[2, 2], 8, true).errors().is_empty());
    assert!(settings(OperatorType::Gt(1), &[2, 2], 8, true).errors().is_empty());
    assert!(settings(OperatorType::Lte(3), &[2, 2], 8, true).errors().is_empty());
}

#[test]
fn settings_builder() {
    //rules, exclude rules and the random number generator are all set on the builder
    let build = |seed: u64| -> Settings {
        return Settings::builder(10)
            .rule(NoDuplicate)
            .rule(NumberRange::all(1, 100))
            .rule(OddEven::new(5, 5))
            .exclude(NumberRange::all(20, 29))
            .exclude(NumberRange::all(51, 75))
            .random(crate::seeded_random::SeededRandom::new(seed))
            .build();
    };
    let settings = build(7);
    assert!(settings.errors().is_empty());
    assert_eq!(settings.count(), 10);
    //the rules that were added and the RandomNumber rule that is always there
    assert_eq!(settings.expected_rules().len(), 4);
    assert_eq!(settings.exclude_rules().as_ref().unwrap().len(), 2);

    let random_numbers_1 = settings.generate().unwrap();
    let numbers = random_numbers_1.numbers();
    assert_eq!(numbers.len(), 10);
    assert_eq!(numbers.iter().collect::<HashSet<_>>().len(), 10);
    assert_eq!(numbers.iter().filter(|x| x.is_multiple_of(2)).count(), 5);
    assert!(numbers.iter().all(|x| (1..=100).contains(x) && !(20..=29).contains(x) && !(51..=75).contains(x)), "{:?}", numbers);

    //generate is the same as random_numbers, and the same seed gives the same numbers
    assert_eq!(random_numbers(&build(7)).numbers().unwrap(), numbers);
    assert_eq!(build(7).generate().unwrap().numbers(), numbers);
    assert_eq!(&build(7).generate().unwrap().into_numbers(), numbers);

    //no exclude rules
    assert!(Settings::builder(3).build().exclude_rules().is_none());
}

#[test]
fn settings_builder_shorthands() {
    let shorthand = Settings::builder(10).range(1, 20).no_duplicates().odd_even(5, 5).build();
    let rules = Settings::builder(10).rule(NumberRange::all(1, 20)).rule(NoDuplicate).rule(OddEven::new(5, 5)).build();
    assert_eq!(format!("{:?}", shorthand.expected_rules()), format!("{:?}", rules.expected_rules()));

    for _ in 0..50 {
        let numbers = shorthand.generate().unwrap().into_numbers();
        assert_eq!(numbers.iter().collect::<HashSet<_>>().len(), 10);
        assert_eq!(numbers.iter().filter(|x| x.is_multiple_of(2)).count(), 5);
        assert!(numbers.iter().all(|x| (1..=20).contains(x)), "{:?}", numbers);
    }

    //problems with the rules are reported the same way
    let settings = Settings::builder(5).range(20, 1).odd_even(1, 1).build();
    assert_eq!(settings.errors().len(), 2);
    assert_eq!(RandomErrorType::BadRequest, settings.generate().unwrap_err().error_type());
}

#[test]
fn settings_builder_boxed_rules() {
    //rules that are already in boxes, such as a list that depends on what the program is asked to do
    let mut rules: Vec<Box<dyn RuleTrait>> = vec![Box::new(NumberRange::all(1, 20))];
    rules.push(Box::new(NoDuplicate));
    let excludes: Vec<Box<dyn ExcludeRuleTrait>> = vec![Box::new(NumberRange::all(5, 15))];
    let settings = Settings::builder(8).boxed_rules(rules).rule(OddEven::new(4, 4)).boxed_excludes(excludes).build();
    assert_eq!(settings.expected_rules().len(), 4);
    assert_eq!(settings.exclude_rules().as_ref().unwrap().len(), 1);
    let numbers = settings.generate().unwrap().into_numbers();
    assert!(numbers.iter().all(|x| (1..=4).contains(x) || (16..=20).contains(x)), "{:?}", numbers);
}

#[test]
#[allow(deprecated)]
fn settings_deprecated_constructors() {
    //Settings::new and Settings::with_exclude_rules still work for code written for version 2
    let old = Settings::new(&[Box::new(NoDuplicate{}), Box::new(NumberRange::all(1, 20))], 10);
    let new = Settings::builder(10).rule(NoDuplicate).rule(NumberRange::all(1, 20)).build();
    assert_eq!(format!("{:?}", old.expected_rules()), format!("{:?}", new.expected_rules()));
    assert!(old.exclude_rules().is_none());
    assert_eq!(RandomResultType::Success, random_numbers(&old).status());

    let old = Settings::with_exclude_rules(&[Box::new(NumberRange::all(1, 20))], 10, Some(vec![Box::new(NumberRange::all(5, 9))]));
    let new = Settings::builder(10).rule(NumberRange::all(1, 20)).exclude(NumberRange::all(5, 9)).build();
    assert_eq!(format!("{:?}", old.exclude_rules()), format!("{:?}", new.exclude_rules()));
    let random_result = random_numbers(&old);
    assert!(random_result.numbers().unwrap().iter().all(|x| !(5..=9).contains(x)));

    assert!(Settings::with_exclude_rules(&[], 3, None).exclude_rules().is_none());
}

#[test]
fn generate_ok() {
    let random_numbers_1 = Settings::builder(5).range(97, 122).build().generate().unwrap();
    assert_eq!(random_numbers_1.numbers().len(), 5);
    assert!(random_numbers_1.attempts() >= 1);
    assert!(!random_numbers_1.logs().is_empty());

    //the string is the numbers as characters in the same order, and the shuffled string has the same characters
    let string = random_numbers_1.string().unwrap();
    let expected: String = random_numbers_1.numbers().iter().map(|x| char::from_u32(*x as u32).unwrap()).collect();
    assert_eq!(string, expected);
    let mut sorted_characters: Vec<char> = string.chars().collect();
    sorted_characters.sort_unstable();
    let mut sorted_shuffled_characters: Vec<char> = random_numbers_1.shuffled_string().unwrap().chars().collect();
    sorted_shuffled_characters.sort_unstable();
    assert_eq!(sorted_characters, sorted_shuffled_characters);
    assert_eq!(format!("{:?}", random_numbers_1), format!("RandomNumbers {{ numbers: {:?}, attempts: {} }}", random_numbers_1.numbers(), random_numbers_1.attempts()));

    //the question mark operator works with generate and with the strings
    fn password() -> std::result::Result<String, RandomError> {
        let random_numbers_1 = Settings::builder(12).rule(NumberPool::alphanumeric(12, true)).build().generate()?;
        return random_numbers_1.shuffled_string();
    }
    assert_eq!(password().unwrap().len(), 12);
}

#[test]
fn generate_err() {
    //the settings can not work
    let error = Settings::builder(5).range(20, 1).odd_even(1, 1).build().generate().unwrap_err();
    assert_eq!(error.error_type(), RandomErrorType::BadRequest);
    assert_eq!(error.messages().len(), 2);
    assert_eq!(error.attempts(), 0);
    assert_eq!(error.to_string(), format!("BadRequest: {}; {}", error.messages()[0], error.messages()[1]));
    assert!(error.to_string().contains("NumberRange min: 20 is greater than max: 1"), "{}", error);

    //no numbers match the rules: all 3 numbers have to be odd and they have to be from 2 to 2
    let error = Settings::builder(3).range(2, 2).odd_even(3, 0).build().generate().unwrap_err();
    assert_eq!(error.error_type(), RandomErrorType::Failed);
    assert_eq!(error.attempts(), 500);
    assert_eq!(error.to_string(), "Failed: Could not find numbers that match the rules in 500 attempts");
    assert!(!error.logs().is_empty());
    //Debug has the short version, the settings and the logs
    let debug = format!("{:?}", error);
    assert!(debug.starts_with("Failed: Could not find numbers"), "{}", debug);
    assert!(debug.contains("Settings:") && debug.contains("Attempt - 500"));

    //0xD800 to 0xDFFF are not valid characters
    let random_numbers_1 = Settings::builder(3).range(0xD800, 0xDFFF).build().generate().unwrap();
    for error in [random_numbers_1.string().unwrap_err(), random_numbers_1.shuffled_string().unwrap_err()] {
        assert_eq!(error.error_type(), RandomErrorType::InvalidCharacter);
        assert!(error.to_string().starts_with("InvalidCharacter: Number "), "{}", error);
    }

    //it is a std error
    let _error: Box<dyn std::error::Error> = Box::new(Settings::builder(5).range(20, 1).build().generate().unwrap_err());
}

#[test]
fn random_result_into_result() {
    let random_result = random_numbers(&Settings::builder(5).range(1, 20).build());
    let numbers = random_result.numbers().unwrap().clone();
    assert_eq!(random_result.into_result().unwrap().into_numbers(), numbers);

    let random_result = random_numbers(&Settings::builder(5).range(20, 1).build());
    assert_eq!(random_result.into_result().unwrap_err().error_type(), RandomErrorType::BadRequest);
}
