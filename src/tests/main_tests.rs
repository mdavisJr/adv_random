use crate::rules::*;
use crate::settings::*;
use std::collections::HashSet;

#[test]
fn uc_random_numbers() {
    //Create 10 random numbers between 1 and 20 (min and max numbers are inclusive)
    let random_result = Settings::builder(10)
        .rule(NumberRange::all(1, 20))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let numbers = random_numbers.numbers();
            assert!(numbers.len() == 10);
            assert!(numbers.iter().all(|x| *x >= 1 && *x <= 20));
            println!("{:?}", numbers);
        },
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn uc_no_duplicate() {
    //Create 10 random numbers between 1 and 20 with no duplicates 
    let random_result = Settings::builder(10)
        .rule(NoDuplicate)
        .rule(NumberRange::all(1, 20))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let numbers = random_numbers.numbers();
            assert!(numbers.len() == 10);
            assert!(numbers.len() == numbers.iter().copied().collect::<HashSet<usize>>().len());
            assert!(numbers.iter().all(|x| *x >= 1 && *x <= 20));
            println!("{:?}", numbers);
        },
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn uc_has_duplicate() {
    //Create 10 random numbers between 1 and 20 with duplicates numbers 
    let random_result = Settings::builder(10)
        .rule(NumberRange::all(1, 20))
        .exclude(NoDuplicate)
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let numbers = random_numbers.numbers();
            println!("{:?}", numbers);
            assert!(numbers.len() == 10);
            assert!(numbers.len() > numbers.iter().copied().collect::<HashSet<usize>>().len());
            assert!(numbers.iter().all(|x| *x >= 1 && *x <= 20));            
        },
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn uc_number_range_1() {
    //Create random phone number
    let random_result = Settings::builder(3)
        .rule(NumberRange::new().at([0, 1], 100, 999).at([2], 1000, 9999))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let numbers = random_numbers.numbers();
            assert!(numbers.len() == 3);
            assert!(numbers[0] >= 100 && numbers[0] <= 999);
            assert!(numbers[1] >= 100 && numbers[1] <= 999);
            assert!(numbers[2] >= 1000 && numbers[2] <= 9999);
            assert!(numbers.len() == 3);
            println!("{}-{}-{}", numbers[0], numbers[1], numbers[2]);
        },
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn uc_number_range_2() {
    //Create 10 numbers between 1 and 100, excluding numbers 20 - 29 and 51 - 75, and with no duplicates
    let random_result = Settings::builder(10)
        .rule(NoDuplicate)
        .rule(NumberRange::all(1, 100))
        .exclude(NumberRange::all(20, 29))
        .exclude(NumberRange::all(51, 75))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let numbers = random_numbers.numbers();
            let mut excluded_numbers = HashSet::new();
            (20..=29).for_each(|x| {excluded_numbers.insert(x);});
            (51..=75).for_each(|x| {excluded_numbers.insert(x);});
            assert!(numbers.len() == 10);
            for number in numbers {
                assert!(!excluded_numbers.contains(number), "{:?}:: should not contain: {}", numbers, number);
            }
            println!("{:?}", numbers);
        },
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn uc_number_pool_1() {
    //Create 10 numbers between 1 and 100 with no duplicates that doesn't contain the number 23, contains the numbers 1 and 4, and at least 3 numbers from 17, 18, 19, 20, 21
    let random_result = Settings::builder(10)
        .rule(NoDuplicate)
        .rule(NumberPool::new()
            .none_of([23])
            .exactly(2, [1, 4])
            .exactly(3, [17, 18, 19, 20, 21]))
        .rule(NumberRange::all(1, 100))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let numbers = random_numbers.numbers();
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
        Err(e) => panic!("{:?}", e)
    }        
}

#[test]
fn uc_odd_even_with_order_1() {
    //Create 5 numbers between 1 and 10 with no duplicates that are Odd, Even, Odd, Even, and Odd
    let random_result = Settings::builder(5)
        .rule(NoDuplicate)
        .rule(NumberRange::all(1, 10))
        .rule(OddEvenByIndex::new().odd([0, 2, 4]).even([1, 3]))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let numbers = random_numbers.numbers();
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
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn uc_odd_even_with_order_2() {
    //Create 5 numbers between 1 and 10 with no duplicates where first 3 numbers are Even and the last 2 numbers are Odd
    let random_result = Settings::builder(5)
        .rule(NoDuplicate)
        .rule(NumberRange::all(1, 10))
        .rule(OddEvenByIndex::new().odd([3, 4]).even([0, 1, 2]))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let numbers = random_numbers.numbers();
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
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn uc_odd_even_with_order_3() {
    //Create 5 numbers between 1 and 10 with no duplicates and the 5th number is odd
    let random_result = Settings::builder(5)
        .rule(NoDuplicate)
        .rule(NumberRange::all(1, 10))
        .rule(OddEvenByIndex::new().odd([4]))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let numbers = random_numbers.numbers();
            assert!(numbers.len() == 5);
            assert!(numbers.len() == numbers.iter().copied().collect::<HashSet<usize>>().len());
            assert!(numbers[4] % 2 == 1);
            assert!(numbers.len() == 5);
            println!("{:?}", numbers);
        },
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn uc_odd_even_1() {
    //Create 10 numbers between 1 and 20 with no duplicates that has 5 odd and 5 even numbers
    let random_result = Settings::builder(10)
        .rule(NoDuplicate)
        .rule(NumberRange::all(1, 20))
        .rule(OddEven::new().odd(5).even(5))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let numbers = random_numbers.numbers();
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
            assert_eq!(odd_even.odd_count(), 5);
            assert_eq!(odd_even.even_count(), 5);
            assert_eq!(odd, 5);
            assert_eq!(even, 5);
            println!("{:?}", numbers);
        },
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn uc_odd_even_2() {
    //Create 10 numbers between 1 and 20 that has all even numbers
    let random_result = Settings::builder(10)
        .rule(NumberRange::all(1, 20))
        .rule(OddEven::new().odd(0).even(10))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let numbers = random_numbers.numbers();
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
            assert_eq!(odd_even.odd_count(), 0);
            assert_eq!(odd_even.even_count(), 10);
            assert_eq!(odd, 0);
            assert_eq!(even, 10);
            println!("{:?}", numbers);
        },
        Err(e) => panic!("{:?}", e)
    }
}

