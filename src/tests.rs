use crate::rules::RuleTrait;
use crate::rules::*;
use crate::random::*;
use crate::settings::*;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::sync::Mutex;
use once_cell::sync::Lazy;

const MAX_TRIES: usize = 2000;


static NUMBER_RANGE: Lazy<Mutex<NumberRange>> = Lazy::new(|| { Mutex::new(NumberRange::all(1, 100)) });

#[test]
fn uc_random_numbers() {
    //Create 10 random numbers between 1 and 20 (min and max numbers are inclusive)
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NumberRange::all(1, 20))
    ], 10));
    match random_result.numbers() {
        Ok(numbers) => {
            assert!(numbers.len() == 10);
            assert!(numbers.iter().all(|x| *x >= 1 && *x <= 20));
            println!("{:?}", numbers);
        },
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_no_duplicate() {
    //Create 10 random numbers between 1 and 20 with no duplicates 
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NoDuplicate{}),
        Box::new(NumberRange::all(1, 20))
    ], 10));
    match random_result.numbers() {
        Ok(numbers) => {
            assert!(numbers.len() == 10);
            assert!(numbers.len() == numbers.iter().copied().collect::<HashSet<usize>>().len());
            assert!(numbers.iter().all(|x| *x >= 1 && *x <= 20));
            println!("{:?}", numbers);
        },
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_has_duplicate() {
    //Create 10 random numbers between 1 and 20 with duplicates(or with repeating numbers) 
    let random_result = random_numbers(&Settings::with_exclude_rules(&[        
        Box::new(NumberRange::all(1, 20))
    ], 10, Some(vec![Box::new(NoDuplicate{})])));
    match random_result.numbers() {
        Ok(numbers) => {
            println!("{:?}", numbers);
            assert!(numbers.len() == 10);
            assert!(numbers.len() > numbers.iter().copied().collect::<HashSet<usize>>().len());
            assert!(numbers.iter().all(|x| *x >= 1 && *x <= 20));            
        },
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_number_range_1() {
    //Create random phone number
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NumberRange::from_map(&[(&vec![0, 1], 100, 999), (&vec![2], 1000, 9999)]))
    ], 3));
    match random_result.numbers() {
        Ok(numbers) => {
            assert!(numbers.len() == 3);
            assert!(numbers[0] >= 100 && numbers[0] <= 999);
            assert!(numbers[1] >= 100 && numbers[1] <= 999);
            assert!(numbers[2] >= 1000 && numbers[2] <= 9999);
            assert!(numbers.len() == 3);
            println!("{}-{}-{}", numbers[0], numbers[1], numbers[2]);
        },
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_number_range_2() {
    //Create 10 numbers between 1 and 100, excluding numbers 20 - 29 and 51 - 75, and with no duplicates
    let random_result = random_numbers(&Settings::with_exclude_rules(&[
        Box::new(NoDuplicate{}),
        Box::new(NumberRange::all(1, 100))
    ], 10, Some(vec![Box::new(NumberRange::all(20, 29)), Box::new(NumberRange::all(51, 75))])));
    let mut excluded_numbers = HashSet::new();
    (20..=29).for_each(|x| {excluded_numbers.insert(x);});
    (51..=75).for_each(|x| {excluded_numbers.insert(x);});
    match random_result.numbers() {
        Ok(numbers) => {
            assert!(numbers.len() == 10);
            for number in numbers {
                assert!(!excluded_numbers.contains(number), "{:?}:: should not contain: {}", numbers, number);
            }
            println!("{:?}", numbers);
        },
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_number_pool_1() {
    //Create 10 numbers between 1 and 100 with no duplicates that doesn't contain the number 23, contains the numbers 1 and 4, and at least 3 numbers from 17, 18, 19, 20, 21
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NoDuplicate{}),
        Box::new(NumberPool::new(&[
            NumberPoolItem::new("exclude_23", &PoolType::Set(HashSet::from_iter([23])), 0),
            NumberPoolItem::new("must_contain_1_4", &PoolType::Set(HashSet::from_iter([1, 4])), 2),
            NumberPoolItem::new("some_set", &PoolType::Set(HashSet::from_iter([17, 18, 19, 20, 21])), 3),
        ])),
        Box::new(NumberRange::all(1, 100)),
    ], 10));
    match random_result.numbers() {
        Ok(numbers) => {
            assert!(numbers.len() == 10);
            let mut must_contain_count = 0;
            let mut some_set_count = 0;
            for number in numbers {
                assert_ne!(*number, 23);
            }
            for number in numbers {
                if *number == 1 || *number == 4 {
                    must_contain_count += 1;
                }
            }
            assert_eq!(must_contain_count, 2);
            for number in numbers {
                if *number == 17 || *number == 18 || *number == 19 || *number == 20 || *number == 21 {
                    some_set_count += 1;
                }
            }
            assert_eq!(some_set_count, 3);
            println!("{:?}", numbers);
        },
        _ => println!("{:?}", random_result.logs())
    }        
}

