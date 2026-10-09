# adv_random
Create random numbers, passwords, or strings based on rules.  You can choose your own random number generator to work with this framework.  By default this framework will use [rand](https://crates.io/crates/rand) if you do not provide a random number generator.

This framework comes with some standard rules that you can use.  If you don't see a rule that you need, you can create your own custom rules to use with this framework or send me a request to add in a new rule.

Please see examples of using rules below.  The framework can handle more combinations of rules than what is shown below.  If you would like to see an example that isn't shown, send me a request and I'll add that example.

# Getting Started
Add adv_random to your `Cargo.toml`.
```toml
[dependencies]
adv_random = "3"
```
Say how many numbers you want, add the rules, and call `generate`.
```rust
use adv_random::random::RandomError;
use adv_random::rules::*;
use adv_random::settings::Settings;

fn main() -> Result<(), RandomError> {
    //10 random numbers between 1 and 20 with no duplicates
    let random_numbers = Settings::builder(10)
        .range(1, 20)
        .no_duplicates()
        .build()
        .generate()?;
    println!("{:?}", random_numbers.numbers());
    Ok(())
}
```
The examples below leave out the `use` lines and the function around them.  They use `?`, so they go in a function that returns `Result<(), RandomError>` like the one above.

# Settings
`Settings::builder(count)` starts the settings for `count` random numbers.

| Method | What it does |
|---|---|
| `.rule(rule)` | Adds a rule that the numbers have to match |
| `.exclude(rule)` | Adds a rule that the numbers can not match |
| `.range(min, max)` | Every number is from min to max.  The same as `.rule(NumberRange::all(min, max))` |
| `.no_duplicates()` | No number is there more than once.  The same as `.rule(NoDuplicate)` |
| `.odd_even(odd, even)` | This many odd numbers and this many even numbers.  The same as `.rule(OddEven::new().odd(odd).even(even))` |
| `.random(generator)` | Uses your random number generator |
| `.boxed_rules(rules)` and `.boxed_excludes(rules)` | Adds rules that are already in a `Vec<Box<dyn RuleTrait>>` or a `Vec<Box<dyn ExcludeRuleTrait>>` |
| `.build()` | Returns the `Settings` |

`Settings` can be used as many times as you want.  Every call to `generate` creates new random numbers.

# Results
`settings.generate()` returns `Result<RandomNumbers, RandomError>`.

`RandomNumbers` has the numbers.

| Method | What it returns |
|---|---|
| `numbers()` | The numbers, in the order they were created |
| `into_numbers()` | The numbers as a `Vec<usize>` that you own |
| `string()` | The numbers as characters, in the order they were created |
| `shuffled_string()` | The numbers as characters, in a random order |
| `attempts()` | How many attempts it took |
| `report()` | The numbers, the attempts and everything that happened in every attempt, for debugging |

`RandomError` says what went wrong.  Printing it with `{}` shows the short version.  `report()` returns the settings, the attempts and everything that happened in every attempt, for debugging.  Printing it with `{:?}` shows the report.

| `error_type()` | What it means |
|---|---|
| `RandomErrorType::BadRequest` | The settings can not work.  `messages()` says why |
| `RandomErrorType::Failed` | No numbers that match the rules were found in the attempts that are allowed |
| `RandomErrorType::InvalidCharacter` | A number is not a valid character, so the numbers can not be a string |

```rust
match Settings::builder(10).range(1, 20).odd_even(5, 5).build().generate() {
    Ok(random_numbers) => println!("{:?}", random_numbers.numbers()),
    Err(e) => println!("{}", e)
}
```

# Invalid Settings
Settings that can not work do not panic.  Every rule checks what it is given, and `build` checks the rules against the count and against each other.  Examples are odd and even counts that do not add up to the count, a range with a min greater than its max, an index that is past the last number, and a pool that is empty.  `generate` returns an error with the type `RandomErrorType::BadRequest`, and the same problems are in `settings.errors()` before you generate anything.
```rust
let settings = Settings::builder(5).range(20, 1).odd_even(1, 1).build();
for error in settings.errors() {
    println!("{}", error);
}
```
#### Output:
```
NumberRange min: 20 is greater than max: 1
Odd count: 1 and Even count: 1 must add up to count: 5
```

# Random Number Generator
Every `Settings` uses `DefaultRandom`, which uses [rand](https://crates.io/crates/rand).  Call `.random(...)` on the builder to use a different one.

`SeededRandom` gives the same numbers every time for the same seed.
```rust
use adv_random::seeded_random::SeededRandom;

let random_numbers = Settings::builder(10)
    .range(1, 20)
    .random(SeededRandom::new(42))
    .build()
    .generate()?;
```
Your own generator is anything that implements `RandomTrait`.
```rust
use adv_random::random_trait::RandomTrait;

#[derive(Debug)]
struct MyRandom {}

impl RandomTrait for MyRandom {
    //a number from min to max.  min and max are inclusive.
    fn get_number(&self, min: usize, max: usize) -> usize {
        return min + my_random_number() % (max - min + 1);
    }

    fn get_bool(&self) -> bool {
        return my_random_number() % 2 == 0;
    }
}
```
If you do not want rand, turn off the default features.  There is no default generator then, so every `Settings` needs `.random(...)`, and `generate` returns a `BadRequest` error without it.
```toml
[dependencies]
adv_random = { version = "3", default-features = false }
```

# Examples
### Create 10 random numbers between 1 and 20 (min and max numbers are inclusive)
```rust
let random_numbers = Settings::builder(10)
    .range(1, 20)
    .build()
    .generate()?;
println!("{:?}", random_numbers.numbers());
```
#### Output: [7, 2, 2, 12, 8, 1, 19, 14, 13, 17]
---
### Create 10 random numbers between 1 and 20 with no duplicates
```rust
let random_numbers = Settings::builder(10)
    .range(1, 20)
    .no_duplicates()
    .build()
    .generate()?;
println!("{:?}", random_numbers.numbers());
```
#### Output: [5, 1, 8, 15, 20, 16, 4, 11, 9, 18]
---
### Create 10 random numbers between 1 and 20 with duplicates numbers
```rust
let random_numbers = Settings::builder(10)
    .range(1, 20)
    .exclude(NoDuplicate)
    .build()
    .generate()?;
println!("{:?}", random_numbers.numbers());
```
#### Output: [12, 13, 7, 1, 9, 20, 19, 8, 14, 13]
---
### Create random phone number
```rust
let random_numbers = Settings::builder(3)
    .rule(NumberRange::new().at([0, 1], 100, 999).at([2], 1000, 9999))
    .build()
    .generate()?;
let numbers = random_numbers.numbers();
println!("{}-{}-{}", numbers[0], numbers[1], numbers[2]);
```
#### Output: 715-889-7948
---
### Create 10 numbers between 1 and 100, excluding numbers 20 - 29 and 51 - 75, and with no duplicates
```rust
let random_numbers = Settings::builder(10)
    .range(1, 100)
    .no_duplicates()
    .exclude(NumberRange::all(20, 29))
    .exclude(NumberRange::all(51, 75))
    .build()
    .generate()?;
println!("{:?}", random_numbers.numbers());
```
#### Output: [94, 96, 90, 11, 12, 79, 34, 98, 43, 93]
---
### Create 10 numbers between 1 and 100 with no duplicates that doesn't contain the number 23, contains the numbers 1 and 4, and at least 3 numbers from 17, 18, 19, 20, 21
```rust
let random_numbers = Settings::builder(10)
    .range(1, 100)
    .no_duplicates()
    .rule(NumberPool::new()
        .none_of([23])
        .exactly(2, [1, 4])
        .at_least(3, [17, 18, 19, 20, 21]))
    .build()
    .generate()?;
println!("{:?}", random_numbers.numbers());
```
#### Output: [48, 17, 4, 1, 20, 35, 76, 95, 19, 88]
A pool is a list of numbers like `[1, 4]`, a range like `1..=10`, or the characters of a string like `"ABC"`.  A number that is picked twice counts twice, so `exactly(2, [1, 4])` means both 1 and 4 because there are no duplicates.  `NumberPool` also has `at_most` and `between`.

---
### Create 5 numbers between 1 and 10 with no duplicates that are Odd, Even, Odd, Even, and Odd
```rust
let random_numbers = Settings::builder(5)
    .range(1, 10)
    .no_duplicates()
    .rule(OddEvenByIndex::new().odd([0, 2, 4]).even([1, 3]))
    .build()
    .generate()?;
println!("{:?}", random_numbers.numbers());
```
#### Output: [5, 4, 9, 10, 1]
---
### Create 5 numbers between 1 and 10 with no duplicates where first 3 numbers are Even and the last 2 numbers are Odd
```rust
let random_numbers = Settings::builder(5)
    .range(1, 10)
    .no_duplicates()
    .rule(OddEvenByIndex::new().odd([3, 4]).even([0, 1, 2]))
    .build()
    .generate()?;
println!("{:?}", random_numbers.numbers());
```
#### Output: [4, 10, 6, 9, 7]
---
### Create 5 numbers between 1 and 10 with no duplicates and the 5th number is odd
```rust
let random_numbers = Settings::builder(5)
    .range(1, 10)
    .no_duplicates()
    .rule(OddEvenByIndex::new().odd([4]))
    .build()
    .generate()?;
println!("{:?}", random_numbers.numbers());
```
#### Output: [5, 4, 2, 9, 7]
---
### Create 10 numbers between 1 and 20 with no duplicates that has 5 odd and 5 even numbers
```rust
let random_numbers = Settings::builder(10)
    .range(1, 20)
    .no_duplicates()
    .odd_even(5, 5)
    .build()
    .generate()?;
println!("{:?}", random_numbers.numbers());
```
#### Output: [7, 17, 4, 14, 9, 1, 13, 2, 6, 18]
---
### Create 10 numbers between 1 and 20 that has all even numbers
```rust
let random_numbers = Settings::builder(10)
    .range(1, 20)
    .odd_even(0, 10)
    .build()
    .generate()?;
println!("{:?}", random_numbers.numbers());
```
#### Output: [20, 20, 8, 2, 4, 14, 2, 2, 14, 8]
---
### Create 5 numbers between 1 and 10 with no duplicates that are all nonsequential
`singles` is how many numbers are not next to another number.  `in_a_row` adds one sequence, so `in_a_row(3)` is 3 numbers in a row like 4, 5, 6.
```rust
let random_numbers = Settings::builder(5)
    .range(1, 10)
    .no_duplicates()
    .rule(Sequential::new().singles(5))
    .build()
    .generate()?;
println!("{:?}", random_numbers.numbers());
```
#### Output: [7, 5, 1, 3, 10]
---
### Create 5 numbers between 1 and 10 with no duplicates that has 3 nonsequential and has 1 sequential set of 2 numbers
```rust
let random_numbers = Settings::builder(5)
    .range(1, 10)
    .no_duplicates()
    .rule(Sequential::new().singles(3).in_a_row(2))
    .build()
    .generate()?;
println!("{:?}", random_numbers.numbers());
```
#### Output: [2, 3, 7, 5, 9]
---
### Create 5 numbers between 1 and 10 with no duplicates that has 1 nonsequential number and has 2 sequential sets of 2 numbers a piece
```rust
let random_numbers = Settings::builder(5)
    .range(1, 10)
    .no_duplicates()
    .rule(Sequential::new().singles(1).in_a_row(2).in_a_row(2))
    .build()
    .generate()?;
println!("{:?}", random_numbers.numbers());
```
#### Output: [6, 7, 1, 2, 10]
---
### Create a random 20 character string with no special characters
`NumberPool::alphanumeric(20)` is letters and 1 to 3 digits.  `with_special_characters` adds 1 or 2 special characters.  `digits` and `special_characters` say exactly how many there are, and the rest are letters.
```rust
let random_numbers = Settings::builder(20)
    .rule(NumberPool::alphanumeric(20))
    .build()
    .generate()?;
println!("{}", random_numbers.shuffled_string()?);
```
#### Output: kAKjsQ8iO5HLewV6frUt
---
### Create a random 20 character string with special characters
```rust
let random_numbers = Settings::builder(20)
    .rule(NumberPool::alphanumeric(20).with_special_characters())
    .build()
    .generate()?;
println!("{}", random_numbers.shuffled_string()?);
```
#### Output: 0scj@HK+ZW4d2bDivAqY
---
### Create a random 20 character string with 10 letters, 10 numbers, no special characters, and no duplicates
```rust
let random_numbers = Settings::builder(20)
    .no_duplicates()
    .rule(NumberPool::alphanumeric(20).digits(10))
    .build()
    .generate()?;
println!("{}", random_numbers.shuffled_string()?);
```
#### Output: 64VEU1C2brQu837y0J95
---
### Create a random 25 character string with 10 letters, 10 numbers, and 5 special characters and no duplicates
```rust
let random_numbers = Settings::builder(25)
    .no_duplicates()
    .rule(NumberPool::alphanumeric(25).digits(10).special_characters(5))
    .build()
    .generate()?;
println!("{}", random_numbers.shuffled_string()?);
```
#### Output: uV605wKk*-)I2+3Hr4v987^1x
---
### Create random 15 character string from upper case characters, from numbers 3 6 5, and from special characters # *
```rust
let random_numbers = Settings::builder(15)
    .rule(NumberPool::new()
        .exactly(11, "ABCDEFGHIJKLMNOPQRSTUVWXYZ")
        .exactly(2, "365")
        .exactly(2, "#*"))
    .build()
    .generate()?;
println!("{}", random_numbers.shuffled_string()?);
```
#### Output: IN5#YTPUSM3ZV*E
---
### Create random license number that starts with 2 apha-characters followed by 7 numeric characters. Also excludes "AB1234567" and "CB1234567" because they are already assigned to someone.
```rust
let random_numbers = Settings::builder(9)
    .rule(ExcludeNumberSets::new().string("AB1234567").string("CB1234567"))
    .rule(NumberPoolByIndex::new()
        .at([0, 1], "ABCDEFGHIJKLMNOPQRSTUVWXYZ")
        .at(2..=8, "1234567890"))
    .build()
    .generate()?;
println!("{}", random_numbers.string()?);
```
#### Output: MW6406422
---
### Create 10 random numbers between 1 and 100 where the space between numbers is less than 3
`run(OperatorType::Lt(3), 9)` is 9 spaces in a row that are less than 3.  10 numbers have 9 spaces between them.
```rust
let random_numbers = Settings::builder(10)
    .range(1, 100)
    .rule(NumberSpace::new().run(OperatorType::Lt(3), 9))
    .build()
    .generate()?;
let mut sorted_numbers = random_numbers.into_numbers();
sorted_numbers.sort_unstable();
println!("{:?}", sorted_numbers);
```
#### Output: [30, 31, 32, 33, 34, 35, 37, 38, 40, 41]
---
### Create 10 random numbers between 1 and 100 where the space between numbers is less than or equal to 3
```rust
let random_numbers = Settings::builder(10)
    .range(1, 100)
    .rule(NumberSpace::new().run(OperatorType::Lte(3), 9))
    .build()
    .generate()?;
let mut sorted_numbers = random_numbers.into_numbers();
sorted_numbers.sort_unstable();
println!("{:?}", sorted_numbers);
```
#### Output: [81, 82, 84, 85, 87, 90, 93, 95, 98, 100]
---
### Create 10 random numbers between 1 and 100 where the space between numbers is 3
```rust
let random_numbers = Settings::builder(10)
    .range(1, 100)
    .rule(NumberSpace::new().run(OperatorType::Eq(3), 9))
    .build()
    .generate()?;
let mut sorted_numbers = random_numbers.into_numbers();
sorted_numbers.sort_unstable();
println!("{:?}", sorted_numbers);
```
#### Output: [37, 40, 43, 46, 49, 52, 55, 58, 61, 64]
---
### Create 10 random numbers between 1 and 100 where the space between numbers is greater than or equal to 3
```rust
let random_numbers = Settings::builder(10)
    .range(1, 100)
    .rule(NumberSpace::new().run(OperatorType::Gte(3), 9))
    .build()
    .generate()?;
let mut sorted_numbers = random_numbers.into_numbers();
sorted_numbers.sort_unstable();
println!("{:?}", sorted_numbers);
```
#### Output: [10, 14, 29, 53, 58, 62, 69, 73, 86, 100]
---
### Create 10 random numbers between 1 and 100 where the space between numbers is greater than 3
```rust
let random_numbers = Settings::builder(10)
    .range(1, 100)
    .rule(NumberSpace::new().run(OperatorType::Gt(3), 9))
    .build()
    .generate()?;
let mut sorted_numbers = random_numbers.into_numbers();
sorted_numbers.sort_unstable();
println!("{:?}", sorted_numbers);
```
#### Output: [5, 29, 33, 52, 61, 66, 72, 88, 92, 97]
---
### Create 10 random numbers between 1 and 100 where the space between numbers is between 3 and 6
```rust
let random_numbers = Settings::builder(10)
    .range(1, 100)
    .rule(NumberSpace::new().run(OperatorType::Between(3, 6), 9))
    .build()
    .generate()?;
let mut sorted_numbers = random_numbers.into_numbers();
sorted_numbers.sort_unstable();
println!("{:?}", sorted_numbers);
```
#### Output: [30, 34, 39, 45, 49, 53, 59, 62, 67, 72]
---

# Custom Rules
A rule is anything that implements `RuleTrait`.  It also has to implement `Clone`, `Debug` and `Display`.  This rule only allows numbers that are a multiple of a number.
```rust
use adv_random::random::CurrentData;
use adv_random::rules::{IsWithinErrorType, RuleTrait, SharedData};
use std::any::Any;
use std::fmt;

#[derive(Clone, Debug)]
struct MultipleOf {
    multiple: usize,
}

impl fmt::Display for MultipleOf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MultipleOf:{}", self.multiple)
    }
}

impl RuleTrait for MultipleOf {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn name(&self) -> String {
        return String::from("MultipleOf");
    }

    //Return an error when the count can not work with this rule
    fn check_count(&self, _count: usize) -> Result<bool, String> {
        return Ok(true);
    }

    //Tell the other rules about the next number.  This rule has nothing to tell.
    fn share_data(&self, _current_data: &CurrentData) -> Option<SharedData> {
        return None;
    }

    //Return the next numbers.  Ok(None) lets another rule get them.
    fn get_numbers(&self, _current_data: &CurrentData) -> Result<Option<Vec<usize>>, String> {
        return Ok(None);
    }

    //Called every time numbers are added.  Return an error when the numbers selected so far can not work.
    fn is_within_range(&self, current_data: &CurrentData) -> Result<(), (IsWithinErrorType, String)> {
        if current_data.selected_numbers().iter().any(|x| x % self.multiple != 0) {
            return Err((IsWithinErrorType::Regular, format!("Not every number is a multiple of {}", self.multiple)));
        }
        return Ok(());
    }

    //Called when all of the numbers are selected
    fn is_match(&self, current_data: &CurrentData) -> Result<(), String> {
        return self.is_within_range(current_data).map_err(|x| x.1);
    }
}
```
It is used like any other rule.
```rust
let random_numbers = Settings::builder(5)
    .range(1, 100)
    .rule(MultipleOf { multiple: 5 })
    .build()
    .generate()?;
println!("{:?}", random_numbers.numbers());
```
#### Output: [100, 40, 65, 50, 55]

To use a rule with `.exclude(...)` it also has to implement `ExcludeRuleTrait`.

A rule that can have more than one thing wrong with it can also implement `validate`, which returns every problem.  Without it the error from `check_count` is used.

Custom rules can share their own values with each other.  One rule returns `Some(SharedData::new().with_custom("my_key", CustomValue::Usize(5)))` from `share_data`, and another rule reads it with `current_data.shared_data().custom("my_key")`.  Only one rule can set a key.

# Upgrading From Version 2
`Settings::new`, `Settings::with_exclude_rules`, `random_numbers`, `NumberRange::from_map`, `NumberPool::alphanumeric_specs`, `NumberPool::alphanumeric_with_random` and `ExcludeNumberSets::new_string` still work.  They are deprecated and the compiler shows what to use instead.  The other lines in this table do not compile in version 3 and have to be changed.

| Version 2 | Version 3 |
|---|---|
| `Settings::new(&[Box::new(rule)], 10)` | `Settings::builder(10).rule(rule).build()` |
| `Settings::with_exclude_rules(&[...], 10, Some(vec![Box::new(rule)]))` | `Settings::builder(10).exclude(rule).build()` |
| `random_numbers(&settings)` | `settings.generate()` |
| `random_result.string(true)` | `random_result.shuffled_string()` |
| `random_result.string(false)` | `random_result.string()` |
| `NumberPool::new(&[NumberPoolItem::new("key", &pool, OperatorType::Eq(2))])` | `NumberPool::new().exactly(2, pool)` or `.item("key", pool, OperatorType::Eq(2))` |
| `NumberSpace::new(&[NumberSpaceItem::new(&OperatorType::Lt(3), 9)])` | `NumberSpace::new().run(OperatorType::Lt(3), 9)` |
| `Sequential::new(1, &[2, 2])` | `Sequential::new().singles(1).in_a_row(2).in_a_row(2)` |
| `NumberPool::alphanumeric(20, true)` | `NumberPool::alphanumeric(20).with_special_characters()` |
| `NumberPool::alphanumeric_specs(10, 10, 5)` | `NumberPool::alphanumeric(25).digits(10).special_characters(5)` |
| `NumberPool::alphanumeric_with_random(&random, 20, false)` | `NumberPool::alphanumeric(20)`, with `.random(random)` on the settings builder |
| `NumberSpaceItem::new_full(&operator, &[2], &[2])` | `NumberSpaceItem::new(operator).needs(&[2]).has(&[2])` |
| `number_space_item.needs()` and `number_space_item.has()` | `number_space_item.needed()` and `number_space_item.found()` |
| `ExcludeNumberSets::new(&sets)` | `ExcludeNumberSets::new().numbers_each(sets)` |
| `ExcludeNumberSets::new_string(&strings)` | `ExcludeNumberSets::new().string("AB1234567")` or `.string_each(strings)` |
| `OddEven::new(5, 5)` | `OddEven::new().odd(5).even(5)` |
| `odd_even.odd()` and `odd_even.even()` | `odd_even.odd_count()` and `odd_even.even_count()` |
| `OddEvenByIndex::new(&[0, 2], &[1])` | `OddEvenByIndex::new().odd([0, 2]).even([1])` |
| `NumberPoolByIndex::new(vec![NumberPoolItemByIndex::new(...)])` | `NumberPoolByIndex::new().at([0, 1], pool)` |
| `NumberRange::from_map(&[(&[0, 1], 100, 999)])` | `NumberRange::new().at([0, 1], 100, 999)` |
| `PoolType::Set(hash_set)` | `PoolType::set([1, 4])` |
| `set_random_trait(generator)` | `.random(generator)` on the builder |

`random_numbers(&settings)` returns a `RandomResult` with `status()`, `numbers()` and `logs()`, the same as in version 2.  Call `into_result()` on it to get the `Result` that `generate` returns.

Custom rules have to change too.  `get_numbers` returns `Ok(None)` instead of `Err("Skip")`, and `share_data` returns `Option<SharedData>`.

A `match` on `OperatorType` or `RandomErrorType` needs a `_` arm, because more types can be added to them.