//Returns how many numbers are not in a sequence, and how long each sequence is from shortest to longest
fn sequence_counts(numbers: &[usize]) -> (usize, Vec<usize>) {
    let mut sorted_numbers = numbers.to_vec();
    sorted_numbers.sort_unstable();
    let mut seq_counts: Vec<usize> = Vec::new();
    let mut seq_count = 1;
    for i in 1..sorted_numbers.len() {
        if sorted_numbers[i] == sorted_numbers[i - 1] + 1 {
            seq_count += 1;
        } else {
            if seq_count > 1 {
                seq_counts.push(seq_count);
            }
            seq_count = 1;
        }
    }
    if seq_count > 1 {
        seq_counts.push(seq_count);
    }
    seq_counts.sort_unstable();
    return (sorted_numbers.len() - seq_counts.iter().sum::<usize>(), seq_counts);
}

#[test]
fn uc_seq_1() {
    //Create 5 numbers between 1 and 10 with no duplicates that are all nonsequential
    let random_result = Settings::builder(5)
        .rule(NoDuplicate)
        .rule(NumberRange::all(1, 10))
        .rule(Sequential::new().singles(5))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let numbers = random_numbers.numbers();
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
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn uc_seq_2() {
    //Create 5 numbers between 1 and 10 with no duplicates that has 3 nonsequential and has 1 sequential set of 2 numbers
    let random_result = Settings::builder(5)
        .rule(NoDuplicate)
        .rule(NumberRange::all(1, 10))
        .rule(Sequential::new().singles(3).in_a_row(2))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let numbers = random_numbers.numbers();
            assert!(numbers.len() == 5);
            assert!(numbers.len() == numbers.iter().copied().collect::<HashSet<usize>>().len());
            assert_eq!(sequence_counts(numbers), (3, vec![2]), "{:?}", numbers);
            println!("{:?}", numbers);
        },
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn uc_seq_3() {
    //Create 5 numbers between 1 and 10 with no duplicates that has 1 nonsequential number and has 2 sequential sets of 2 numbers a piece
    let random_result = Settings::builder(5)
        .rule(NoDuplicate)
        .rule(NumberRange::all(1, 10))
        .rule(Sequential::new().singles(1).in_a_row(2).in_a_row(2))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let numbers = random_numbers.numbers();
            assert!(numbers.len() == 5);
            assert!(numbers.len() == numbers.iter().copied().collect::<HashSet<usize>>().len());
            assert_eq!(sequence_counts(numbers), (1, vec![2, 2]), "{:?}", numbers);
            println!("{:?}", numbers);
        },
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn uc_random_string_1() {
    //Create a random 20 character string with no special characters
    let random_result = Settings::builder(20)
        .rule(NumberPool::alphanumeric(20))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let s = random_numbers.shuffled_string().unwrap();
            assert!(s.len() == 20);
            println!("{}", s)
        }
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn uc_random_string_2() {
    //Create a random 22 character string with special characters
    let random_result = Settings::builder(20)
        .rule(NumberPool::alphanumeric(20).with_special_characters())
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let s = random_numbers.shuffled_string().unwrap();
            assert!(s.len() == 20);
            println!("{}", s)
        }
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn uc_random_string_3() {
    //Create a random 20 character string with 10 letters, 10 numbers, no special characters, and no duplicates
    let random_result = Settings::builder(20)
        .rule(NoDuplicate)
        .rule(NumberPool::alphanumeric(20).digits(10))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let s = random_numbers.shuffled_string().unwrap();
            assert!(s.len() == 20);
            assert!(s.len() == s.chars().collect::<HashSet<char>>().len());
            let mut number_count = 0;
            let mut char_count = 0;
            let mut special_char_count = 0;
            for c in s.chars() {
                if c.is_numeric() {
                    number_count += 1;
                } else if NP_SPECIAL_CHAR_SET.contains(&c) {
                    special_char_count += 1;
                } else if NP_ALPHABET_SET.contains(&c) {
                    char_count += 1;
                }
            }
            assert_eq!(10, number_count);
            assert_eq!(10, char_count);
            assert_eq!(0, special_char_count);
            println!("{}", s)
        },
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn uc_random_string_4() {
    //Create a random 25 character string with 10 letters, 10 numbers, and 5 special characters and not duplicates
    let random_result = Settings::builder(25)
        .rule(NoDuplicate)
        .rule(NumberPool::alphanumeric(25).digits(10).special_characters(5))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let s = random_numbers.shuffled_string().unwrap();
            assert!(s.len() == 25);
            assert!(s.len() == s.chars().collect::<HashSet<char>>().len());
            let mut number_count = 0;
            let mut char_count = 0;
            let mut special_char_count = 0;
            for c in s.chars() {
                if c.is_numeric() {
                    number_count += 1;
                } else if NP_SPECIAL_CHAR_SET.contains(&c) {
                    special_char_count += 1;
                } else if NP_ALPHABET_SET.contains(&c) {
                    char_count += 1;
                }
            }
            assert_eq!(10, number_count);
            assert_eq!(10, char_count);
            assert_eq!(5, special_char_count);
            println!("{}", s)
        },
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn uc_random_string_5() {
    //Create random 15 character string from upper case characters, from numbers 3 6 5, and from special characters # * 
    let random_result = Settings::builder(15)
        .rule(NumberPool::new()
            .exactly(11, "ABCDEFGHIJKLMNOPQRSTUVWXYZ")
            .exactly(2, "365")
            .exactly(2, "#*"))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let s = random_numbers.shuffled_string().unwrap();
            assert!(s.len() == 15);
            let mut number_count = 0;
            let mut char_count = 0;
            let mut special_char_count = 0;
            for c in s.chars() {
                if c.is_numeric() {
                    number_count += 1;
                } else if NP_SPECIAL_CHAR_SET.contains(&c) {
                    special_char_count += 1;
                } else if NP_ALPHABET_SET.contains(&c) {
                    char_count += 1;
                }
            }
            assert_eq!(2, number_count);
            assert_eq!(11, char_count);
            assert_eq!(2, special_char_count);
            println!("{}", s)
        },
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn uc_random_string_6() {    
    //Create random license number that starts with 2 apha-characters followed by 7 numeric characters. Also excludes "AB1234567" and "CB1234567" because they are already assigned to someone.
    let uc_alpha_set: HashSet<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let random_result = Settings::builder(9)
        .rule(ExcludeNumberSets::new().string("AB1234567").string("CB1234567"))
        .rule(NumberPoolByIndex::new()
            .at([0, 1], "ABCDEFGHIJKLMNOPQRSTUVWXYZ")
            .at(2..=8, "1234567890"))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let s = random_numbers.string().unwrap();
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
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn uc_number_space_lt() {
    //Create 10 random numbers between 1 and 100 where the space between numbers is less than 3
    let random_result = Settings::builder(10)
        .rule(NumberRange::all(1, 100))
        .rule(NumberSpace::new().run(OperatorType::Lt(3), 9))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let numbers = random_numbers.numbers();
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
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn uc_number_space_lte() {
    //Create 10 random numbers between 1 and 100 where the space between numbers is less than or equal to 3 
    let random_result = Settings::builder(10)
        .rule(NumberRange::all(1, 100))
        .rule(NumberSpace::new().run(OperatorType::Lte(3), 9))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let numbers = random_numbers.numbers();
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
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn uc_number_space_equals() {
    //Create 10 random numbers between 1 and 100 where the space between numbers is 3 
    let random_result = Settings::builder(10)
        .rule(NumberRange::all(1, 100))
        .rule(NumberSpace::new().run(OperatorType::Eq(3), 9))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let numbers = random_numbers.numbers();
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
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn uc_number_space_gte() {
    //Create 10 random numbers between 1 and 100 where the space between numbers is greater than or equal to 3 
    let random_result = Settings::builder(10)
        .rule(NumberRange::all(1, 100))
        .rule(NumberSpace::new().run(OperatorType::Gte(3), 9))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let numbers = random_numbers.numbers();
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
        Err(e) => panic!("{:?}", e)
    }
    println!("{:?}", random_result);
}

#[test]
fn uc_number_space_gt() {
    //Create 10 random numbers between 1 and 100 where the space between numbers is greater than 3
    let random_result = Settings::builder(10)
        .rule(NumberRange::all(1, 100))
        .rule(NumberSpace::new().run(OperatorType::Gt(3), 9))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let numbers = random_numbers.numbers();
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
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn uc_number_space_between() {
    //Create 10 random numbers between 1 and 100 where the space between numbers is between 3 and 6
    let random_result = Settings::builder(10)
        .rule(NumberRange::all(1, 100))
        .rule(NumberSpace::new().run(OperatorType::Between(3, 6), 9))
        .build()
        .generate();
    match &random_result {
        Ok(random_numbers) => {
            let numbers = random_numbers.numbers();
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
        Err(e) => panic!("{:?}", e)
    }
    println!("{:?}", random_result)
}





//Is within Range

