# 🦀 Advent of Code 2022

In preparation of the AoC, I wrote a CLI tool to easily list and run my solutions as days go by.

It was the occasion to learn more about Rust's procedural macros !

### CLI Runner
Every day, I just need to add an annotated struct to `main.rs`, before the `declare_exercise_modules!();` 
line. This takes care of registering the exercise.

The CLI framework then expects a corresponding `aoc_<year>_<day>.rs` module next to `main.rs`, containing
the actual implementation of the solution for the designated challenge.

For instance, after the first day, this is my main.rs
```rust
#[advent_of_code(1, 2022)]
struct CalorieCounting;

declare_exercise_modules!();

fn main() -> Result<()> {
    // CLI code...
}
```

Then, in `aoc_2022_1.rs`:
```rust
pub fn run(input: &str) -> anyhow::Result<String> {
    // First part of the exercise...
}

pub fn run2(input: &str) -> anyhow::Result<String> {
    // Second part of the exercise...
}
```

When running the project with `./aoc-2022 run 2022 1 --id <aoc_session_id>`, it will automatically
download my account's problem input (thanks to the provided cookie session ID) if not already cached, then 
call the correct function and display its result.

That's it!

#### Cached input?
To not overload the AoC server while debugging, this utility will only download the input once, saving
it in the `downloaded_inputs/` directory next to the executable. Next time you run the same 
configuration, it will read from the cached version instead.

#### Why the session ID?
Each AoC account has its own input, so you need to be logged in to be able to retrieve it. The AoC 
website doesn't change the cookie session ID very often, so one can just get it from their browser
headers and use it with this utility. Hence the additional argument!

### Usage
Run the solution for a given day:
```shell
> ./aoc-2022 run 2022 1 --id somesessionid
Result: 720365
> ./aoc-2022 run 2022 24 --id somesessionid
Exercise of Dec 24, 2022 is not implemented. Exiting...
```

List available days that can be run:
```shell
> ./aoc-2022 run 2022 1 --id somesessionid
Dec 1, 2022 - CalorieCounting
Dec 2, 2022 - RockPaperScissors
```

Clear the cached input files:
```shell
> ./aoc-2022 clearcache
```

Help for the executable or any of the subcommands:
```shell
> ./aoc-2022 -h
Utility to run advent of code implementations

Usage: aoc-2022.exe [COMMAND]

Commands:
  clearcache  Clear the cache of downloaded inputs
  ls          Lists all days that can be run
  run         Runs the given exercise
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
  
> ./aoc-2022 run -h
Runs the given exercise

Usage: aoc-2022.exe run [OPTIONS] --id <ID> <YEAR> <DAY>

Arguments:
  <YEAR>  Year of the exercise to run
  <DAY>   Day of the exercise to run

Options:
      --id <ID>  The OAUTH session ID (cookie) for adventofcode.com
  -s, --second   Whether to execute the Second part of the exercise
  -h, --help     Print help information

```
