//problem 36 -> is valid sudoku
//this problem is about validating a sudoku, we are given a 2 dimentional vector of chars, we have
//to validate these 3 conditions, to solve this we need to
//1. check 1-9 is present only once in each of 9 rows, 9 columns and 9 - 3x3 boxes
//2. we will use 9 hashsets for 9 rows, 9 hashsets for all 9 columns and 9 hashsets for 9 boxes
//3. each box will be determined by a box index which requires a simple formula -> (row/3) * 3 +
//   (column/3)

use std::collections::HashSet;
pub struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_set: Vec<HashSet<char>> = (0..9).map(|_| HashSet::new()).collect();
        let mut col_set: Vec<HashSet<char>> = (0..9).map(|_| HashSet::new()).collect();
        let mut box_set: Vec<HashSet<char>> = (0..9).map(|_| HashSet::new()).collect();

        for i in 0..9 {
            //because board is 9x9
            for j in 0..9 {
                let val = board[i][j];
                if val == '.' {
                    continue;
                }
                let box_index = (i / 3) * 3 + (j / 3);
                if row_set[i].contains(&val)
                    || col_set[j].contains(&val)
                    || box_set[box_index].contains(&val)
                {
                    return false;
                }
                row_set[i].insert(val);
                col_set[j].insert(val);
                box_set[box_index].insert(val);
            }
        }
        true
    }
}
