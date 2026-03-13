use crate::random_trait::get_random_trait;
#[cfg(not(feature="rand"))]
use crate::random_trait::set_random_trait;
use crate::rules::RuleTrait;
use crate::rules::*;
use crate::random::*;
use crate::settings::*;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::sync::Mutex;
use once_cell::sync::Lazy;
#[cfg(not(feature="rand"))]
use once_cell::sync::OnceCell;
#[cfg(not(feature="rand"))]
use std::sync::OnceLock;

const MAX_TRIES: usize = 2000;
static NUMBER_RANGE: Lazy<Mutex<NumberRange>> = Lazy::new(|| { Mutex::new(NumberRange::all(1, 100)) });

#[test]
fn odd_even_with_order_2() {
    let random_result = random_numbers(&Settings::new(&[
        Box::new(OddEvenByIndex::new(&[0, 2, 4], &[1,3]))
    ], 5));
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
        let current_data_settings: Settings = Settings::new(&[], item.2.len());
        let current_data_shared_data: HashMap<String, MapAnyValue> = HashMap::new();
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
            let rand_result = random_numbers(&Settings::new(
                &[Box::new(seq_number.clone()), Box::new(NUMBER_RANGE.lock().unwrap().clone())],
                item.2.len(),
            ));
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
    let data: Vec<(&str, NumberRange, &[usize], std::result::Result<(), (IsWithinErrorType, String)>, std::result::Result<(), String>)> = vec![
        ("[(0, (1, 100))]", NumberRange::all(1, 100), &[1, 99, 4, 6, 7], Ok(()), Ok(())),
        ("[(0, (1, 100)), (1, (1, 100)), (2, (1, 100)), (3, (1, 100)), (4, (1, 100))]", NumberRange::from_map(&[(&[0,1,2,3,4], 1 , 100)]), &[1, 99, 4, 6, 7], Ok(()), Ok(())),
        ("[(0, (1, 1)), (1, (99, 99)), (2, (4, 4)), (3, (6, 6)), (4, (7, 7))]", NumberRange::from_map(&[(&[0], 1,1), (&[1], 99,99), (&[2], 4,4), (&[3], 6,6), (&[4], 7,7)]), &[1, 99, 4, 6, 7], Ok(()), Ok(())),

        ("[(0, (1, 100))]", NumberRange::all(1, 100), &[1, 99, 4, 6, 101], Err((IsWithinErrorType::Regular, "Exclude: false - Selected number 101 at index 4 is not within range of min: 1 and max: 100. Numbers:[1, 99, 4, 6, 101].  Map Index:0".to_owned())), Err("Exclude: false - Selected number 101 at index 4 is not within range of min: 1 and max: 100. Numbers:[1, 99, 4, 6, 101].  Map Index:0".to_owned())),
        ("[(0, (1, 100)), (1, (1, 100)), (2, (1, 100)), (3, (1, 100)), (4, (1, 100))]", NumberRange::from_map(&[(&[0,1,2,3,4], 1 , 100)]), &[1, 100, 4, 6, 7], Ok(()), Ok(())),
        ("[(0, (1, 1)), (1, (99, 99)), (2, (4, 4)), (3, (6, 6)), (4, (7, 7))]", NumberRange::from_map(&[(&[0], 1,1), (&[1], 99,99), (&[2], 4,4), (&[3], 6,6), (&[4], 7,7)]), &[1, 99, 5, 6, 7], Err((IsWithinErrorType::Regular, "Exclude: false - Selected number 5 at index 2 is not within range of min: 4 and max: 4. Numbers:[1, 99, 5, 6, 7].  Map Index:2".to_owned())), Err("Exclude: false - Selected number 5 at index 2 is not within range of min: 4 and max: 4. Numbers:[1, 99, 5, 6, 7].  Map Index:2".to_owned())),
    ];
    for (main_idx, item) in data.iter().enumerate() {
        let current_data_numbers: Vec<usize> = item.2.to_vec();
        let current_data_settings: Settings = Settings::new(&[], item.2.len());
        let current_data_shared_data: HashMap<String, MapAnyValue> = HashMap::new();
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
        let rand_result = random_numbers(&Settings::new(&[Box::new(item.1.clone())], item.2.len()));
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
            item.1.is_match(&CurrentData::new(rand_result.numbers().unwrap(), &Settings::new(&[], item.2.len()), &HashMap::new())),
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
    let settings = Settings::new(&rules, 10);
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
    let settings = Settings::new(&rules, 10);
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
    let settings = Settings::new(&rules, 10);
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
    let settings = Settings::new(&rules, 2);
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
        let current_data_settings: Settings = Settings::new(&[], item.2.len());
        let current_data_shared_data: HashMap<String, MapAnyValue> = HashMap::new();
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
            let rand_result = random_numbers(&Settings::new(
                &[Box::new(*odd_even), Box::new(NUMBER_RANGE.lock().unwrap().clone())],
                item.2.len(),
            ));
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
    let settings = Settings::new(&rules, 10);
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
        let rand_result = random_numbers(&Settings::new(&[Box::new(item.clone())], item.len()));
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
            item.is_match(&CurrentData::new(rand_result.numbers().unwrap(), &Settings::new(&[], item.len()), &HashMap::new())),
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
            NumberPoolItem::new("exclude_set", &PoolType::Set(exclude_set.clone()), OperatorType::Eq(0)),
            NumberPoolItem::new("all_set", &PoolType::Set(all_set.clone()), OperatorType::Eq(all_set.len())),
            NumberPoolItem::new("some_set", &PoolType::Set(some_set.clone()), OperatorType::Eq(3)),
        ])),
        Box::new(NUMBER_RANGE.lock().unwrap().clone()),
    ];
    let settings_1 = Settings::new(&rules_1, 10);
    let rules_2: Vec<Box<dyn RuleTrait>> = vec![
        Box::new(NumberPool::new(&[
            NumberPoolItem::new("exclude_set", &PoolType::MinMax(23, 23), OperatorType::Eq(0)),
            NumberPoolItem::new("all_set", &PoolType::Set(all_set.clone()), OperatorType::Eq(all_set.len())),
            NumberPoolItem::new("some_set", &PoolType::MinMax(17, 21), OperatorType::Eq(3)),
        ])),
        Box::new(NUMBER_RANGE.lock().unwrap().clone()),
    ];
    let settings_2 = Settings::new(&rules_2, 10);
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
            NumberPoolItem::new("exclude_set", &PoolType::Set(exclude_set.clone()), OperatorType::Eq(0)),
            NumberPoolItem::new("all_set", &PoolType::Set(all_set.clone()), OperatorType::Eq(all_set.len())),
        ])),
        Box::new(NUMBER_RANGE.lock().unwrap().clone()),
    ];
    let settings_1 = Settings::new(&rules_1, 10);
    let rules_2: Vec<Box<dyn RuleTrait>> = vec![
        Box::new(NumberPool::new(&[
            NumberPoolItem::new("exclude_set", &PoolType::MinMax(11, 100), OperatorType::Eq(0)),
            NumberPoolItem::new("all_set", &PoolType::MinMax(1, 10), OperatorType::Eq(all_set.len())),
        ])),
        Box::new(NUMBER_RANGE.lock().unwrap().clone()),
    ];
    let settings_2 = Settings::new(&rules_2, 10);
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
    let current_data_settings_1: Settings = Settings::new(&[], current_data_numbers_1.len());
    let current_data_shared_data_1: HashMap<String, MapAnyValue> = HashMap::new();
    let current_data_1 = CurrentData::new(&current_data_numbers_1, &current_data_settings_1, &current_data_shared_data_1);

    let current_data_numbers_2: Vec<usize> = vec![25,24,23,22,21];
    let current_data_settings_2: Settings = Settings::new(&[], current_data_numbers_2.len());
    let current_data_shared_data_2: HashMap<String, MapAnyValue> = HashMap::new();
    let current_data_2 = CurrentData::new(&current_data_numbers_2, &current_data_settings_2, &current_data_shared_data_2);

    let current_data_numbers_3: Vec<usize> = vec![35,35,33,32,31];
    let current_data_settings_3: Settings = Settings::new(&[], current_data_numbers_3.len());
    let current_data_shared_data_3: HashMap<String, MapAnyValue> = HashMap::new();
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
    //assert_ne!(Vec::from_iter(current_data_1.selected_numbers_set().clone()), current_data_numbers_1.iter.clone());
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
    let mut map: HashMap<String, MapAnyValue> = HashMap::new();
            map.insert(
                "min".to_owned(),
                MapAnyValue::Usize(1),
            );
            map.insert(
                "max".to_owned(),
                MapAnyValue::Usize(100),
            );
    let rule = Sequential::new(5, &[]);
    match rule.is_within_range(&CurrentData::new(&vec![5, 2, 7, 6/*20, 35, 4, 11, 12*/], &Settings::new(&[], 5), &HashMap::new())) {
        Ok(_) => panic!("Should Have Thrown Error."),
        Err(e) => assert_eq!("Expected Not: 5 and Seq_Counts: [].  Actual Not: 1 and Seq_Counts: [3].", e.1)
    }
}


