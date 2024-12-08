# Advent of Code 2024

My solutions for [Advent of Code](https://adventofcode.com) 2024. Merry Christmas!

## Usage

Inputs are expected to be in a directory named `inputs`, with file names
corresponding to the module for that day:

```
inputs/
    d01.txt
    d02.txt
    ...
```

Each module is expected to have 3 public functions:

- `part_1`
- `part_2`
- `parse_input`

`parse_input` parses the puzzle input into some useful type. `part_1` and
`part_2` take a _reference_ to that type and return answers to the puzzle.

The main binary executes solutions using these and times each step.

To run all days:

```
cargo run
```

To run a specific day (day 2 in this case):

```
cargo run 02
```

## TODO

- d02 (efficiency)
