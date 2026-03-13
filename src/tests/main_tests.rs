#[cfg(not(feature="rand"))]
use crate::random_trait::set_random_trait;
use crate::rules::*;
use crate::random::*;
use crate::settings::*;
use std::collections::HashSet;
use std::iter::FromIterator;
#[cfg(not(feature="rand"))]
use once_cell::sync::OnceCell;
#[cfg(not(feature="rand"))]
use std::sync::OnceLock;

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
        _ => panic!("{}", random_result)
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
        _ => panic!("{}", random_result)
    }
}

#[test]
fn uc_has_duplicate() {
    //Create 10 random numbers between 1 and 20 with duplicates numbers 
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
        _ => panic!("{}", random_result)
    }
}

#[test]
fn uc_number_range_1() {
    //Create random phone number
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NumberRange::from_map(&[(&[0, 1], 100, 999), (&[2], 1000, 9999)]))
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
        _ => panic!("{}", random_result)
    }
}

#[test]
fn uc_number_range_2() {
    //Create 10 numbers between 1 and 100, excluding numbers 20 - 29 and 51 - 75, and with no duplicates
    let random_result = random_numbers(&Settings::with_exclude_rules(&[
        Box::new(NoDuplicate{}),
        Box::new(NumberRange::all(1, 100))
    ], 10, Some(vec![Box::new(NumberRange::all(20, 29)), Box::new(NumberRange::all(51, 75))])));    
    match random_result.numbers() {
        Ok(numbers) => {
            let mut excluded_numbers = HashSet::new();
            (20..=29).for_each(|x| {excluded_numbers.insert(x);});
            (51..=75).for_each(|x| {excluded_numbers.insert(x);});
            assert!(numbers.len() == 10);
            for number in numbers {
                assert!(!excluded_numbers.contains(number), "{:?}:: should not contain: {}", numbers, number);
            }
            println!("{:?}", numbers);
        },
        _ => panic!("{}", random_result)
    }
}

#[test]
fn uc_number_pool_1() {
    //Create 10 numbers between 1 and 100 with no duplicates that doesn't contain the number 23, contains the numbers 1 and 4, and at least 3 numbers from 17, 18, 19, 20, 21
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NoDuplicate{}),
        Box::new(NumberPool::new(&[
            NumberPoolItem::new("exclude_23", &PoolType::Set(HashSet::from_iter([23])), OperatorType::Eq(0)),
            NumberPoolItem::new("must_contain_1_4", &PoolType::Set(HashSet::from_iter([1, 4])), OperatorType::Eq(2)),
            NumberPoolItem::new("some_set", &PoolType::Set(HashSet::from_iter([17, 18, 19, 20, 21])), OperatorType::Eq(3)),
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
        _ => panic!("{}", random_result)
    }        
}

#[test]
fn uc_odd_even_with_order_1() {
    //Create 5 numbers between 1 and 10 with no duplicates that are Odd, Even, Odd, Even, and Odd
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NoDuplicate{}),
        Box::new(NumberRange::all(1, 10)),
        Box::new(OddEvenByIndex::new(&[0, 2, 4], &[1,3])),
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
        _ => panic!("{}", random_result)
    }
}

#[test]
fn uc_odd_even_with_order_2() {
    //Create 5 numbers between 1 and 10 with no duplicates where first 3 numbers are Even and the last 2 numbers are Odd
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NoDuplicate{}),
        Box::new(NumberRange::all(1, 10)),
        Box::new(OddEvenByIndex::new(&[3,4], &[0, 1, 2]))
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
        _ => panic!("{}", random_result)
    }
}

#[test]
fn uc_odd_even_with_order_3() {
    //Create 5 numbers between 1 and 10 with no duplicates and the 5th number is odd
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NoDuplicate{}),
        Box::new(NumberRange::all(1, 10)),
        Box::new(OddEvenByIndex::new(&[4], &[]))
    ], 5));
    match random_result.numbers() {
        Ok(numbers) => {
            assert!(numbers.len() == 5);
            assert!(numbers.len() == numbers.iter().copied().collect::<HashSet<usize>>().len());
            assert!(numbers[4] % 2 == 1);
            assert!(numbers.len() == 5);
            println!("{:?}", numbers);
        },
        _ => panic!("{}", random_result)
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
        _ => panic!("{}", random_result)
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
        _ => panic!("{}", random_result)
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
        _ => panic!("{}", random_result)
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
        _ => panic!("{}", random_result)
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
        _ => panic!("{}", random_result)
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
        _ => panic!("{}", random_result)
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
        _ => panic!("{}", random_result)
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
        _ => panic!("{}", random_result)
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
        _ => panic!("{}", random_result)
    }
}