#[test]
fn uc_odd_even_with_order_1() {
    //Create 5 numbers between 1 and 10 with no duplicates that are Odd, Even, Odd, Even, and Odd
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NoDuplicate{}),
        Box::new(NumberRange::all(1, 10)),
        Box::new(OddEvenByIndex::new(&vec![0, 2, 4], &vec![1,3])),
    ], 5));
    match random_result.numbers() {
        Ok(numbers) => {
            assert!(numbers.len() == 5);
            assert!(numbers.len() == numbers.iter().copied().collect::<HashSet<usize>>().len());
            assert!(numbers[0] % 2 == 1);
            assert!(numbers[1] % 2 == 0);
            assert!(numbers[2] % 2 == 1);
            assert!(numbers[3] % 2 == 0);
            assert!(numbers[4] % 2 == 1);
            assert!(numbers.len() == 5);
            println!("{:?}", numbers);
        },
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_odd_even_with_order_2() {
    //Create 5 numbers between 1 and 10 with no duplicates where first 3 numbers are Even and the last 2 numbers are Odd
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NoDuplicate{}),
        Box::new(NumberRange::all(1, 10)),
        Box::new(OddEvenByIndex::new(&vec![3,4], &vec![0, 1, 2]))
    ], 5));
    match random_result.numbers() {
        Ok(numbers) => {
            assert!(numbers.len() == 5);
            assert!(numbers.len() == numbers.iter().copied().collect::<HashSet<usize>>().len());
            assert!(numbers[0] % 2 == 0);
            assert!(numbers[1] % 2 == 0);
            assert!(numbers[2] % 2 == 0);
            assert!(numbers[3] % 2 == 1);
            assert!(numbers[4] % 2 == 1);
            assert!(numbers.len() == 5);
            println!("{:?}", numbers);
        },
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_odd_even_with_order_3() {
    //Create 5 numbers between 1 and 10 with no duplicates and the 5th number is odd
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NoDuplicate{}),
        Box::new(NumberRange::all(1, 10)),
        Box::new(OddEvenByIndex::new(&vec![4], &vec![]))
    ], 5));
    match random_result.numbers() {
        Ok(numbers) => {
            assert!(numbers.len() == 5);
            assert!(numbers.len() == numbers.iter().copied().collect::<HashSet<usize>>().len());
            assert!(numbers[4] % 2 == 1);
            assert!(numbers.len() == 5);
            println!("{:?}", numbers);
        },
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_odd_even_1() {
    //Create 10 numbers between 1 and 20 with no duplicates that has 5 odd and 5 even numbers
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NoDuplicate{}),
        Box::new(NumberRange::all(1, 20)),
        Box::new(OddEven::new(5, 5))
    ], 10));
    match random_result.numbers() {
        Ok(numbers) => {
            assert!(numbers.len() == 10);
            assert!(numbers.len() == numbers.iter().copied().collect::<HashSet<usize>>().len());
            assert_eq!(numbers.len(), 10);
            let odd_even = OddEven::from_numbers(numbers);
            let mut odd: usize = 0;
            let mut even: usize = 0;
            numbers.iter().for_each(|x| {
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
            println!("{:?}", numbers);
        },
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_odd_even_2() {
    //Create 10 numbers between 1 and 20 that has all even numbers
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NumberRange::all(1, 20)),
        Box::new(OddEven::new(0, 10))
    ], 10));
    match random_result.numbers() {
        Ok(numbers) => {
            assert_eq!(numbers.len(), 10);
            let odd_even = OddEven::from_numbers(numbers);
            let mut odd: usize = 0;
            let mut even: usize = 0;
            numbers.iter().for_each(|x| {
                if x % 2_usize == 0 {
                    even += 1
                } else {
                    odd += 1
                }
            });
            assert_eq!(odd_even.odd(), 0);
            assert_eq!(odd_even.even(), 10);
            assert_eq!(odd, 0);
            assert_eq!(even, 10);
            println!("{:?}", numbers);
        },
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_seq_1() {
    //Create 5 numbers between 1 and 10 with no duplicates that are all nonsequential
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NoDuplicate{}),
        Box::new(NumberRange::all(1, 10)),
        Box::new(Sequential::new(5, &[]))
    ], 5));
    match random_result.numbers() {
        Ok(numbers) => {
            assert!(numbers.len() == 5);
            assert!(numbers.len() == numbers.iter().copied().collect::<HashSet<usize>>().len());
            for number in numbers.iter().copied() {
                for number2 in numbers.iter().copied() {
                    assert!((number - 1) != number2);
                    assert!((number + 1) != number2);
                }
            }
            println!("{:?}", numbers);
        },
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_seq_2() {
    //Create 5 numbers between 1 and 10 with no duplicates that has 3 nonsequential and has 1 sequential set of 2 numbers
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NoDuplicate{}),
        Box::new(NumberRange::all(1, 10)),
        Box::new(Sequential::new(3, &[2]))
    ], 5));
    match random_result.numbers() {
        Ok(numbers) => {
            assert!(numbers.len() == 5);
            assert!(numbers.len() == numbers.iter().copied().collect::<HashSet<usize>>().len());
            //TODO Test
            println!("{:?}", numbers);
        },
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_seq_3() {
    //Create 5 numbers between 1 and 10 with no duplicates that has 1 nonsequential number and has 2 sequential sets of 2 numbers a piece
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NoDuplicate{}),
        Box::new(NumberRange::all(1, 10)),
        Box::new(Sequential::new(1, &[2, 2]))
    ], 5));
    match random_result.numbers() {
        Ok(numbers) => {
            assert!(numbers.len() == 5);
            assert!(numbers.len() == numbers.iter().copied().collect::<HashSet<usize>>().len());
            //TODO Test
            println!("{:?}", numbers);
        },
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_random_string_1() {
    //Create a random 20 character string with no special characters
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NumberPool::alphanumeric(20, false))
    ], 20));
    match random_result.string(true) {
        Ok(s) => {
            assert!(s.len() == 20);
            println!("{}", s)
        }
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_random_string_2() {
    //Create a random 22 character string with special characters
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NumberPool::alphanumeric(20, true))
    ], 20));
    match random_result.string(true) {
        Ok(s) => {
            assert!(s.len() == 20);
            println!("{}", s)
        }
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_random_string_3() {
    //Create a random 20 character string with 10 letters, 10 numbers, no special characters, and no duplicates
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NoDuplicate{}),
        Box::new(NumberPool::alphanumeric_specs(10, 10, 0))
    ], 20));
    match random_result.string(true) {
        Ok(s) => {
            assert!(s.len() == 20);
            assert!(s.len() == s.chars().collect::<HashSet<char>>().len());
            let mut number_count = 0;
            let mut char_count = 0;
            let mut special_char_count = 0;
            for c in s.chars() {
                if c.is_numeric() {
                    number_count += 1;
                } else if NP_SPECIAL_CHAR_SET.lock().unwrap().contains(&c) {
                    special_char_count += 1;
                } else if NP_ALPHABET_SET.lock().unwrap().contains(&c) {
                    char_count += 1;
                }
            }
            assert_eq!(10, number_count);
            assert_eq!(10, char_count);
            assert_eq!(0, special_char_count);
            println!("{}", s)
        },
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_random_string_4() {
    //Create a random 25 character string with 10 letters, 10 numbers, and 5 special characters and not duplicates
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NoDuplicate{}),
        Box::new(NumberPool::alphanumeric_specs(10, 10, 5))
    ], 25));
    match random_result.string(true) {
        Ok(s) => {
            assert!(s.len() == 25);
            assert!(s.len() == s.chars().collect::<HashSet<char>>().len());
            let mut number_count = 0;
            let mut char_count = 0;
            let mut special_char_count = 0;
            for c in s.chars() {
                if c.is_numeric() {
                    number_count += 1;
                } else if NP_SPECIAL_CHAR_SET.lock().unwrap().contains(&c) {
                    special_char_count += 1;
                } else if NP_ALPHABET_SET.lock().unwrap().contains(&c) {
                    char_count += 1;
                }
            }
            assert_eq!(10, number_count);
            assert_eq!(10, char_count);
            assert_eq!(5, special_char_count);
            println!("{}", s)
        },
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_random_string_5() {
    //Create random 15 character string from upper case characters, from numbers 3 6 5, and from special characters # * 
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NumberPool::new(&[
            NumberPoolItem::new("upper_case_alpha_set", &PoolType::new(&"ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect()), 11),
            NumberPoolItem::new("numeric_set", &PoolType::new(&"365".chars().collect()), 2),
            NumberPoolItem::new("special_char_set", &PoolType::new(&"#*".chars().collect()), 2),         
        ]))
    ], 15));
    match random_result.string(true) {
        Ok(s) => {
            assert!(s.len() == 15);
            let mut number_count = 0;
            let mut char_count = 0;
            let mut special_char_count = 0;
            for c in s.chars() {
                if c.is_numeric() {
                    number_count += 1;
                } else if NP_SPECIAL_CHAR_SET.lock().unwrap().contains(&c) {
                    special_char_count += 1;
                } else if NP_ALPHABET_SET.lock().unwrap().contains(&c) {
                    char_count += 1;
                }
            }
            assert_eq!(2, number_count);
            assert_eq!(11, char_count);
            assert_eq!(2, special_char_count);
            println!("{}", s)
        },
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_random_string_6() {    
    //Create random license number that starts with 2 apha-characters followed by 7 numeric characters. Also excludes "AB1234567" and "CB1234567" because they are already assigned to someone.
    let uc_alpha_set: HashSet<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let random_result = random_numbers(&Settings::new(&[
        Box::new(ExcludeNumberSets::new_string(&HashSet::from_iter(vec![String::from("AB1234567"), String::from("CB1234567")]))),
        Box::new(NumberPoolByIndex::new(vec![
            NumberPoolItemByIndex::new("upper_case_alpha_set", &PoolType::new(&uc_alpha_set.clone()), &HashSet::from_iter(vec![0, 1])),
            NumberPoolItemByIndex::new("numeric_set", &PoolType::new(&"1234567890".chars().collect()), &HashSet::from_iter(vec![2, 3, 4, 5, 6, 7, 8])),         
        ]))
    ], 9));
    match random_result.string(false) {
        Ok(s) => {
            assert!(s.len() == 9);
            let mut chars = s.chars();
            assert!(uc_alpha_set.contains(&chars.next().unwrap()));
            assert!(uc_alpha_set.contains(&chars.next().unwrap()));
            assert!(chars.next().unwrap().is_numeric());
            assert!(chars.next().unwrap().is_numeric());
            assert!(chars.next().unwrap().is_numeric());
            assert!(chars.next().unwrap().is_numeric());
            assert!(chars.next().unwrap().is_numeric());
            assert!(chars.next().unwrap().is_numeric());
            assert!(chars.next().unwrap().is_numeric());
            println!("{}", s)
        },
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_number_space_lt() {
    //Create 10 random numbers between 1 and 100 where the space between numbers is less than 3
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NumberRange::all(1, 100)),
        Box::new(NumberSpace::new(&vec![NumberSpaceItem::new(&NumberSpaceType::Lt(3), 10)]))
    ], 10));
    match random_result.numbers() {
        Ok(numbers) => {
            let mut sorted_numbers: Vec<usize> = numbers.to_vec();
            sorted_numbers.sort_unstable();

            let mut i = 1;
            while i < sorted_numbers.len() {
                let num_space = sorted_numbers[i] - sorted_numbers[i-1];                 
                assert!(num_space < 3, "i={}; sorted_numbers[i]={}; i-1={}; sorted_numbers[i-1]={}; num_space={}", i, sorted_numbers[i], i-1, sorted_numbers[i-1], num_space);
                i += 1;
            }

            println!("{:?}", sorted_numbers);
        },
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_number_space_lte() {
    //Create 10 random numbers between 1 and 100 where the space between numbers is less than or equal to 3 
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NumberRange::all(1, 100)),
        Box::new(NumberSpace::new(&vec![NumberSpaceItem::new(&NumberSpaceType::Lte(3), 10)]))
    ], 10));
    match random_result.numbers() {
        Ok(numbers) => {
            let mut sorted_numbers: Vec<usize> = numbers.to_vec();
            sorted_numbers.sort_unstable();

            let mut i = 1;
            while i < sorted_numbers.len() {  
                let num_space = sorted_numbers[i] - sorted_numbers[i-1];               
                assert!(num_space <= 3, "i={}; sorted_numbers[i]={}; i-1={}; sorted_numbers[i-1]={}; num_space={}", i, sorted_numbers[i], i-1, sorted_numbers[i-1], num_space);
                i += 1;
            }

            println!("{:?}", sorted_numbers);
        },
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_number_space_equals() {
    //Create 10 random numbers between 1 and 100 where the space between numbers is 3 
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NumberRange::all(1, 100)),
        Box::new(NumberSpace::new(&vec![NumberSpaceItem::new(&NumberSpaceType::Eq(3), 10)]))
    ], 10));
    match random_result.numbers() {
        Ok(numbers) => {
            let mut sorted_numbers: Vec<usize> = numbers.to_vec();
            sorted_numbers.sort_unstable();

            let mut i = 1;
            while i < sorted_numbers.len() { 
                let num_space = sorted_numbers[i] - sorted_numbers[i-1];                
                assert!(num_space == 3, "i={}; sorted_numbers[i]={}; i-1={}; sorted_numbers[i-1]={}; num_space={}", i, sorted_numbers[i], i-1, sorted_numbers[i-1], num_space);
                i += 1;
            }

            println!("{:?}", sorted_numbers);
        },
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_number_space_gte() {
    //Create 10 random numbers between 1 and 100 where the space between numbers is greater than or equal to 3 
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NumberRange::all(1, 100)),
        Box::new(NumberSpace::new(&vec![NumberSpaceItem::new(&NumberSpaceType::Gte(3), 10)]))
    ], 10));
    match random_result.numbers() {
        Ok(numbers) => {
            let mut sorted_numbers: Vec<usize> = numbers.to_vec();
            sorted_numbers.sort_unstable();

            let mut i = 1;
            while i < sorted_numbers.len() {   
                let num_space = sorted_numbers[i] - sorted_numbers[i-1];              
                assert!(num_space >= 3, "i={}; sorted_numbers[i]={}; i-1={}; sorted_numbers[i-1]={}; num_space={}", i, sorted_numbers[i], i-1, sorted_numbers[i-1], num_space);
                i += 1;
            }

            println!("{:?}", sorted_numbers);
        },
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_number_space_gt() {
    //Create 10 random numbers between 1 and 100 where the space between numbers is greater than 3
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NumberRange::all(1, 100)),
        Box::new(NumberSpace::new(&vec![NumberSpaceItem::new(&NumberSpaceType::Gt(3), 10)]))
    ], 10));
    match random_result.numbers() {
        Ok(numbers) => {
            let mut sorted_numbers: Vec<usize> = numbers.to_vec();
            sorted_numbers.sort_unstable();

            let mut i = 1;
            while i < sorted_numbers.len() {       
                let num_space = sorted_numbers[i] - sorted_numbers[i-1];          
                assert!(num_space > 3, "i={}; sorted_numbers[i]={}; i-1={}; sorted_numbers[i-1]={}; num_space={}", i, sorted_numbers[i], i-1, sorted_numbers[i-1], num_space);
                i += 1;
            }

            println!("{:?}", sorted_numbers);
        },
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn uc_number_space_between() {
    //Create 10 random numbers between 1 and 100 where the space between numbers is greater than 3
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NumberRange::all(1, 100)),
        Box::new(NumberSpace::new(&vec![NumberSpaceItem::new(&NumberSpaceType::Between(3, 6), 10)]))
    ], 10));
    match random_result.numbers() {
        Ok(numbers) => {
            let mut sorted_numbers: Vec<usize> = numbers.to_vec();
            sorted_numbers.sort_unstable();

            let mut i = 1;
            while i < sorted_numbers.len() {     
                let num_space = sorted_numbers[i] - sorted_numbers[i-1];           
                assert!(num_space >= 3 && num_space <= 6, "i={}; sorted_numbers[i]={}; i-1={}; sorted_numbers[i-1]={}; num_space={}", i, sorted_numbers[i], i-1, sorted_numbers[i-1], num_space);
                i += 1;
            }

            println!("{:?}", sorted_numbers);
        },
        _ => println!("{:?}", random_result.logs())
    }
}

