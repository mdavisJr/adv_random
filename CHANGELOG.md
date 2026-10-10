# Changelog

## 3.0.0

Version 3 changes how settings and rules are created, checks them before any numbers are generated, and fixes the rules that gave wrong or unreliable results.  Most version 2 code has to change.  The [README](README.md#upgrading-from-version-2) has a table that shows the version 2 code next to the version 3 code.

The minimum Rust version is 1.87.

### Added
- **Settings builder.**  `Settings::builder(count)` with `rule`, `exclude`, `random`, `boxed_rules`, `boxed_excludes` and `build`.  Rules are passed as they are, without `Box::new` or a slice.  `range`, `no_duplicates` and `odd_even` are shorthands for the most common rules.
- **A `Result` from generating.**  `settings.generate()` returns `Result<RandomNumbers, RandomError>`, so `?` works.
  - `RandomNumbers` has `numbers`, `into_numbers`, `string`, `shuffled_string`, `attempts`, `logs`, `clear_err_tracker` and `report`.
  - `RandomError` has `error_type`, `messages`, `attempts`, `logs`, `clear_err_tracker` and `report`.  It implements `std::error::Error`.  `{}` prints one line and `{:?}` prints the report.
  - `RandomErrorType` is `BadRequest`, `Failed` or `InvalidCharacter`.
- **Rules are created with chained methods.**
  - `NumberPool::new()` with `exactly`, `at_least`, `at_most`, `between`, `none_of` and `item`.  A pool can be a list or set of numbers, a range like `1..=10`, or the characters of a string like `"ABC"`.
  - `NumberPool::alphanumeric(count)` with `digits`, `with_special_characters` and `special_characters`.
  - `NumberSpace::new()` with `run`, `runs`, `item`, `items` and `process`.
  - `NumberSpaceItem::new(type)` with `needs` and `has`.
  - `Sequential::new()` with `singles`, `in_a_row` and `in_a_row_each`.
  - `OddEven::new()` with `odd` and `even`.
  - `OddEvenByIndex::new()` with `odd` and `even`.
  - `NumberPoolByIndex::new()` with `at`.
  - `NumberRange::new()` with `at`.
  - `ExcludeNumberSets::new()` with `numbers`, `numbers_each`, `string` and `string_each`.
- **Settings are checked before any numbers are generated.**  Every problem is in `settings.errors()`, and `generate` returns a `BadRequest` error with the same messages and no attempts.  What is checked:
  - Counts that do not add up to the count, for `OddEven` and `Sequential`.
  - A range with a min greater than its max.
  - An index that is past the last number.
  - The same index with two ranges, two pools, or both odd and even.
  - A sequence of fewer than 2 numbers, a run of 0 spaces, and an `OperatorType` that can never match.
  - A pool that is empty when numbers have to come from it, and a pool key that is used twice.
  - Runs or pools that need more numbers than the count allows.
  - Runs or pools that are not possible with `NoDuplicate`.
  - A rule that nothing was added to.
  - A value that can only be set once and was set twice.
  - Two custom rules that share the same custom key.
- **A random number generator for each `Settings`.**  `.random(generator)` on the builder, or `settings.with_random(generator)`.  This works with and without the `rand` feature.
- **`SeededRandom`**, which gives the same numbers every time for the same seed.
- **`adv_random::prelude`**, which brings in everything most programs need.
- **`SharedData`** for what rules tell each other, with `with_custom` and `custom` for values that custom rules share with each other.
- **`RuleTrait::validate` and `ExcludeRuleTrait::validate_exclude`.**  A custom rule can use them to report more than one problem.  They are optional.
- `PoolType::set`, `PoolType::from_chars` and `PoolSet`.
- `Settings` and the rules can be sent to and shared between threads.
- The examples in the README are compiled and run by `cargo test`.

### Changed
These need changes to version 2 code.

- **The old `new(...)` with arguments is gone** from `NumberPool`, `NumberSpace`, `NumberSpaceItem`, `Sequential`, `OddEven`, `OddEvenByIndex`, `NumberPoolByIndex` and `ExcludeNumberSets`.  `new()` takes no arguments and the values are given with the chained methods above.  `NumberPool::alphanumeric` takes only the count.
- **A number that is picked twice counts twice in a `NumberPool`.**  Version 2 counted it once.  `exactly(2, [1, 4])` means both 1 and 4 only when `NoDuplicate` is also used.
- **The counts of `OddEven` and of `Sequential` have to add up to the count.**  Version 2 accepted smaller counts and then never found any numbers.
- **Settings that can not work return a `BadRequest` error.**  Version 2 panicked for some and tried 500 times for others.
- **Custom rules:**
  - `get_numbers` returns `Result<Option<Vec<usize>>, String>`.  `Ok(None)` replaces `Err("Skip")`.
  - `share_data` returns `Option<SharedData>`.
  - `RuleTrait` and `ExcludeRuleTrait` require `Send + Sync`.
  - `CurrentData::new` takes `&SharedData`.
  - `CurrentData::get_rand` and `PoolType::random_number` return `Option<usize>`.  `None` means no number that fits was found.
  - `CurrentData::random` returns the random number generator of the settings.
- **Custom random number generators:**
  - `RandomTrait` no longer requires `Clone`.
  - `get_random_vec_item` and `shuffle_vec` take the generator as their first argument.  `get_random_vec_item` returns `Option<&T>`.
  - `DefaultRandom::new` returns a `DefaultRandom`.
- **Readers that were renamed** because their names now set the value:
  - `OddEven`: `odd()` and `even()` are `odd_count()` and `even_count()`.
  - `NumberSpaceItem`: `needs()` and `has()` are `needed()` and `found()`.
- `RandomResult::string(bool)` is `string()` and `shuffled_string()`.
- `PoolType::Set` holds a `PoolSet`.  Create one with `PoolType::set`.
- `PoolType::has` and `NumberPool::from_numbers` take `&[usize]`.
- `NumberPool::number_pool_items` returns a `BTreeMap`.
- `NP_ALPHABET_SET`, `NP_NUMERIC_SET` and `NP_SPECIAL_CHAR_SET` are arrays of characters.
- `Sequential::needs_seq` takes `&CurrentData`.
- A `match` on `OperatorType` or `RandomErrorType` needs a `_` arm, because more types can be added to them.
- The `once_cell` dependency is removed, the `cdylib` crate type is removed, and the optional `rand` dependency is version 0.10.

### Deprecated
These still work.  The compiler shows what to use instead.

- `Settings::new` and `Settings::with_exclude_rules`.  Use `Settings::builder`.
- `random_numbers`.  Use `settings.generate()`.
- `NumberRange::from_map`.  Use `NumberRange::new().at(...)`.
- `NumberSpace::new_with_process_number_space_items`.  Use `NumberSpace::new().item(...).process(...)`.
- `NumberPool::alphanumeric_with_random` and `NumberPool::alphanumeric_specs`.  Use `NumberPool::alphanumeric(count)`.
- `ExcludeNumberSets::new_string`.  Use `ExcludeNumberSets::new().string(...)`.

### Removed
- `get_random_trait` and `set_random_trait`.  Use `.random(generator)` on the settings builder.
- `RandomTraitClone`.
- `MapAnyValue` and the `SHARED_DATA_*` constants.  Use `SharedData`.
- `Settings::get_min_max` and `Settings::get_number_within_number_range`.  Use `current_data.shared_data().min_max()` and `current_data.get_rand(...)`.
- `NumberPoolItemByIndex`.  Use `NumberPoolByIndex::new().at(...)`.
- `NumberSpaceItem::new_full`.  Use `NumberSpaceItem::new(type).needs(...).has(...)`.
- `halt_from_error` is no longer public.

### Fixed
- `NumberPool` could return more numbers from a pool than it was asked for when a number was picked twice.
- `Sequential` never found numbers when its sequence counts were not given from smallest to largest.  The order no longer matters.
- `Sequential` could not create more than one sequence unless there were also numbers that are not in a sequence.
- `NumberSpace` with `Eq`, with `Lte(1)`, and with a `Between` that has the same lower and upper bound created no numbers or found them only some of the time.
- `NumberSpace` with more than one run, and with no spare numbers, found numbers only some of the time.
- Creating a string panicked when a number was not a valid character.  It returns an `InvalidCharacter` error.
- A range with a min greater than its max panicked.
- A count of 0 tried 500 times and failed.  It returns no numbers.
- Numbers near the largest `usize` could overflow.
- An exclude rule that asked to be made the priority panicked.
- A rule could get 0 as its random number when no number that fits was found, even when 0 was not in the range.
- `NoDuplicate` did not stop a number that was already picked from being picked again, so settings that need most of the numbers in a range took many more attempts or failed.

### Performance
- Creating numbers from a `NumberPool`, such as a password, is about twice as fast.
- `NumberSpace` and `Sequential` take far fewer attempts.

### Known Limitations
- `NumberSpace` with `Gte(1)` or `Gt(0)`, more than one run, duplicates allowed and spare numbers finds numbers a little more than half of the time.  Every space between two different numbers matches those types, so two runs can only be separated by a number that is there twice.