#[test]
fn uc_random_string_5() {
    //Create random 15 character string from upper case characters, from numbers 3 6 5, and from special characters # * 
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NumberPool::new(&[
            NumberPoolItem::new("upper_case_alpha_set", &PoolType::new(&"ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect()), OperatorType::Eq(11)),
            NumberPoolItem::new("numeric_set", &PoolType::new(&"365".chars().collect()), OperatorType::Eq(2)),
            NumberPoolItem::new("special_char_set", &PoolType::new(&"#*".chars().collect()), OperatorType::Eq(2)),         
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
        _ => panic!("{}", random_result)
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
        _ => panic!("{}", random_result)
    }
}

#[test]
fn uc_number_space_lt() {
    //Create 10 random numbers between 1 and 100 where the space between numbers is less than 3
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NumberRange::all(1, 100)),
        Box::new(NumberSpace::new(&[NumberSpaceItem::new(&OperatorType::Lt(3), 9)]))
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
        _ => panic!("{}", random_result)
    }
}

#[test]
fn uc_number_space_lte() {
    //Create 10 random numbers between 1 and 100 where the space between numbers is less than or equal to 3 
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NumberRange::all(1, 100)),
        Box::new(NumberSpace::new(&[NumberSpaceItem::new(&OperatorType::Lte(3), 9)]))
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
        _ => panic!("{}", random_result)
    }
}

#[test]
fn uc_number_space_equals() {
    //Create 10 random numbers between 1 and 100 where the space between numbers is 3 
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NumberRange::all(1, 100)),
        Box::new(NumberSpace::new(&[NumberSpaceItem::new(&OperatorType::Eq(3), 9)]))
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
        _ => panic!("{}", random_result)
    }
}

#[test]
fn uc_number_space_gte() {
    //Create 10 random numbers between 1 and 100 where the space between numbers is greater than or equal to 3 
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NumberRange::all(1, 100)),
        Box::new(NumberSpace::new(&[NumberSpaceItem::new(&OperatorType::Gte(3), 9)]))
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
        _ => panic!("{}", random_result)
    }
    println!("{}", random_result);
}

#[test]
fn uc_number_space_gt() {
    //Create 10 random numbers between 1 and 100 where the space between numbers is greater than 3
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NumberRange::all(1, 100)),
        Box::new(NumberSpace::new(&[NumberSpaceItem::new(&OperatorType::Gt(3), 9)]))
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
        _ => panic!("{}", random_result)
    }
}

#[test]
fn uc_number_space_between() {
    //Create 10 random numbers between 1 and 100 where the space between numbers is between 3 and 6
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NumberRange::all(1, 100)),
        Box::new(NumberSpace::new(&[NumberSpaceItem::new(&OperatorType::Between(3, 6), 9)]))
    ], 10));
    match random_result.numbers() {
        Ok(numbers) => {
            let mut sorted_numbers: Vec<usize> = numbers.to_vec();
            sorted_numbers.sort_unstable();

            let mut i = 1;
            while i < sorted_numbers.len() {     
                let num_space = sorted_numbers[i] - sorted_numbers[i-1];           
                assert!((3..=6).contains(&num_space), "i={}; sorted_numbers[i]={}; i-1={}; sorted_numbers[i-1]={}; num_space={}", i, sorted_numbers[i], i-1, sorted_numbers[i-1], num_space);
                i += 1;
            }

            println!("{:?}", sorted_numbers);
        },
        _ => panic!("{}", random_result)
    }
    println!("{}", random_result)
}



// #[test]
// fn num_space_type_get_num() {
//     let number_space_base: usize = 7;
//     let max: usize = 40;
//     let min: usize = 1;
//     let lt = NumberSpaceType::Lt(3).get_number(number_space_base, min, max);
//     assert!(lt >= 8 && lt < 10);
//     let lte = NumberSpaceType::Lte(3).get_number(number_space_base, min, max);
//     assert!(lte >= 8 && lte <= 10);
//     let eq = NumberSpaceType::Eq(3).get_number(number_space_base, min, max);
//     assert!(eq == 10);
//     let gt = NumberSpaceType::Gt(3).get_number(number_space_base, min, max);                 //7+3+1=11;max  -- 11,...      
//     assert!(gt >= 11 && gt <= max);
//     let gte = NumberSpaceType::Gte(3).get_number(number_space_base, min, max);               //7+3=10;max    -- 10,...
//     assert!(gte >= 10 && gte <= max);
//     let between = NumberSpaceType::Between(1, 3).get_number(number_space_base, min, max);
//     assert!(between >= 8 && between <= 10);
// }


//Is within Range

