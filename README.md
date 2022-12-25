# Advent of code 2022

This repo holds the solution to the [Advent of code
2022](https://adventofcode.com/2022) written in rust. In the following sections
a brief summary of each problem is presented.

## Day 1

The input is a list of numbers separated by empty lines.

- Part 1: Calculate the maximum of the sum of blocks of integers in the file.
- Part 2: Calculate the sum of the 3 largest sums of blocks of integers in the file.

## Day 2

Calculate the sum of the result of paper/scissor/rock games.

- Part 1: The input is the opponent's and my choice for each game.
- Part 2: The input is the opponent's choice and the result of each game.

## Day 3

The input is a list of strings.

- Part 1: Get a character that appears in both half of a string.
- Part 2: Find the character that appears in three consecutive lines.

## Day 4

The input are pairs of ranges (start-end)

- Part 1: Calculate how many ranges are fully contained in the associate range.
- Part 2: Calculate how many pairs of ranges have non empty overlaps. I do also
  implement the track of all the overlaps.

## Day 5

The input is a list of stacks of characters and a set of moves to apply to them.

- Part 1: The moves are applied with pop and push
- Part 2: The moves are applied to whole blocks (keeping the order)

## Day 6

The input is a long string. To obtain the result of part 1 set the variable
`LEN_BLOCK` to 4 and to 14 for the part 2.

- Part 1: Find the index for the firs block of 4 different characters.
- Part 2: Find the index for the firs block of 14 different characters.

## Day 7

The input is a file system structure. The composite pattern is used to solve
these problems (it can also be solved with stacks but I used this as an
opportunity to practise this kind of patterns in rust).

- Part 1: Find the sum of the sizes of directories with sizes less than 100000.
- Part 2: Find the shortest directory to free up the needed space.

## Day 8

The input is a matrix with trees' heights.

- Part 1: Find the number of visible trees.
- Part 2: Find the spot with the largest visibility.

## Day 9

The input is a set of moves to apply to a rope

- Part 1: Supposing a rope with 2 knots, find the positions that the tail have
  visited.
- Part 2: The same but with a rope with 10 knots.

## Day 10

The input is a series of commands to control a display

- Part 1: Add the product of a value and the cycle number every 40 cycles.
- Part 2: Read the commands to light up the pixels of the display if the cycle
  match with the CRT.

## Day 11

Monkey game

- Part 1: Items processed after 20 rounds.
- Part 2: Items processed after 10000 rounds.

## Day 12

Path finding allowing only maximum 1 height unit increase at each step.

- Part 1: Find shortest path from a start point.
- Part 2: Find the starting point with the shortest path. This could be
  implemented more efficiently maybe saving the path from visited nodes but 3
  seconds of runtime is OK.

## Day 13

Compare pairs of list of ints or lists. Again composite pattern.

- Part 1: Find how many pairs are in the correct order
- Part 2: Sort all of the inputs and find the indexes of two auxiliary flags.

## Day 14

Sand falling in the cave.

- Part 1: How many grains are stopped until one reaches the floor.
- Part 2: How many grains until the reach the source.

## Day 15

Sensors and closes beacon position.

- Part 1: Find the imposible region in a single line.
- Part 2: Find the only spot where a beacon can be.

## Day 16

Find optimal path to open valves.

- Part 1: 30 minutes to open valves one person.
- Part 2: 26 minutes to open valves one person and one elephant. This could be
  optimized.

## Day 17

Tetris.

- Part 1: Height after 2022 pieces.
- Part 2: Height after 1e12 pieces.

