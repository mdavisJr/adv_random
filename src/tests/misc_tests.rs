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
    let random_result = Settings::builder(5)
        .rule(OddEvenByIndex::new().odd([0, 2, 4]).even([1, 3]))
        .build()
        .generate();
    match random_result {
        Ok(random_numbers) => println!("{:?}", random_numbers.numbers()),
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn sequential_1() {
    let data: Vec<(&str, Sequential, &[usize])> = vec![
        ("NOT:5", Sequential::new().singles(5), &[1, 15, 10, 25, 8]),
        ("NOT:2,SEQ1:3", Sequential::new().singles(2).in_a_row(3), &[1, 2, 10, 6, 3]),
        ("NOT:3,SEQ1:2", Sequential::new().singles(3).in_a_row(2), &[1, 2, 26, 6, 8]),
        (
            "NOT:0,SEQ1:2,SEQ2:2",
            Sequential::new().in_a_row(2).in_a_row(2),
            &[1, 2, 4, 5],
        ),
        (
            "NOT:1,SEQ1:2,SEQ2:2",
            Sequential::new().singles(1).in_a_row(2).in_a_row(2),
            &[1, 2, 4, 7, 8],
        ),
        ("NOT:0,SEQ1:5", Sequential::new().in_a_row(5), &[1, 2, 3, 4, 5]),
        (
            "NOT:3,SEQ1:2",
            Sequential::new().singles(3).in_a_row(2),
            &[1, 10, 15, 18, 17],
        ),
        (
            "NOT:3,SEQ1:3",
            Sequential::new().singles(3).in_a_row(3),
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
            let rand_result = Settings::builder(item.2.len())
                .rule(seq_number.clone())
                .rule(NUMBER_RANGE.lock().unwrap().clone())
                .build()
                .generate();
            assert!(rand_result.is_ok(),
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
        ("[(0, (1, 100)), (1, (1, 100)), (2, (1, 100)), (3, (1, 100)), (4, (1, 100))]", NumberRange::new().at([0, 1, 2, 3, 4], 1, 100), &[1, 99, 4, 6, 7], Ok(()), Ok(())),
        ("[(0, (1, 1)), (1, (99, 99)), (2, (4, 4)), (3, (6, 6)), (4, (7, 7))]", NumberRange::new().at([0], 1, 1).at([1], 99, 99).at([2], 4, 4).at([3], 6, 6).at([4], 7, 7), &[1, 99, 4, 6, 7], Ok(()), Ok(())),

        ("[(0, (1, 100))]", NumberRange::all(1, 100), &[1, 99, 4, 6, 101], Err((IsWithinErrorType::Regular, "Exclude: false - Selected number 101 at index 4 is not within range of min: 1 and max: 100. Numbers:[1, 99, 4, 6, 101].  Map Index:0".to_owned())), Err("Exclude: false - Selected number 101 at index 4 is not within range of min: 1 and max: 100. Numbers:[1, 99, 4, 6, 101].  Map Index:0".to_owned())),
        ("[(0, (1, 100)), (1, (1, 100)), (2, (1, 100)), (3, (1, 100)), (4, (1, 100))]", NumberRange::new().at([0, 1, 2, 3, 4], 1, 100), &[1, 100, 4, 6, 7], Ok(()), Ok(())),
        ("[(0, (1, 1)), (1, (99, 99)), (2, (4, 4)), (3, (6, 6)), (4, (7, 7))]", NumberRange::new().at([0], 1, 1).at([1], 99, 99).at([2], 4, 4).at([3], 6, 6).at([4], 7, 7), &[1, 99, 5, 6, 7], Err((IsWithinErrorType::Regular, "Exclude: false - Selected number 5 at index 2 is not within range of min: 4 and max: 4. Numbers:[1, 99, 5, 6, 7].  Map Index:2".to_owned())), Err("Exclude: false - Selected number 5 at index 2 is not within range of min: 4 and max: 4. Numbers:[1, 99, 5, 6, 7].  Map Index:2".to_owned())),
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
        let rand_result = Settings::builder(item.2.len()).rule(item.1.clone()).build().generate();
        assert!(rand_result.is_ok(),
            "MAIN IDX: {}, NUM_RANGE: {:?} \n\n{:?}",
            main_idx,
            item.1,
            rand_result
        );
        assert_eq!(
            Ok(()),
            item.1.is_match(&CurrentData::new(rand_result.as_ref().unwrap().numbers(), &Settings::builder(item.2.len()).build(), &SharedData::new())),
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
        let random_result = settings.generate();
        assert!(random_result.is_ok(),
            "{:?}",
            random_result
        );
        assert_eq!(random_result.as_ref().unwrap().numbers().len(), 10);
        for number in random_result.as_ref().unwrap().numbers() {
            assert!(*number == 1 || *number == 2);
        }
    }
}

#[test]
fn number_range_3() {
    let rules: Vec<Box<dyn RuleTrait>> = vec![Box::new(NumberRange::all(1, 1))];
    let settings = Settings::builder(10).boxed_rules(rules.clone()).build();
    for _ in 0..MAX_TRIES {
        let random_result = settings.generate();
        assert!(random_result.is_ok(),
            "{:?}",
            random_result
        );
        assert_eq!(random_result.as_ref().unwrap().numbers().len(), 10);
        for number in random_result.as_ref().unwrap().numbers() {
            assert_eq!(*number, 1);
        }
    }
}

#[test]
fn number_range_4() {
    let rules: Vec<Box<dyn RuleTrait>> = vec![Box::new(NumberRange::all(100, 100))];
    let settings = Settings::builder(10).boxed_rules(rules.clone()).build();
    for _ in 0..MAX_TRIES {
        let random_result = settings.generate();
        assert!(random_result.is_ok(),
            "{:?}",
            random_result
        );
        assert_eq!(random_result.as_ref().unwrap().numbers().len(), 10);
        for number in random_result.as_ref().unwrap().numbers() {
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
        let random_result = settings.generate();
        assert!(random_result.is_ok(),
            "{:?}",
            random_result
        );
        assert_eq!(random_result.as_ref().unwrap().numbers().len(), 2);
        for number in random_result.as_ref().unwrap().numbers() {
            assert!(*number == 1 || *number == 2);
        }
    }
}

#[test]
fn odd_even_1() {
    let data: Vec<(&str, OddEven, &[usize])> = vec![
        ("ODD:3,EVEN:2", OddEven::new().odd(3).even(2), &[1, 15, 10, 25, 8]),
        ("ODD:3,EVEN:3", OddEven::new().odd(3).even(3), &[4, 2, 10, 5, 11, 15]),
        ("ODD:2,EVEN:3", OddEven::new().odd(2).even(3), &[1, 9, 26, 6, 8]),
        (
            "ODD:5,EVEN:5",
            OddEven::new().odd(5).even(5),
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
            let rand_result = Settings::builder(item.2.len())
                .rule(*odd_even)
                .rule(NUMBER_RANGE.lock().unwrap().clone())
                .build()
                .generate();
            assert!(rand_result.is_ok(),
                "MAIN IDX: {}, FROM_NUMBERS: {}; ODD_EVEN: {:?} \n\n{:?}",
                main_idx,
                idx == 1,
                odd_even,
                rand_result
            );
            assert_eq!(
                Ok(()),
                OddEven::from_numbers(rand_result.as_ref().unwrap().numbers()).is_match(&current_data),
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
        vec![Box::new(OddEven::new().odd(5).even(5)), Box::new(NUMBER_RANGE.lock().unwrap().clone())];
    let settings = Settings::builder(10).boxed_rules(rules.clone()).build();
    for _ in 0..MAX_TRIES {
        let random_result = settings.generate();
        assert!(random_result.is_ok(),
            "{:?}",
            random_result
        );
        assert_eq!(random_result.as_ref().unwrap().numbers().len(), 10);
        let odd_even = OddEven::from_numbers(random_result.as_ref().unwrap().numbers());
        let mut odd: usize = 0;
        let mut even: usize = 0;
        random_result.as_ref().unwrap().numbers().iter().for_each(|x| {
            if x % 2_usize == 0 {
                even += 1
            } else {
                odd += 1
            }
        });
        assert_eq!(odd_even.odd_count(), 5);
        assert_eq!(odd_even.even_count(), 5);
        assert_eq!(odd, 5);
        assert_eq!(even, 5);
    }
}

#[test]
fn random_number() {
    let data: Vec<NumberRange> = vec![
        NumberRange::new().at([0], 1, 1).at([1], 99, 99).at([2], 4, 4).at([3], 6, 6).at([4], 7, 7),
        NumberRange::new().at([0], 20, 20).at([1], 55, 55).at([2], 4, 10).at([3], 71, 75).at([4], 47, 47),
    ];
    for (main_idx, item) in data.iter().enumerate() {
        let rand_result = Settings::builder(item.len()).rule(item.clone()).build().generate();
        assert!(rand_result.is_ok(),
            "MAIN IDX: {} \n\n{:?}",
            main_idx,
            rand_result
        );
        assert_eq!(5, rand_result.as_ref().unwrap().attempts(), "MAIN IDX: {}",
            main_idx,
        );
        assert_eq!(
            Ok(()),
            item.is_match(&CurrentData::new(rand_result.as_ref().unwrap().numbers(), &Settings::builder(item.len()).build(), &SharedData::new())),
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
        Box::new(NumberPool::new()
            .item("exclude_set", PoolType::set(exclude_set.clone()), OperatorType::Eq(0))
            .item("all_set", PoolType::set(all_set.clone()), OperatorType::Eq(all_set.len()))
            .item("some_set", PoolType::set(some_set.clone()), OperatorType::Eq(3))),
        Box::new(NUMBER_RANGE.lock().unwrap().clone()),
        Box::new(NoDuplicate),
    ];
    let settings_1 = Settings::builder(10).boxed_rules(rules_1.clone()).build();
    let rules_2: Vec<Box<dyn RuleTrait>> = vec![
        Box::new(NumberPool::new()
            .item("exclude_set", PoolType::MinMax(23, 23), OperatorType::Eq(0))
            .item("all_set", PoolType::set(all_set.clone()), OperatorType::Eq(all_set.len()))
            .item("some_set", PoolType::MinMax(17, 21), OperatorType::Eq(3))),
        Box::new(NUMBER_RANGE.lock().unwrap().clone()),
        Box::new(NoDuplicate),
    ];
    let settings_2 = Settings::builder(10).boxed_rules(rules_2.clone()).build();
    for settings in [settings_1, settings_2] {
        for _ in 0..MAX_TRIES {
            let random_result = settings.generate();
            assert!(random_result.is_ok(),
                "{:?}",
                random_result
            );
            assert_eq!(random_result.as_ref().unwrap().numbers().len(), 10);
            let numbers_set: HashSet<usize> = random_result.as_ref().unwrap().numbers().iter().copied().collect();

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
        Box::new(NumberPool::new()
            .item("exclude_set", PoolType::set(exclude_set.clone()), OperatorType::Eq(0))
            .item("all_set", PoolType::set(all_set.clone()), OperatorType::Eq(all_set.len()))),
        Box::new(NUMBER_RANGE.lock().unwrap().clone()),
        Box::new(NoDuplicate),
    ];
    let settings_1 = Settings::builder(10).boxed_rules(rules_1.clone()).build();
    let rules_2: Vec<Box<dyn RuleTrait>> = vec![
        Box::new(NumberPool::new()
            .item("exclude_set", PoolType::MinMax(11, 100), OperatorType::Eq(0))
            .item("all_set", PoolType::MinMax(1, 10), OperatorType::Eq(all_set.len()))),
        Box::new(NUMBER_RANGE.lock().unwrap().clone()),
        Box::new(NoDuplicate),
    ];
    let settings_2 = Settings::builder(10).boxed_rules(rules_2.clone()).build();
    for settings in [settings_1, settings_2] {
        for _ in 0..MAX_TRIES {
            let random_result = settings.generate();
            assert!(random_result.is_ok(),
                "{:?}",
                random_result
            );
            assert_eq!(random_result.as_ref().unwrap().numbers().len(), 10);
            let numbers_set: HashSet<usize> = random_result.as_ref().unwrap().numbers().iter().copied().collect();

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

//is_selected and has_duplicates give the same answers as the set, both for a few numbers, which are looked through,
//and for more, which go by the set
#[test]
fn is_selected_and_has_duplicates_match_the_set() {
    let settings = Settings::builder(1).build();
    let shared_data = SharedData::new();
    for len in [0, 1, 5, 16, 17, 40] {
        let unique: Vec<usize> = (1..=len).map(|x| x * 3).collect();
        let mut repeated = unique.clone();
        if let Some(first) = unique.first() {
            repeated.push(*first);
        }
        for numbers in [&unique, &repeated] {
            let current_data = CurrentData::new(numbers, &settings, &shared_data);
            for val in 0..=len * 3 + 1 {
                assert_eq!(current_data.is_selected(val), current_data.selected_numbers_set().contains(&val), "{} in {:?}", val, numbers);
            }
            assert_eq!(current_data.has_duplicates(), numbers.len() != current_data.selected_numbers_set().len(), "{:?}", numbers);
        }
        assert!(!CurrentData::new(&unique, &settings, &shared_data).has_duplicates());
        assert_eq!(CurrentData::new(&repeated, &settings, &shared_data).has_duplicates(), len > 0);
    }
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
    let rule = Sequential::new().singles(5);
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
    assert!(NumberSpaceItem::new(lte).needs(&[2, 3]).has(&[1, 5]).has_too_many());
    //more runs than needed
    assert!(NumberSpaceItem::new(lte).needs(&[2]).has(&[1, 1]).has_too_many());
    assert!(!NumberSpaceItem::new(lte).needs(&[3, 2]).has(&[1, 3]).has_too_many());
    assert!(!NumberSpaceItem::new(lte).needs(&[2, 3]).has(&[2]).has_too_many());
    assert!(!NumberSpaceItem::new(lte).needs(&[2, 3]).has_too_many());
}

#[test]
fn number_pool_counts_duplicates() {
    let digits = PoolType::MinMax(48, 57);
    let rule = NumberPool::new().item("digits", &digits, OperatorType::Eq(2));
    let settings = Settings::builder(4).build();
    let shared_data = SharedData::new();
    //48 is selected three times, so there are three numbers from the pool, not one
    let numbers = vec![48, 48, 48, 70];
    assert!(rule.is_match(&CurrentData::new(&numbers, &settings, &shared_data)).is_err());
    assert!(rule.is_within_range(&CurrentData::new(&numbers, &settings, &shared_data)).is_err());
    let numbers = vec![48, 48, 70, 71];
    assert!(rule.is_match(&CurrentData::new(&numbers, &settings, &shared_data)).is_ok());

    for _ in 0..MAX_TRIES {
        let random_result = Settings::builder(8)
            .rule(NumberRange::all(40, 70))
            .rule(NumberPool::new().item("digits", &digits, OperatorType::Eq(2)))
            .build()
            .generate();
        let numbers = random_result.as_ref().unwrap().numbers();
        assert_eq!(numbers.iter().filter(|x| digits.contains(**x)).count(), 2, "{:?}", numbers);
    }
}

#[test]
fn sequential_seq_counts_any_order() {
    for seq_counts in [[2, 3], [3, 2]] {
        for _ in 0..100 {
            let random_result = Settings::builder(10)
                .rule(NoDuplicate)
                .rule(NumberRange::all(1, 100))
                .rule(Sequential::new().singles(5).in_a_row_each(&seq_counts))
                .build()
                .generate();
            assert!(random_result.is_ok(), "{:?}", random_result);
        }
    }
}

#[test]
fn number_space_single_value_spaces() {
    //Lte(1), Between(4, 4) and Eq(3) only allow one possible higher number
    for operator_type in [OperatorType::Lte(1), OperatorType::Between(4, 4), OperatorType::Eq(3)] {
        for _ in 0..100 {
            let random_result = Settings::builder(10)
                .rule(NumberRange::all(1, 100))
                .rule(NumberSpace::new().run(operator_type, 9))
                .build()
                .generate();
            assert!(random_result.is_ok(), "{:?}", random_result);
            let mut sorted_numbers = random_result.as_ref().unwrap().numbers().to_vec();
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
    assert_eq!(NumberSpaceItem::new(lte).needs(&[3, 2]).needed(), &vec![2, 3]);
}

#[test]
fn string_with_invalid_character() {
    //0xD800 to 0xDFFF are surrogates, which are not valid characters
    let random_result = Settings::builder(3).rule(NumberRange::all(0xD800, 0xDFFF)).build().generate();
    assert!(random_result.is_ok());
    assert!(random_result.as_ref().unwrap().string().is_err());
    assert!(random_result.as_ref().unwrap().shuffled_string().is_err());

    //no number range, so the numbers are almost always too large to be characters
    let random_result = Settings::builder(20).build().generate();
    assert!(random_result.unwrap().string().is_err());

    let random_result = Settings::builder(3).rule(NumberRange::all(97, 122)).build().generate();
    assert_eq!(random_result.as_ref().unwrap().string().unwrap().len(), 3);
}

#[test]
fn check_count_must_equal_count() {
    assert!(OddEven::new().odd(5).even(5).check_count(10).is_ok());
    assert!(OddEven::new().odd(1).even(1).check_count(5).is_err());
    assert!(OddEven::new().odd(5).even(5).check_count(9).is_err());
    assert!(Sequential::new().singles(1).in_a_row(2).in_a_row(2).check_count(5).is_ok());
    assert!(Sequential::new().singles(1).in_a_row(2).check_count(5).is_err());
    assert!(Sequential::new().singles(5).in_a_row(2).check_count(5).is_err());
}

#[test]
fn settings_odd_even_count_mismatch() {
    let settings = Settings::builder(5).rule(OddEven::new().odd(1).even(1)).build();
    assert_eq!(settings.errors().len(), 1);
    assert!(settings.errors()[0].contains("must add up to count"), "{:?}", settings.errors());
    let random_result = settings.generate();
    let error = random_result.unwrap_err();
    assert_eq!(RandomErrorType::BadRequest, error.error_type());
    assert_eq!(error.attempts(), 0);
    assert!(error.messages()[0].contains("must add up to count"));
    assert!(error.logs()[0].to_string().contains("must add up to count"));
}

#[test]
fn no_duplicate_full_range() {
    //every number from 1 to 50 has to be picked, so the last picks only have a few numbers left to choose from
    for _ in 0..50 {
        let random_result = Settings::builder(50)
            .rule(NoDuplicate)
            .rule(NumberRange::all(1, 50))
            .build()
            .generate();
        assert!(random_result.is_ok());
        assert_eq!(random_result.as_ref().unwrap().numbers().iter().collect::<HashSet<_>>().len(), 50);
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
    let random_result = Settings::builder(3)
        .rule(NumberRange::all(1, 10))
        .rule(NumberPool::new().item("exclude", PoolType::MinMax(1, 10), OperatorType::Eq(0)))
        .build()
        .generate();
    assert_eq!(RandomErrorType::Failed, random_result.as_ref().unwrap_err().error_type());
    assert!(random_result.unwrap_err().logs().iter().any(|x| x.to_string().contains(NO_NUMBER_FOUND_ERR)));
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
    let settings = Settings::builder(3).rule(Sequential::new().in_a_row(3)).build();
    let selected = vec![usize::MAX - 1];
    let shared_data = SharedData::new();
    for _ in 0..50 {
        //extending usize::MAX - 1 into a sequence of 3 does not fit, starting a new sequence does
        match Sequential::new().in_a_row(3).get_numbers(&CurrentData::new(&selected, &settings, &shared_data)) {
            Ok(numbers) => assert_eq!(numbers.unwrap().len(), 3),
            Err(e) => assert!(e.contains("does not fit"), "{}", e)
        }
    }
}

#[test]
fn count_zero() {
    let random_result = Settings::builder(0).rule(NumberRange::all(1, 20)).build().generate();
    assert!(random_result.is_ok());
    assert!(random_result.as_ref().unwrap().numbers().is_empty());
    assert_eq!(random_result.as_ref().unwrap().shuffled_string().unwrap(), "");
}

#[test]
fn number_range_min_greater_than_max() {
    let settings = Settings::builder(3).rule(NumberRange::all(20, 1)).build();
    assert_eq!(settings.errors(), ["NumberRange min: 20 is greater than max: 1"]);
    assert_eq!(RandomErrorType::BadRequest, settings.generate().unwrap_err().error_type());
}

#[test]
fn settings_with_more_than_one_error() {
    let settings = Settings::builder(5)
        .rule(NumberRange::all(20, 1))
        .rule(OddEven::new().odd(1).even(1))
        .rule(Sequential::new().singles(1).in_a_row(2))
        .build();
    assert_eq!(settings.errors().len(), 3);
    let random_result = settings.generate();
    assert_eq!(RandomErrorType::BadRequest, random_result.as_ref().unwrap_err().error_type());
    assert_eq!(random_result.as_ref().unwrap_err().messages().len(), 3);
    assert_eq!(random_result.unwrap_err().logs().len(), 3);

    assert!(Settings::builder(5).rule(NumberRange::all(1, 20)).build().errors().is_empty());
}

#[test]
fn number_pool_alphanumeric_count_too_small() {
    for (count, include_special_char) in [(1, false), (2, true)] {
        let number_pool = if include_special_char { NumberPool::alphanumeric(count).with_special_characters() } else { NumberPool::alphanumeric(count) };
        let settings = Settings::builder(count).rule(number_pool).build();
        assert_eq!(settings.errors().len(), 1);
        assert!(settings.errors()[0].contains("or more"), "{:?}", settings.errors());
        assert_eq!(RandomErrorType::BadRequest, settings.generate().unwrap_err().error_type());
    }
    for (count, include_special_char) in [(2, false), (3, true)] {
        let number_pool = if include_special_char { NumberPool::alphanumeric(count).with_special_characters() } else { NumberPool::alphanumeric(count) };
        let random_result = Settings::builder(count).rule(number_pool).build().generate();
        assert_eq!(random_result.as_ref().unwrap().shuffled_string().unwrap().len(), count);
    }
}

#[test]
fn number_range_min_greater_than_max_by_index() {
    assert!(NumberRange::new().at([0, 1], 100, 999).at([2], 1000, 9999).check_count(3).is_ok());
    assert!(NumberRange::new().at([0, 1], 100, 999).at([2], 9999, 1000).check_count(3).is_err());
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
            let random_result = Settings::builder(count)
                .rule(NoDuplicate)
                .rule(NumberRange::all(min, max))
                .rule(Sequential::new().singles(not).in_a_row_each(&seq_counts))
                .build()
                .generate();
            assert!(random_result.is_ok(), "{:?}", random_result);
            let numbers = random_result.as_ref().unwrap().numbers();
            assert!(numbers.iter().all(|x| *x >= min && *x <= max), "{:?}", numbers);
            let actual = Sequential::from_numbers(&CurrentData::new(numbers, &Settings::builder(count).build(), &SharedData::new()), false);
            assert_eq!(actual.not(), not, "{:?}", numbers);
            assert_eq!(actual.seq_counts(), &seq_counts, "{:?}", numbers);
        }
    }
}

#[test]
fn sequential_is_within_range_sequence_too_long() {
    let rule = Sequential::new().singles(1).in_a_row(2).in_a_row(2);
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
    let item = NumberSpaceItem::new(lte).needs(&[2, 3]).has(&[3]);
    assert_eq!(item.missing(), vec![2, 0]);
    assert_eq!(item.process_missing_count(), 2);

    let item = NumberSpaceItem::new(lte).needs(&[1, 4]).has(&[1]);
    assert_eq!(item.missing(), vec![1, 3]);
    assert_eq!(item.process_missing_count(), 3);

    let item = NumberSpaceItem::new(lte).needs(&[2, 3]);
    assert_eq!(item.missing(), vec![2, 3]);
    assert_eq!(item.process_missing_count(), 3);

    let item = NumberSpaceItem::new(lte).needs(&[2, 3]).has(&[2, 3]);
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
    assert_eq!(NumberSpaceItem::new(eq).needs(&[2, 2, 3]).process_missing(), Some((3, true)));
    //the run of 3 is there, so the next run is a new run of 2
    assert_eq!(NumberSpaceItem::new(eq).needs(&[2, 2, 3]).has(&[3]).process_missing(), Some((2, true)));
    //the run of 1 is the start of the run of 3, so the rest of it is not a new run
    assert_eq!(NumberSpaceItem::new(eq).needs(&[2, 2, 3]).has(&[1]).process_missing(), Some((2, false)));
    assert_eq!(NumberSpaceItem::new(eq).needs(&[2, 2, 3]).has(&[2, 2, 3]).process_missing(), None);
    assert_eq!(NumberSpaceItem::new(eq).needs(&[2, 2, 3]).has(&[2, 2, 3]).process_missing_count(), 0);
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
            let random_result = Settings::builder(12)
                .rule(NumberRange::all(1, 100))
                .rule(NumberSpace::new().runs(operator_type, &[2, 2, 3]))
                .build()
                .generate();
            if let Ok(numbers) = random_result.as_ref().map(|x| x.numbers()) {
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
    let number_space = NumberSpace::new().runs(OperatorType::Eq(3), &[2, 2, 2]);
    assert!(number_space.check_count(9).is_ok());
    assert!(number_space.check_count(8).is_err());

    let settings = Settings::builder(8).rule(NumberRange::all(1, 100)).rule(number_space).build();
    assert_eq!(settings.errors().len(), 1);
    assert_eq!(RandomErrorType::BadRequest, settings.generate().unwrap_err().error_type());
}

#[test]
fn number_space_is_within_range_lone_numbers() {
    let shared_data = SharedData::new();
    let rule = NumberSpace::new().runs(OperatorType::Eq(3), &[2, 2, 2]);

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
            let random_result = Settings::builder(count)
                .rule(NoDuplicate)
                .rule(NumberRange::all(1, 100))
                .rule(NumberSpace::new().runs(operator_type, &runs))
                .build()
                .generate();
            if let Ok(numbers) = random_result.as_ref().map(|x| x.numbers()) {
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
        let random_result = Settings::builder(6)
            .rule(NumberRange::all(10, 100))
            .rule(ShareMultiple { name: "ShareMultiple".to_owned(), multiple: 5 })
            .rule(MultipleOfShared {})
            .build()
            .generate();
        assert!(random_result.is_ok(), "{:?}", random_result);
        let numbers = random_result.as_ref().unwrap().numbers();
        assert_eq!(numbers.len(), 6);
        assert!(numbers.iter().all(|x| x.is_multiple_of(5) && *x >= 10 && *x <= 100), "{:?}", numbers);
    }
}

#[test]
fn custom_rules_set_same_custom_key() {
    let random_result = Settings::builder(6)
        .rule(NumberRange::all(10, 100))
        .rule(ShareMultiple { name: "RuleA".to_owned(), multiple: 5 })
        .rule(ShareMultiple { name: "RuleB".to_owned(), multiple: 7 })
        .rule(MultipleOfShared {})
        .build()
        .generate();
    let random_error = random_result.unwrap_err();
    assert_eq!(RandomErrorType::BadRequest, random_error.error_type());
    assert_eq!(random_error.attempts(), 1);
    assert_eq!(random_error.messages().len(), 1);
    let error = random_error.logs().last().unwrap().to_string();
    assert!(error.contains("Custom shared data key \"multiple\" was set by both"), "{}", error);
    assert!(error.contains("RuleA") && error.contains("RuleB"), "{}", error);
}

#[test]
fn number_space_not_possible_with_no_duplicate() {
    let settings = |operator_type: OperatorType, runs: &[usize], count: usize, no_duplicate: bool| -> Settings {
        let mut rules: Vec<Box<dyn RuleTrait>> = vec![
            Box::new(NumberRange::all(1, 100)),
            Box::new(NumberSpace::new().runs(operator_type, runs)),
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
            let random_result = settings.generate();
            assert_eq!(RandomErrorType::BadRequest, random_result.as_ref().unwrap_err().error_type());
            assert_eq!(random_result.unwrap_err().attempts(), 0);
        }
    }

    //one run of 7 is possible
    let settings_1 = settings(OperatorType::Gte(1), &[7], 8, true);
    assert!(settings_1.errors().is_empty());
    assert!(settings_1.generate().is_ok());

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
            .rule(OddEven::new().odd(5).even(5))
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

    //the same seed gives the same numbers
    assert_eq!(build(7).generate().unwrap().numbers(), numbers);
    assert_eq!(&build(7).generate().unwrap().into_numbers(), numbers);

    //no exclude rules
    assert!(Settings::builder(3).build().exclude_rules().is_none());
}

#[test]
fn settings_builder_shorthands() {
    let shorthand = Settings::builder(10).range(1, 20).no_duplicates().odd_even(5, 5).build();
    let rules = Settings::builder(10).rule(NumberRange::all(1, 20)).rule(NoDuplicate).rule(OddEven::new().odd(5).even(5)).build();
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
    let settings = Settings::builder(8).boxed_rules(rules).rule(OddEven::new().odd(4).even(4)).boxed_excludes(excludes).build();
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
        let random_numbers_1 = Settings::builder(12).rule(NumberPool::alphanumeric(12).with_special_characters()).build().generate()?;
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
    //Debug is the report
    assert_eq!(format!("{:?}", error), error.report());

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
#[allow(deprecated)]
fn random_result_into_result() {
    let random_result = random_numbers(&Settings::builder(5).range(1, 20).build());
    let numbers = random_result.numbers().unwrap().clone();
    assert_eq!(random_result.into_result().unwrap().into_numbers(), numbers);

    let random_result = random_numbers(&Settings::builder(5).range(20, 1).build());
    assert_eq!(random_result.into_result().unwrap_err().error_type(), RandomErrorType::BadRequest);
}

#[test]
fn report() {
    //the numbers were found
    let random_numbers_1 = Settings::builder(5).range(1, 20).no_duplicates().build().generate().unwrap();
    let report = random_numbers_1.report();
    let expected_start = format!("Status - Success\nNumbers - {:?}\nAttempts - {}\nClear Errs - []\n\nInfo  : CLEAR - []\n\nInfo  : Attempt - 1\n", random_numbers_1.numbers(), random_numbers_1.attempts());
    assert!(report.starts_with(&expected_start), "{}", report);
    //every log is there, and there is a blank line before every attempt
    assert_eq!(report.lines().filter(|x| x.starts_with("Info  : ") || x.starts_with("Error : ")).count(), random_numbers_1.logs().len());
    assert_eq!(report.matches("\n\nInfo  : Attempt - ").count(), random_numbers_1.attempts());

    //no numbers match the rules: all 3 numbers have to be odd and they have to be from 2 to 2
    let error = Settings::builder(3).range(2, 2).odd_even(3, 0).build().generate().unwrap_err();
    let report = error.report();
    assert!(report.starts_with("Settings:\n\n"), "{}", report);
    assert!(report.contains("\n\nStatus - Failed\nError - Could not find numbers that match the rules in 500 attempts\nAttempts - 500\nClear Errs - ["), "{}", report);
    assert_eq!(report.matches("\n\nInfo  : Attempt - ").count(), 500);
    assert!(!error.clear_err_tracker().is_empty());
    let mut clear_errs: Vec<usize> = error.clear_err_tracker().iter().copied().collect();
    clear_errs.sort_unstable();
    assert!(report.contains(&format!("Clear Errs - {:?}\n", clear_errs)), "{}", report);

    //the settings can not work, so there are no attempts
    let error = Settings::builder(5).range(20, 1).build().generate().unwrap_err();
    assert!(error.report().ends_with("Status - BadRequest\nError - NumberRange min: 20 is greater than max: 1\nAttempts - 0\nClear Errs - []\n\nError : NumberRange min: 20 is greater than max: 1"), "{}", error.report());
}

#[test]
fn rule_chains() {
    //a pool is a list of numbers, a range, or the characters of a string
    let other_pool = PoolType::MinMax(60, 69);
    let number_pool = NumberPool::new()
        .none_of([23])
        .exactly(2, vec![4, 1])
        .at_least(3, 17..=21)
        .at_most(1, "AB")
        .between(1, 2, HashSet::from([50, 51]))
        .item("my_key", &other_pool, OperatorType::Gt(0));
    assert_eq!(number_pool.number_pool_items().keys().collect::<Vec<_>>(), ["my_key", "pool_001", "pool_002", "pool_003", "pool_004", "pool_005"]);
    assert_eq!(number_pool.needs("pool_001"), Some(OperatorType::Eq(0)));
    assert_eq!(number_pool.needs("pool_002"), Some(OperatorType::Eq(2)));
    assert_eq!(number_pool.needs("pool_003"), Some(OperatorType::Gte(3)));
    assert_eq!(number_pool.needs("pool_004"), Some(OperatorType::Lte(1)));
    assert_eq!(number_pool.needs("pool_005"), Some(OperatorType::Between(1, 2)));
    assert_eq!(number_pool.needs("my_key"), Some(OperatorType::Gt(0)));
    assert_eq!(format!("{:?}", number_pool.pool("pool_002").unwrap()), "Set([1, 4])");
    assert_eq!(format!("{:?}", number_pool.pool("pool_003").unwrap()), "MinMax(17, 21)");
    assert_eq!(format!("{:?}", number_pool.pool("pool_004").unwrap()), "Set([65, 66])");
    assert_eq!(format!("{:?}", number_pool.pool("my_key").unwrap()), "MinMax(60, 69)");
    let slice: &[usize] = &[3, 1, 3];
    assert_eq!(format!("{:?}", PoolType::from(slice)), "Set([1, 3])");

    //the order the sequences are given in does not matter
    let sequential = Sequential::new().singles(1).in_a_row(3).in_a_row(2);
    assert_eq!((sequential.not(), sequential.seq_counts()), (1, &vec![2, 3]));
    assert_eq!(Sequential::new().singles(1).in_a_row_each(&[3, 2]).seq_counts(), &vec![2, 3]);
    assert_eq!((Sequential::new().not(), Sequential::new().seq_counts().len()), (0, 0));

    //the runs of one type are kept together
    let number_space = NumberSpace::new().run(OperatorType::Lt(3), 3).run(OperatorType::Gte(10), 1).run(OperatorType::Lt(3), 2);
    assert_eq!(number_space.number_space_items().len(), 2);
    assert_eq!(number_space.number_space_items()[0].needed(), &vec![2, 3]);
    assert_eq!(number_space.number_space_items()[1].needed(), &vec![1]);
    assert_eq!(NumberSpace::new().runs(OperatorType::Lt(3), &[3, 2]).number_space_items()[0].needed(), &vec![2, 3]);
    let item = NumberSpaceItem::new(OperatorType::Between(1, 3)).needs(&[2]).has(&[2]);
    assert_eq!(NumberSpace::new().item(item).number_space_items()[0].found(), &vec![2]);
    let items = vec![NumberSpaceItem::new(OperatorType::Lt(3)).needs(&[2]), NumberSpaceItem::new(OperatorType::Gt(9)).needs(&[1])];
    assert_eq!(NumberSpace::new().items(items).number_space_items().len(), 2);

    //the by index rules take lists and ranges of indexes
    assert_eq!(NumberRange::new().at([0, 1], 100, 999).at(2..=2, 1000, 9999).to_string(), "[(0, (100, 999)), (1, (100, 999)), (2, (1000, 9999))]");
    let random_numbers_1 = Settings::builder(4)
        .range(1, 10)
        .rule(OddEvenByIndex::new().odd(0..2).even([2, 3]))
        .rule(NumberPoolByIndex::new().at([3], [10]))
        .build()
        .generate()
        .unwrap();
    let numbers = random_numbers_1.numbers();
    assert!(numbers[0] % 2 == 1 && numbers[1] % 2 == 1 && numbers[2].is_multiple_of(2) && numbers[3] == 10, "{:?}", numbers);
}

#[test]
fn rule_validation() {
    let errors = |rule: Box<dyn RuleTrait>, count: usize| -> Vec<String> {
        return Settings::builder(count).boxed_rules(vec![rule]).build().errors().to_vec();
    };

    //rules that were created the right way have nothing wrong
    assert!(errors(Box::new(Sequential::new().singles(1).in_a_row(2).in_a_row(2)), 5).is_empty());
    assert!(errors(Box::new(NumberRange::new().at([0, 1], 100, 999).at([2], 1000, 9999)), 3).is_empty());
    assert!(errors(Box::new(OddEvenByIndex::new().odd([0, 2, 4]).even([1, 3])), 5).is_empty());
    assert!(errors(Box::new(NumberPoolByIndex::new().at([0, 1], "AB").at(2..=4, 1..=9)), 5).is_empty());
    assert!(errors(Box::new(NumberPool::new().none_of([23]).exactly(2, [1, 4]).at_least(3, 17..=21)), 10).is_empty());
    assert!(errors(Box::new(NumberSpace::new().run(OperatorType::Lt(3), 4).run(OperatorType::Between(10, 20), 2)), 10).is_empty());

    //Sequential
    assert_eq!(errors(Box::new(Sequential::new().singles(2).in_a_row(1)), 3), ["Sequential in_a_row: 1 must be 2 or more"]);
    //every problem is there, not only the first one
    let sequential_errors = errors(Box::new(Sequential::new().singles(1).in_a_row(0).in_a_row(1)), 5);
    assert_eq!(sequential_errors.len(), 3, "{:?}", sequential_errors);
    assert!(sequential_errors[2].contains("must equal count: 5"), "{:?}", sequential_errors);

    //NumberRange
    assert_eq!(errors(Box::new(NumberRange::new().at([0, 1], 1, 9).at([1], 1, 5)), 3), ["NumberRange index: 1 has more than one range"]);
    assert_eq!(errors(Box::new(NumberRange::new().at([0], 9, 1)), 3), ["NumberRange min: 9 is greater than max: 1"]);
    assert_eq!(errors(Box::new(NumberRange::new().at([7], 1, 9)), 5), ["NumberRange index: 7 is past the last number for count: 5"]);
    assert_eq!(errors(Box::new(NumberRange::all(1, 9).at([0], 1, 5)), 5), ["NumberRange at can not be used with all"]);

    //OddEvenByIndex
    assert_eq!(errors(Box::new(OddEvenByIndex::new().odd([0, 2]).even([2])), 5), ["OddEvenByIndex index: 2 can not be both odd and even"]);
    assert_eq!(errors(Box::new(OddEvenByIndex::new().odd([5])), 5), ["OddEvenByIndex index: 5 is past the last number for count: 5"]);
    assert!(errors(Box::new(OddEvenByIndex::new().odd([0, 0])), 5).is_empty());

    //NumberPoolByIndex
    assert_eq!(errors(Box::new(NumberPoolByIndex::new().at([0], "")), 5), ["NumberPoolByIndex pool is empty"]);
    assert_eq!(errors(Box::new(NumberPoolByIndex::new().at([0, 1], "AB").at([1, 2], "CD")), 5), ["NumberPoolByIndex index: 1 has more than one pool"]);
    assert_eq!(errors(Box::new(NumberPoolByIndex::new().at([9], "AB")), 5), ["NumberPoolByIndex index: 9 is past the last number for count: 5"]);

    //NumberPool
    assert_eq!(errors(Box::new(NumberPool::new().exactly(1, Vec::new())), 5), ["NumberPool pool \"pool_001\" is empty"]);
    assert_eq!(errors(Box::new(NumberPool::new().exactly(1, PoolType::MinMax(9, 1))), 5), ["NumberPool pool \"pool_001\" is empty"]);
    //an empty pool is fine when nothing has to come from it
    assert!(errors(Box::new(NumberPool::new().none_of(Vec::new()).at_most(2, Vec::new())), 5).is_empty());
    assert_eq!(errors(Box::new(NumberPool::new().between(3, 1, [1, 2, 3])), 5), ["NumberPool pool \"pool_001\" Between lower: 3 is greater than upper: 1"]);
    assert_eq!(errors(Box::new(NumberPool::new().item("a", [1], OperatorType::Eq(1)).item("a", [2], OperatorType::Eq(1))), 5), ["NumberPool has more than one pool with the key \"a\""]);
    assert_eq!(errors(Box::new(NumberPool::new().exactly(6, [1, 2, 3])), 5), ["NumberPool count: 6 is greater than count: 5 "]);

    //NumberSpace
    assert_eq!(errors(Box::new(NumberSpace::new().run(OperatorType::Lt(3), 0)), 5), ["NumberSpace run of Lt(3): 0 must be 1 or more"]);
    assert_eq!(errors(Box::new(NumberSpace::new().run(OperatorType::Between(6, 3), 2)), 5), ["NumberSpace Between lower: 6 is greater than upper: 3"]);
    assert_eq!(errors(Box::new(NumberSpace::new().run(OperatorType::Lt(0), 2)), 5), ["NumberSpace Lt(0) never matches because nothing is less than 0"]);
    assert_eq!(errors(Box::new(NumberSpace::new().run(OperatorType::Lt(3), 5)), 5).len(), 1);

    //a pool can not give more different numbers than it has
    let settings = Settings::builder(5).range(1, 20).rule(NumberPool::new().exactly(3, [1, 2])).build();
    assert!(settings.errors().is_empty());
    let settings = Settings::builder(5).range(1, 20).no_duplicates().rule(NumberPool::new().exactly(3, [1, 2])).build();
    assert_eq!(settings.errors(), ["NumberPool pool \"pool_001\" needs 3 numbers but only has 2, which is not possible with NoDuplicate"]);

    //exclude rules are checked too
    let settings = Settings::builder(5)
        .range(1, 20)
        .exclude(NumberRange::new().at([0], 1, 5).at([0], 6, 9))
        .exclude(Sequential::new().in_a_row(1))
        .build();
    assert_eq!(settings.errors(), ["NumberRange index: 0 has more than one range", "Sequential in_a_row: 1 must be 2 or more"]);

    //no numbers are created when a rule has something wrong
    let error = Settings::builder(5).range(1, 20).rule(OddEvenByIndex::new().odd([1]).even([1])).build().generate().unwrap_err();
    assert_eq!(error.error_type(), RandomErrorType::BadRequest);
    assert_eq!(error.attempts(), 0);
    assert_eq!(error.messages(), &vec!["OddEvenByIndex index: 1 can not be both odd and even".to_owned()]);
}

#[test]
#[allow(deprecated)]
fn rule_deprecated_constructors() {
    //NumberRange::from_map and NumberSpace::new_with_process_number_space_items still work
    let old = NumberRange::from_map(&[(&[0, 1], 100, 999), (&[2], 1000, 9999)]);
    assert_eq!(old.to_string(), NumberRange::new().at([0, 1], 100, 999).at([2], 1000, 9999).to_string());
    let items = [NumberSpaceItem::new(OperatorType::Lt(3)).needs(&[9])];
    let old = NumberSpace::new_with_process_number_space_items(&items, ProcessNumberSpaceItems::OnePerRequest);
    let new = NumberSpace::new().run(OperatorType::Lt(3), 9).process(ProcessNumberSpaceItems::OnePerRequest);
    assert_eq!(old.number_space_items()[0].needed(), new.number_space_items()[0].needed());
    assert!(Settings::builder(10).range(1, 100).rule(old).build().generate().is_ok());
}

#[test]
fn number_pool_alphanumeric() {
    let counts = |string: &str| -> (usize, usize, usize) {
        return (
            string.chars().filter(|x| x.is_ascii_alphabetic()).count(),
            string.chars().filter(|x| x.is_ascii_digit()).count(),
            string.chars().filter(|x| NP_SPECIAL_CHAR_SET.contains(x)).count()
        );
    };
    let generate = |number_pool: NumberPool, count: usize| -> String {
        return Settings::builder(count).rule(number_pool).build().generate().unwrap().shuffled_string().unwrap();
    };

    for _ in 0..50 {
        //letters and 1 to 3 digits
        let (letters, digits, special_characters) = counts(&generate(NumberPool::alphanumeric(20), 20));
        assert!((1..=3).contains(&digits) && letters == 20 - digits && special_characters == 0);
        //and 1 or 2 special characters
        let (letters, digits, special_characters) = counts(&generate(NumberPool::alphanumeric(20).with_special_characters(), 20));
        assert!((1..=3).contains(&digits) && (1..=2).contains(&special_characters) && letters == 20 - digits - special_characters);
        //fewer than 10 characters have 1 digit and 1 special character
        assert_eq!(counts(&generate(NumberPool::alphanumeric(9).with_special_characters(), 9)), (7, 1, 1));
    }

    //exactly this many digits and special characters, and the rest are letters
    assert_eq!(counts(&generate(NumberPool::alphanumeric(20).digits(10), 20)), (10, 10, 0));
    assert_eq!(counts(&generate(NumberPool::alphanumeric(25).digits(10).special_characters(5), 25)), (10, 10, 5));
    assert_eq!(counts(&generate(NumberPool::alphanumeric(8).digits(8), 8)), (0, 8, 0));
    assert_eq!(counts(&generate(NumberPool::alphanumeric(8).digits(0).special_characters(2), 8)), (6, 0, 2));

    //the other pool methods work with it.  None of these look alike characters are used.
    for _ in 0..50 {
        let string = generate(NumberPool::alphanumeric(20).none_of("O0oIl1"), 20);
        assert!(!string.chars().any(|x| "O0oIl1".contains(x)), "{}", string);
    }

    //what is wrong with it is reported
    let errors = |number_pool: NumberPool, count: usize| -> Vec<String> {
        return Settings::builder(count).rule(number_pool).build().errors().to_vec();
    };
    assert_eq!(errors(NumberPool::alphanumeric(1), 1), ["NumberPool alphanumeric count: 1 must be 2 or more"]);
    assert_eq!(errors(NumberPool::alphanumeric(2).with_special_characters(), 2), ["NumberPool alphanumeric count: 2 must be 3 or more"]);
    assert_eq!(errors(NumberPool::alphanumeric(5).digits(4).special_characters(2), 5), ["NumberPool alphanumeric digits: 4 and special characters: 2 are more than count: 5"]);
    assert_eq!(errors(NumberPool::new().exactly(1, [1]).digits(2), 5), ["NumberPool digits can only be used with alphanumeric"]);
    assert_eq!(errors(NumberPool::alphanumeric(20), 10), ["NumberPool count: 20 is greater than count: 10 "]);
}

#[test]
#[allow(deprecated)]
fn number_pool_alphanumeric_deprecated_constructors() {
    use crate::seeded_random::SeededRandom;
    //alphanumeric_with_random and the settings pick the same counts and characters for the same seed
    for seed in 0..20 {
        let random = SeededRandom::new(seed);
        let old = Settings::builder(12)
            .rule(NumberPool::alphanumeric_with_random(&random, 12, true))
            .random(random)
            .build()
            .generate()
            .unwrap()
            .into_numbers();
        let new = Settings::builder(12)
            .rule(NumberPool::alphanumeric(12).with_special_characters())
            .random(SeededRandom::new(seed))
            .build()
            .generate()
            .unwrap()
            .into_numbers();
        assert_eq!(old, new);
    }

    let old = NumberPool::alphanumeric_specs(10, 8, 2);
    assert_eq!(old.needs("alpha_set"), Some(OperatorType::Eq(10)));
    assert_eq!(old.needs("numeric_set"), Some(OperatorType::Eq(8)));
    assert_eq!(old.needs("special_char_set"), Some(OperatorType::Eq(2)));
    assert!(Settings::builder(20).rule(old).build().generate().is_ok());
}

#[test]
fn exclude_number_sets() {
    let rule = ExcludeNumberSets::new()
        .numbers(&[1, 2, 3])
        .numbers_each(vec![vec![3, 2, 1], vec![2, 1, 3]])
        .string("abc")
        .string_each(["abd", "abe"]);
    assert_eq!(rule.excluded_number_sets().len(), 6);
    assert!(rule.excluded_number_sets().contains(&vec![97, 98, 99]));
    assert!(rule.excluded_number_sets().contains(&vec![97, 98, 101]));

    //1, 2 and 3 in any order has 6 ways, and 3 of them are excluded
    let settings = Settings::builder(3)
        .range(1, 3)
        .no_duplicates()
        .rule(ExcludeNumberSets::new().numbers(&[1, 2, 3]).numbers_each(vec![vec![3, 2, 1], vec![2, 1, 3]]))
        .build();
    assert!(settings.errors().is_empty());
    let mut found: HashSet<Vec<usize>> = HashSet::new();
    for _ in 0..200 {
        found.insert(settings.generate().unwrap().into_numbers());
    }
    assert_eq!(found, HashSet::from([vec![1, 3, 2], vec![2, 3, 1], vec![3, 1, 2]]));

    //a list that does not have as many numbers as the count can never be the random numbers
    let settings = Settings::builder(3).range(1, 3).rule(ExcludeNumberSets::new().numbers(&[1, 2, 3]).string("ab")).build();
    assert_eq!(settings.errors(), ["ExcludeNumberSets [97, 98] has 2 numbers, so it can never be the 3 random numbers"]);
    assert_eq!(RandomErrorType::BadRequest, settings.generate().unwrap_err().error_type());
}

#[test]
#[allow(deprecated)]
fn exclude_number_sets_deprecated_constructor() {
    let old = ExcludeNumberSets::new_string(&HashSet::from([String::from("AB1234567"), String::from("CB1234567")]));
    let new = ExcludeNumberSets::new().string("AB1234567").string("CB1234567");
    assert_eq!(old.excluded_number_sets(), new.excluded_number_sets());
}

#[test]
fn rule_validation_incomplete_chains() {
    fn errors(rule: impl RuleTrait + 'static, count: usize) -> Vec<String> {
        return Settings::builder(count).range(1, 100).rule(rule).build().errors().to_vec();
    }

    //a rule that nothing was added to does nothing, so it is reported
    assert_eq!(errors(NumberPool::new(), 5), ["NumberPool has no pools"]);
    assert_eq!(errors(NumberSpace::new(), 5), ["NumberSpace has no runs"]);
    assert_eq!(errors(NumberRange::new(), 5), ["NumberRange has no ranges"]);
    assert_eq!(errors(OddEvenByIndex::new(), 5), ["OddEvenByIndex has no indexes"]);
    assert_eq!(errors(NumberPoolByIndex::new(), 5), ["NumberPoolByIndex has no pools"]);
    assert_eq!(errors(ExcludeNumberSets::new(), 5), ["ExcludeNumberSets has nothing to exclude"]);
    assert_eq!(errors(NumberSpace::new().items(Vec::new()), 5), ["NumberSpace has no runs"]);
    assert_eq!(errors(ExcludeNumberSets::new().numbers_each(Vec::new()), 5), ["ExcludeNumberSets has nothing to exclude"]);
    //Sequential and OddEven with nothing added do not add up to the count
    assert_eq!(errors(Sequential::new(), 5).len(), 1);
    assert_eq!(errors(OddEven::new(), 5).len(), 1);

    //the same rules with something added are fine
    assert!(errors(NumberPool::new().none_of([7]), 5).is_empty());
    assert!(errors(NumberPool::alphanumeric(5), 5).is_empty());
    assert!(errors(NumberSpace::new().run(OperatorType::Lt(3), 2), 5).is_empty());
    assert!(errors(NumberRange::new().at([0], 1, 9), 5).is_empty());
    assert!(errors(OddEvenByIndex::new().odd([0]), 5).is_empty());
    assert!(errors(NumberPoolByIndex::new().at([0], [1, 2]), 5).is_empty());
    assert!(errors(ExcludeNumberSets::new().numbers(&[1, 2, 3, 4, 5]), 5).is_empty());

    //a method that sets one value can only be called once
    assert_eq!(errors(Sequential::new().singles(2).singles(5), 5), ["Sequential singles was set more than once"]);
    assert_eq!(errors(OddEven::new().odd(1).odd(3).even(2), 5), ["OddEven odd was set more than once"]);
    assert_eq!(errors(OddEven::new().odd(3).even(1).even(2), 5), ["OddEven even was set more than once"]);
    assert_eq!(errors(NumberPool::alphanumeric(5).digits(1).digits(2), 5), ["NumberPool digits was set more than once"]);
    assert_eq!(errors(NumberPool::alphanumeric(5).with_special_characters().special_characters(2), 5), ["NumberPool special characters were set more than once"]);
    //a method that adds to a list can be called as many times as it is needed
    assert!(errors(Sequential::new().singles(1).in_a_row(2).in_a_row(2), 5).is_empty());

    //exclude rules are checked the same way
    let settings = Settings::builder(5)
        .range(1, 100)
        .exclude(NumberSpace::new())
        .exclude(OddEven::new().odd(1).odd(2))
        .exclude(ExcludeNumberSets::new())
        .build();
    assert_eq!(settings.errors(), ["NumberSpace has no runs", "OddEven odd was set more than once", "ExcludeNumberSets has nothing to exclude"]);
}

#[test]
fn number_pool_spread() {
    //counts per pool, for the pools 1 to 13, 14 to 26 and 27 to 39
    let counts = |numbers: &[usize]| -> [usize; 3] {
        return [
            numbers.iter().filter(|x| **x <= 13).count(),
            numbers.iter().filter(|x| **x >= 14 && **x <= 26).count(),
            numbers.iter().filter(|x| **x >= 27).count()
        ];
    };

    let number_pool = NumberPool::new().spread([1, 0, 4]).over(1..=13).over(14..=26).over(27..=39);
    assert_eq!(number_pool.spreads().len(), 1);
    //the counts are kept from the largest to the smallest, because the order they are given in does not matter
    assert_eq!(number_pool.spreads()[0].counts(), &vec![4, 1, 0]);
    assert_eq!(number_pool.spreads()[0].pools().len(), 3);
    assert_eq!(number_pool.spreads()[0].has(&[1, 2, 30, 31, 32]), vec![2, 0, 3]);

    //4 numbers are from one pool, 1 number is from another pool, and every way that can happen is found
    let settings = Settings::builder(5).range(1, 39).no_duplicates().rule(number_pool).build();
    assert!(settings.errors().is_empty(), "{:?}", settings.errors());
    let mut found: HashSet<[usize; 3]> = HashSet::new();
    for _ in 0..600 {
        let random_numbers_1 = settings.generate().unwrap();
        let has = counts(random_numbers_1.numbers());
        let mut sorted_has = has;
        sorted_has.sort_unstable_by(|a, b| b.cmp(a));
        assert_eq!(sorted_has, [4, 1, 0], "{:?}", random_numbers_1.numbers());
        found.insert(has);
    }
    assert_eq!(found.len(), 6, "{:?}", found);

    //another rule decides which pool gets which count.  The 5 numbers add up to 60 or less only when 4 of them are low.
    #[derive(Clone, Debug)]
    struct SumAtMost(usize);
    impl std::fmt::Display for SumAtMost {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "SumAtMost:{}", self.0) }
    }
    impl RuleTrait for SumAtMost {
        fn as_any(&self) -> &dyn std::any::Any { self }
        fn share_data(&self, _current_data: &CurrentData) -> Option<SharedData> { None }
        fn get_numbers(&self, _current_data: &CurrentData) -> std::result::Result<Option<Vec<usize>>, String> { Ok(None) }
        fn is_within_range(&self, current_data: &CurrentData) -> std::result::Result<(), (IsWithinErrorType, String)> {
            let sum: usize = current_data.selected_numbers().iter().sum();
            if sum > self.0 { Err((IsWithinErrorType::Regular, format!("Sum {} is more than {}", sum, self.0))) } else { Ok(()) }
        }
        fn is_match(&self, current_data: &CurrentData) -> std::result::Result<(), String> { self.is_within_range(current_data).map_err(|x| x.1) }
        fn check_count(&self, _count: usize) -> std::result::Result<bool, String> { Ok(true) }
        fn name(&self) -> String { String::from("SumAtMost") }
    }
    let settings = Settings::builder(5)
        .range(1, 39)
        .no_duplicates()
        .rule(NumberPool::new().spread([4, 1, 0]).over(1..=13).over(14..=26).over(27..=39))
        .rule(SumAtMost(60))
        .build();
    for _ in 0..50 {
        let random_numbers_1 = settings.generate().unwrap();
        assert_eq!(counts(random_numbers_1.numbers())[0], 4, "{:?}", random_numbers_1.numbers());
        assert!(random_numbers_1.numbers().iter().sum::<usize>() <= 60);
    }

    //it works with the other pool methods, with pools of different kinds, and with numbers that are not in any pool
    let settings = Settings::builder(6)
        .range(1, 100)
        .rule(NumberPool::new().spread([2, 1]).over(1..=10).over([50, 60, 70]).none_of([5]).exactly(1, [99]))
        .build();
    assert!(settings.errors().is_empty(), "{:?}", settings.errors());
    for _ in 0..100 {
        let numbers = settings.generate().unwrap().into_numbers();
        let low = numbers.iter().filter(|x| **x <= 10).count();
        let tens = numbers.iter().filter(|x| [50, 60, 70].contains(*x)).count();
        assert!((low == 2 && tens == 1) || (low == 1 && tens == 2), "{:?}", numbers);
        assert!(!numbers.contains(&5) && numbers.iter().filter(|x| **x == 99).count() == 1, "{:?}", numbers);
    }

    //a full house: 3 of one number and 2 of another
    let settings = Settings::builder(5)
        .range(1, 6)
        .rule(NumberPool::new().spread([3, 2, 0, 0, 0, 0]).over([1]).over([2]).over([3]).over([4]).over([5]).over([6]))
        .build();
    for _ in 0..50 {
        let numbers = settings.generate().unwrap().into_numbers();
        let mut of_a_kind: Vec<usize> = (1..=6).map(|x| numbers.iter().filter(|y| **y == x).count()).filter(|x| *x > 0).collect();
        of_a_kind.sort_unstable();
        assert_eq!(of_a_kind, vec![2, 3], "{:?}", numbers);
    }
}

#[test]
fn number_pool_spread_validation() {
    fn errors(number_pool: NumberPool, count: usize, no_duplicates: bool) -> Vec<String> {
        let builder = Settings::builder(count).range(1, 100).rule(number_pool);
        let builder = if no_duplicates { builder.no_duplicates() } else { builder };
        return builder.build().errors().to_vec();
    }

    assert!(errors(NumberPool::new().spread([4, 1, 0]).over(1..=13).over(14..=26).over(27..=39), 5, true).is_empty());
    //a pool that only has a spread is not empty
    assert!(errors(NumberPool::new().spread([1]).over([7]), 5, false).is_empty());

    assert_eq!(errors(NumberPool::new().over(1..=13), 5, false), ["NumberPool over can only be used after spread"]);
    assert_eq!(errors(NumberPool::new().spread([4, 1, 0]).over(1..=13).over(14..=26), 5, false),
        ["NumberPool spread [4, 1, 0] has 3 counts and 2 pools.  Call over one time for every count"]);
    assert_eq!(errors(NumberPool::new().spread([4, 1]).over(1..=13).over(14..=26).over(27..=39), 5, false),
        ["NumberPool spread [4, 1] has 2 counts and 3 pools.  Call over one time for every count"]);
    assert_eq!(errors(NumberPool::new().spread([4, 1]).over(1..=13).over(13..=26), 5, false),
        ["NumberPool spread [4, 1] pools 1 and 2 both have the number 13"]);
    assert_eq!(errors(NumberPool::new().spread([4, 1]).over([3, 20, 40]).over(15..=26), 5, false),
        ["NumberPool spread [4, 1] pools 1 and 2 both have the number 20"]);
    assert_eq!(errors(NumberPool::new().spread(Vec::new()), 5, false), ["NumberPool spread has no counts"]);
    //the counts are more than the count of the settings
    assert_eq!(errors(NumberPool::new().spread([4, 2]).over(1..=13).over(14..=26), 5, false), ["NumberPool count: 6 is greater than count: 5 "]);
    //3 different numbers can not come from a pool of 2 numbers
    assert!(errors(NumberPool::new().spread([3, 1]).over([1, 2]).over([8, 9]), 5, false).is_empty());
    assert_eq!(errors(NumberPool::new().spread([3, 1]).over([1, 2]).over([8, 9]), 5, true),
        ["NumberPool spread [3, 1] needs 3 numbers from a pool that only has 2, which is not possible with NoDuplicate"]);
    //the largest count goes in the largest pool
    assert!(errors(NumberPool::new().spread([1, 3]).over([1, 2]).over([7, 8, 9]), 5, true).is_empty());

    assert_eq!(PoolType::MinMax(1, 13).first_shared_number(&PoolType::MinMax(14, 26)), None);
    assert_eq!(PoolType::MinMax(1, 13).first_shared_number(&PoolType::MinMax(10, 26)), Some(10));
    assert_eq!(PoolType::set([5, 30]).first_shared_number(&PoolType::MinMax(14, 39)), Some(30));
    assert_eq!(PoolType::set([5, 30]).first_shared_number(&PoolType::set([6, 31])), None);
}
