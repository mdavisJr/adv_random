# adv_random
Create random numbers, passwords, or strings based on rules.  You can choose your own random number generator to work with this framework.  By default this framework will use [random](https://crates.io/crates/random) if you do not provide a random number generator.

This framework comes with some standard rules that you can use.  If you don't see a rule that you need, you can create your own custom rules to use with this framework or send me a request to add in a new rule.

Please see examples of using rules below.  The framework can handle more combinations of rules than what is shown below.  If you would like to see an example that isn't shown, send me a request and I'll add that example.

# Examples
### Create 10 random numbers between 1 and 20 (min and max numbers are inclusive)
```
let random_result = random_numbers(&Settings::new(&[
    Box::new(NumberRange::all(1, 20))
], 10));
match random_result.numbers() {
    Ok(numbers) => {
        println!("{:?}", numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: [13, 10, 13, 7, 4, 3, 10, 8, 13, 18]
---
### Create 10 random numbers between 1 and 20 with no duplicates
```
let random_result = random_numbers(&Settings::new(&[
    Box::new(NoDuplicate{}),
    Box::new(NumberRange::all(1, 20))
], 10));
match random_result.numbers() {
    Ok(numbers) => {
        println!("{:?}", numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: [9, 8, 4, 13, 15, 17, 11, 3, 5, 14]
---
### Create 10 random numbers between 1 and 20 with duplicates numbers
```
let random_result = random_numbers(&Settings::with_exclude_rules(&[        
    Box::new(NumberRange::all(1, 20))
], 10, Some(vec![Box::new(NoDuplicate{})])));
match random_result.numbers() {
    Ok(numbers) => {
        println!("{:?}", numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: [9, 20, 19, 6, 19, 3, 7, 17, 5, 1]
---
### Create random phone number
```
let random_result = random_numbers(&Settings::new(&[
    Box::new(NumberRange::from_map(&[(&vec![0, 1], 100, 999), (&vec![2], 1000, 9999)]))
], 3));
match random_result.numbers() {
    Ok(numbers) => {
        println!("{:?}", numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: 752-804-6592
---
### Create 10 numbers between 1 and 100, excluding numbers 20 - 29 and 51 - 75, and with no duplicates
```
let random_result = random_numbers(&Settings::with_exclude_rules(&[
    Box::new(NoDuplicate{}),
    Box::new(NumberRange::all(1, 100))
], 10, Some(vec![Box::new(NumberRange::all(20, 29)), Box::new(NumberRange::all(51, 75))])));    
match random_result.numbers() {
    Ok(numbers) => {
        println!("{:?}", numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: [37, 81, 43, 10, 15, 4, 31, 12, 46, 93]
---
### Create 10 numbers between 1 and 100 with no duplicates that doesn't contain the number 23, contains the numbers 1 and 4, and at least 3 numbers from 17, 18, 19, 20, 21
```
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
        println!("{:?}", numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: [34, 4, 1, 19, 7, 69, 97, 21, 3, 20]
---
### Create 5 numbers between 1 and 10 with no duplicates that are Odd, Even, Odd, Even, and Odd
```
let random_result = random_numbers(&Settings::new(&[
    Box::new(NoDuplicate{}),
    Box::new(NumberRange::all(1, 10)),
    Box::new(OddEvenByIndex::new(&vec![0, 2, 4], &vec![1,3])),
], 5));
match random_result.numbers() {
    Ok(numbers) => {
        println!("{:?}", numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: [9, 6, 7, 2, 1]
---
### Create 5 numbers between 1 and 10 with no duplicates where first 3 numbers are Even and the last 2 numbers are Odd
```
let random_result = random_numbers(&Settings::new(&[
    Box::new(NoDuplicate{}),
    Box::new(NumberRange::all(1, 10)),
    Box::new(OddEvenByIndex::new(&vec![3,4], &vec![0, 1, 2]))
], 5));
match random_result.numbers() {
    Ok(numbers) => {
        println!("{:?}", numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: [6, 2, 8, 5, 9]
---
### Create 5 numbers between 1 and 10 with no duplicates and the 5th number is odd
```
let random_result = random_numbers(&Settings::new(&[
    Box::new(NoDuplicate{}),
    Box::new(NumberRange::all(1, 10)),
    Box::new(OddEvenByIndex::new(&vec![4], &vec![]))
], 5));
match random_result.numbers() {
    Ok(numbers) => {
        println!("{:?}", numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: [3, 2, 7, 5, 1]
---
### Create 10 numbers between 1 and 20 with no duplicates that has 5 odd and 5 even numbers
```
let random_result = random_numbers(&Settings::new(&[
    Box::new(NoDuplicate{}),
    Box::new(NumberRange::all(1, 20)),
    Box::new(OddEven::new(5, 5))
], 10));
match random_result.numbers() {
    Ok(numbers) => {
        println!("{:?}", numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: [7, 10, 1, 6, 2, 19, 14, 20, 11, 9]
---
### Create 10 numbers between 1 and 20 that has all even numbers
```
let random_result = random_numbers(&Settings::new(&[
    Box::new(NumberRange::all(1, 20)),
    Box::new(OddEven::new(0, 10))
], 10));
match random_result.numbers() {
    Ok(numbers) => {
        println!("{:?}", numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: [20, 18, 8, 8, 6, 18, 18, 4, 10, 2]
---
### Create 5 numbers between 1 and 10 with no duplicates that are all nonsequential
```
let random_result = random_numbers(&Settings::new(&[
    Box::new(NoDuplicate{}),
    Box::new(NumberRange::all(1, 10)),
    Box::new(Sequential::new(5, &[]))
], 5));
match random_result.numbers() {
    Ok(numbers) => {
        println!("{:?}", numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: [1, 8, 10, 6, 4]
---
### Create 5 numbers between 1 and 10 with no duplicates that has 3 nonsequential and has 1 sequential set of 2 numbers
```
let random_result = random_numbers(&Settings::new(&[
    Box::new(NoDuplicate{}),
    Box::new(NumberRange::all(1, 10)),
    Box::new(Sequential::new(3, &[2]))
], 5));
match random_result.numbers() {
    Ok(numbers) => {
        println!("{:?}", numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: [7, 8, 10, 4, 2]
---
### Create 5 numbers between 1 and 10 with no duplicates that has 1 nonsequential number and has 2 sequential sets of 2 numbers a piece
```
let random_result = random_numbers(&Settings::new(&[
    Box::new(NoDuplicate{}),
    Box::new(NumberRange::all(1, 10)),
    Box::new(Sequential::new(1, &[2, 2]))
], 5));
match random_result.numbers() {
    Ok(numbers) => {
        println!("{:?}", numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: [9, 10, 4, 5, 7]
---
### Create a random 20 character string with no special characters
```
let random_result = random_numbers(&Settings::new(&[
    Box::new(NumberPool::alphanumeric(20, false))
], 20));
match random_result.string(true) {
    Ok(numbers) => {
        println!("{:?}", numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: nAHeWQskMJ0gSafFw7dC
---
### Create a random 22 character string with special characters
```
let random_result = random_numbers(&Settings::new(&[
    Box::new(NumberPool::alphanumeric(20, true))
], 20));
match random_result.string(true) {
    Ok(numbers) => {
        println!("{:?}", numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: sEnTrfWXAqZhlB8+*YVd
---
### Create a random 20 character string with 10 letters, 10 numbers, no special characters, and no duplicates
```
let random_result = random_numbers(&Settings::new(&[
    Box::new(NoDuplicate{}),
    Box::new(NumberPool::alphanumeric_specs(10, 10, 0))
], 20));
match random_result.string(true) {
    Ok(numbers) => {
        println!("{:?}", numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: 61Wk34IMN52bRQ78r09Y
---
### Create a random 25 character string with 10 letters, 10 numbers, and 5 special characters and not duplicates
```
let random_result = random_numbers(&Settings::new(&[
    Box::new(NoDuplicate{}),
    Box::new(NumberPool::alphanumeric_specs(10, 10, 5))
], 25));
match random_result.string(true) {
    Ok(numbers) => {
        println!("{:?}", numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: 9JE6^*bG217$4AN83MF-0x#r5
---
### Create random 15 character string from upper case characters, from numbers 3 6 5, and from special characters # *
```
let random_result = random_numbers(&Settings::new(&[
    Box::new(NumberPool::new(&[
        NumberPoolItem::new("upper_case_alpha_set", &PoolType::new(&"ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect()), 11),
        NumberPoolItem::new("numeric_set", &PoolType::new(&"365".chars().collect()), 2),
        NumberPoolItem::new("special_char_set", &PoolType::new(&"#*".chars().collect()), 2),         
    ]))
], 15));
match random_result.string(true) {
    Ok(numbers) => {
        println!("{:?}", numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: *CBXSI#W6GONU5F
---
### Create random license number that starts with 2 apha-characters followed by 7 numeric characters. Also excludes "AB1234567" and "CB1234567" because they are already assigned to someone.
```
let uc_alpha_set: HashSet<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
let random_result = random_numbers(&Settings::new(&[
    Box::new(ExcludeNumberSets::new_string(&HashSet::from_iter(vec![String::from("AB1234567"), String::from("CB1234567")]))),
    Box::new(NumberPoolByIndex::new(vec![
        NumberPoolItemByIndex::new("upper_case_alpha_set", &PoolType::new(&uc_alpha_set.clone()), &HashSet::from_iter(vec![0, 1])),
        NumberPoolItemByIndex::new("numeric_set", &PoolType::new(&"1234567890".chars().collect()), &HashSet::from_iter(vec![2, 3, 4, 5, 6, 7, 8])),         
    ]))
], 9));
match random_result.string(false) {
    Ok(numbers) => {
        println!("{:?}", numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: ZW1184036
---
### Create 10 random numbers between 1 and 100 where the space between numbers is less than 3
```
let random_result = random_numbers(&Settings::new(&[
    Box::new(NumberRange::all(1, 100)),
    Box::new(NumberSpace::new(&vec![NumberSpaceItem::new(&NumberSpaceType::Lt(3), 9)]))
], 10));
match random_result.numbers() {
    Ok(numbers) => {
        let mut sorted_numbers: Vec<usize> = numbers.to_vec();
        sorted_numbers.sort_unstable();
        println!("{:?}", sorted_numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: [86, 87, 90, 93, 95, 96, 98, 99, 99, 100]
---
### Create 10 random numbers between 1 and 100 where the space between numbers is less than or equal to 3
```
let random_result = random_numbers(&Settings::new(&[
    Box::new(NumberRange::all(1, 100)),
    Box::new(NumberSpace::new(&vec![NumberSpaceItem::new(&NumberSpaceType::Lte(3), 9)]))
], 10));
match random_result.numbers() {
    Ok(numbers) => {
        let mut sorted_numbers: Vec<usize> = numbers.to_vec();
        sorted_numbers.sort_unstable();
        println!("{:?}", sorted_numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: [40, 42, 43, 46, 47, 49, 51, 53, 55, 58]
---
### Create 10 random numbers between 1 and 100 where the space between numbers is 3
```
let random_result = random_numbers(&Settings::new(&[
    Box::new(NumberRange::all(1, 100)),
    Box::new(NumberSpace::new(&vec![NumberSpaceItem::new(&NumberSpaceType::Eq(3), 9)]))
], 10));
match random_result.numbers() {
    Ok(numbers) => {
        let mut sorted_numbers: Vec<usize> = numbers.to_vec();
        sorted_numbers.sort_unstable();
        println!("{:?}", sorted_numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: [38, 41, 44, 47, 50, 53, 56, 59, 62, 65]
---
### Create 10 random numbers between 1 and 100 where the space between numbers is greater than or equal to 3
```
let random_result = random_numbers(&Settings::new(&[
    Box::new(NumberRange::all(1, 100)),
    Box::new(NumberSpace::new(&vec![NumberSpaceItem::new(&NumberSpaceType::Gte(3), 9)]))
], 10));
match random_result.numbers() {
    Ok(numbers) => {
        let mut sorted_numbers: Vec<usize> = numbers.to_vec();
        sorted_numbers.sort_unstable();
        println!("{:?}", sorted_numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: [9, 16, 28, 34, 47, 58, 76, 86, 92, 99]
---
### Create 10 random numbers between 1 and 100 where the space between numbers is greater than 3
```
let random_result = random_numbers(&Settings::new(&[
    Box::new(NumberRange::all(1, 100)),
    Box::new(NumberSpace::new(&vec![NumberSpaceItem::new(&NumberSpaceType::Gt(3), 9)]))
], 10));
match random_result.numbers() {
    Ok(numbers) => {
        let mut sorted_numbers: Vec<usize> = numbers.to_vec();
        sorted_numbers.sort_unstable();
        println!("{:?}", sorted_numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: [4, 14, 19, 29, 44, 50, 55, 69, 73, 99]
---
### Create 10 random numbers between 1 and 100 where the space between numbers is between 3 and 6
```
let random_result = random_numbers(&Settings::new(&[
    Box::new(NumberRange::all(1, 100)),
    Box::new(NumberSpace::new(&vec![NumberSpaceItem::new(&NumberSpaceType::Between(3, 6), 9)]))
], 10));
match random_result.numbers() {
    Ok(numbers) => {
        println!("{:?}", numbers);
    },
    _ => println!("{:?}", random_result.logs())
}
```
#### Output: [43, 49, 55, 61, 66, 71, 76, 82, 88, 91]
---
