# Sudoku solver

A fast sudoku solver using bitboards, using Rust's u128 type to fit 81 bits of state into one number.

## Usage

This solver takes sudoku puzzles in the form:

    800000000003600000070090200050007000000045700000100030001000068008500010090000400

which is the puzzle read from left to right and top to bottom, with empty spaces being zeros.

If given a file as the first argument, it solves and prints the solution for all puzzles within. Otherwise, it reads and solves puzzles from stdin.

## Internal representation

Each puzzle is represented with nine 128 bit bitboards, with each bitboard representing a number from 1-9, and each bit representing a square. A bit being on means that the square represented by that bit can be the number represented by which bitboard it is in. 

## Algorithm

This solver finds new solved squares by looking for squares which can only be one number, and squares which are the only ones in a row, column, or box which can be a certain number. When these techniques get stuck, it uses backtracking in conjunction with them, prioritizing guesses with the fewest possibilities.