#[test]
fn even_number_1() {
    let expected_rules = &Vec::new();
    let settings = &Settings::new(&Vec::new(), 10);
    let shared_data = &HashMap::from_iter([
        (SHARED_DATA_MIN.to_owned(), MapAnyValue::Usize(1)), 
        (SHARED_DATA_MAX.to_owned(), MapAnyValue::Usize(20))
    ]);
    let current_data = &CurrentData::new(expected_rules, settings, shared_data);
    for _ in 0..2000 {
        let even_number = current_data.get_rand(NumberType::Even,None, None);
        assert!(even_number.is_multiple_of(2), "{}", even_number);
        assert!((1..=20).contains(&even_number), "{}", even_number);
    }
}

#[test]
fn odd_number_1() {
    let expected_rules = &Vec::new();
    let settings = &Settings::new(&Vec::new(), 10);
    let shared_data = &HashMap::from_iter([
        (SHARED_DATA_MIN.to_owned(), MapAnyValue::Usize(1)), 
        (SHARED_DATA_MAX.to_owned(), MapAnyValue::Usize(20))
    ]);
    let current_data = &CurrentData::new(expected_rules, settings, shared_data);
    for _ in 0..2000 {
        let odd_number = current_data.get_rand(NumberType::Odd,None, None);
        assert!(odd_number % 2 == 1, "{}", odd_number);
        assert!((1..=20).contains(&odd_number), "{}", odd_number);
    }
}

#[test]
fn random_trait_get_number_1() {
    let mut set = HashSet::new();
    let min = 1;
    let max = 200;
    for _ in 0..2000 {
        let number = get_random_trait().get_number(min, max);
        set.insert(number);
        assert!(number >= min && number <= max, "{}", number);
    }
    assert!(set.len() == max);    
}

#[test]
fn random_trait_get_bool_1() {
    let mut set = HashSet::new();
    for _ in 0..100 {
        let bool_value = get_random_trait().get_bool();
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