#[test]
fn odd_even_with_order_2() {
    let random_result = random_numbers(&Settings::new(&[
        Box::new(OddEvenByIndex::new(&vec![0, 2, 4], &vec![1,3]))
    ], 5));
    match random_result.numbers() {
        Ok(s) => println!("{:?}", s),
        _ => println!("{:?}", random_result.logs())
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
        let current_data_shared_data: HashMap<String, HashMap<String, MapAnyValue>> = HashMap::new();
        let current_data = CurrentData::new(&current_data_numbers, &current_data_settings, &current_data_shared_data);
        let seq_numbers = vec![item.1.clone(), Sequential::from_numbers(&current_data, false)];
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
        ("[(0, (1, 100)), (1, (1, 100)), (2, (1, 100)), (3, (1, 100)), (4, (1, 100))]", NumberRange::from_map(&[(&vec![0,1,2,3,4], 1 , 100)]), &[1, 99, 4, 6, 7], Ok(()), Ok(())),
        ("[(0, (1, 1)), (1, (99, 99)), (2, (4, 4)), (3, (6, 6)), (4, (7, 7))]", NumberRange::from_map(&[(&vec![0], 1,1), (&vec![1], 99,99), (&vec![2], 4,4), (&vec![3], 6,6), (&vec![4], 7,7)]), &[1, 99, 4, 6, 7], Ok(()), Ok(())),

        ("[(0, (1, 100))]", NumberRange::all(1, 100), &[1, 99, 4, 6, 101], Err((IsWithinErrorType::Regular, "Exclude: false - Selected number 101 at index 4 is not within range of min: 1 and max: 100. Numbers:[1, 99, 4, 6, 101].  Map Index:0".to_owned())), Err("Exclude: false - Selected number 101 at index 4 is not within range of min: 1 and max: 100. Numbers:[1, 99, 4, 6, 101].  Map Index:0".to_owned())),
        ("[(0, (1, 100)), (1, (1, 100)), (2, (1, 100)), (3, (1, 100)), (4, (1, 100))]", NumberRange::from_map(&[(&vec![0,1,2,3,4], 1 , 100)]), &[1, 100, 4, 6, 7], Ok(()), Ok(())),
        ("[(0, (1, 1)), (1, (99, 99)), (2, (4, 4)), (3, (6, 6)), (4, (7, 7))]", NumberRange::from_map(&[(&vec![0], 1,1), (&vec![1], 99,99), (&vec![2], 4,4), (&vec![3], 6,6), (&vec![4], 7,7)]), &[1, 99, 5, 6, 7], Err((IsWithinErrorType::Regular, "Exclude: false - Selected number 5 at index 2 is not within range of min: 4 and max: 4. Numbers:[1, 99, 5, 6, 7].  Map Index:2".to_owned())), Err("Exclude: false - Selected number 5 at index 2 is not within range of min: 4 and max: 4. Numbers:[1, 99, 5, 6, 7].  Map Index:2".to_owned())),
    ];
    for (main_idx, item) in data.iter().enumerate() {
        let current_data_numbers: Vec<usize> = item.2.to_vec();
        let current_data_settings: Settings = Settings::new(&[], item.2.len());
        let current_data_shared_data: HashMap<String, HashMap<String, MapAnyValue>> = HashMap::new();
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
            item.1.is_match(&CurrentData::new(&rand_result.numbers().unwrap(), &Settings::new(&[], item.2.len()), &HashMap::new())),
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
        let odd_evens = vec![item.1, OddEven::from_numbers(item.2)];
        for (idx, odd_even) in odd_evens.iter().enumerate() {
            let current_data_numbers: Vec<usize> = item.2.to_vec();
        let current_data_settings: Settings = Settings::new(&[], item.2.len());
        let current_data_shared_data: HashMap<String, HashMap<String, MapAnyValue>> = HashMap::new();
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
        NumberRange::from_map(&[(&vec![0], 1, 1), (&vec![1], 99, 99), (&vec![2], 4, 4), (&vec![3], 6, 6), (&vec![4], 7, 7)]),
        NumberRange::from_map(&[(&vec![0], 20,20), (&vec![1], 55,55), (&vec![2], 4,10), (&vec![3], 71,75), (&vec![4], 47,47)]),
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
            item.is_match(&CurrentData::new(&rand_result.numbers().unwrap(), &Settings::new(&[], item.len()), &HashMap::new())),
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
            NumberPoolItem::new("exclude_set", &PoolType::Set(exclude_set.clone()), 0),
            NumberPoolItem::new("all_set", &PoolType::Set(all_set.clone()), all_set.len()),
            NumberPoolItem::new("some_set", &PoolType::Set(some_set.clone()), 3),
        ])),
        Box::new(NUMBER_RANGE.lock().unwrap().clone()),
    ];
    let settings_1 = Settings::new(&rules_1, 10);
    let rules_2: Vec<Box<dyn RuleTrait>> = vec![
        Box::new(NumberPool::new(&[
            NumberPoolItem::new("exclude_set", &PoolType::MinMax(23, 23), 0),
            NumberPoolItem::new("all_set", &PoolType::Set(all_set.clone()), all_set.len()),
            NumberPoolItem::new("some_set", &PoolType::MinMax(17, 21), 3),
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
            NumberPoolItem::new("exclude_set", &PoolType::Set(exclude_set.clone()), 0),
            NumberPoolItem::new("all_set", &PoolType::Set(all_set.clone()), all_set.len()),
        ])),
        Box::new(NUMBER_RANGE.lock().unwrap().clone()),
    ];
    let settings_1 = Settings::new(&rules_1, 10);
    let rules_2: Vec<Box<dyn RuleTrait>> = vec![
        Box::new(NumberPool::new(&[
            NumberPoolItem::new("exclude_set", &PoolType::MinMax(11, 100), 0),
            NumberPoolItem::new("all_set", &PoolType::MinMax(1, 10), all_set.len()),
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
    let current_data_shared_data_1: HashMap<String, HashMap<String, MapAnyValue>> = HashMap::new();
    let current_data_1 = CurrentData::new(&current_data_numbers_1, &current_data_settings_1, &current_data_shared_data_1);

    let current_data_numbers_2: Vec<usize> = vec![25,24,23,22,21];
    let current_data_settings_2: Settings = Settings::new(&[], current_data_numbers_2.len());
    let current_data_shared_data_2: HashMap<String, HashMap<String, MapAnyValue>> = HashMap::new();
    let current_data_2 = CurrentData::new(&current_data_numbers_2, &current_data_settings_2, &current_data_shared_data_2);

    let current_data_numbers_3: Vec<usize> = vec![35,35,33,32,31];
    let current_data_settings_3: Settings = Settings::new(&[], current_data_numbers_3.len());
    let current_data_shared_data_3: HashMap<String, HashMap<String, MapAnyValue>> = HashMap::new();
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
fn num_space_type_get_num() {
    let number_space_base: usize = 7;
    let max: usize = 40;
    let lt = NumberSpaceType::Lt(3).get_number(number_space_base, max);
    assert!(lt >= 8 && lt < 10);
    let lte = NumberSpaceType::Lte(3).get_number(number_space_base, max);
    assert!(lte >= 8 && lte <= 10);
    let eq = NumberSpaceType::Eq(3).get_number(number_space_base, max);
    assert!(eq == 10);
    let gt = NumberSpaceType::Gt(3).get_number(number_space_base, max);                 //7+3+1=11;max  -- 11,...      
    assert!(gt >= 11 && gt <= max);
    let gte = NumberSpaceType::Gte(3).get_number(number_space_base, max);               //7+3=10;max    -- 10,...
    assert!(gte >= 10 && gte <= max);
    let between = NumberSpaceType::Between(1, 3).get_number(number_space_base, max);
    assert!(between >= 8 && between <= 10);
}


//Is within Range

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
    assert_eq!(Ok(()), rule.is_within_range(&CurrentData::new(&vec![20, 35, 4, 11, 12], &Settings::new(&[], 5), &HashMap::new())))
}