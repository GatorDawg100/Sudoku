use crate::types::{Message, GridApp};

pub fn clear(grid : &mut [[i32; 9]; 9]){
    for r in 0..9{
        for c in 0..9{
            grid[r][c] = 0;
        }
    }
}

fn check(grid : &[[i32; 9]; 9]) -> bool {//Checks if whole grid is valid
    let mut rows : [i32; 9] = [0; 9];
    let mut cols : [i32; 9] = [0; 9];
    let mut boxs : [i32; 9] = [0; 9];
    
    for r in 0..9{
        for c in 0..9{
            if grid[r][c] == 0{
                continue;
            }
            let bit : i32 = 1 << (grid[r][c]-1);
            let box_num : usize = ((r / 3) * 3) + (c / 3);
            
            if (rows[r] & bit != 0) || (cols[c] & bit != 0) || (boxs[box_num] & bit != 0){
                return false;
            }

            rows[r] |= bit;
            cols[c] |= bit;
            boxs[box_num] |= bit;
        } 
    }
  true  
}

fn is_valid(grid: &[[i32; 9]; 9], row: usize, col: usize, num: i32) -> bool {//Checks if single number addition is still valid
        // Check row and column
		for i in 0..9 {
	    if grid[row][i] == num || grid[i][col] == num {
	        return false;
            }
	}
        // Check 3x3 subgrid
        let start_row = (row / 3) * 3;
        let start_col = (col / 3) * 3;
        for r in 0..3 {
            for c in 0..3 {
                if grid[start_row + r][start_col + c] == num {
                    return false;
                }
            }
        }
        true
    }
    
    fn find_best_cell(grid: &[[i32; 9]; 9]) -> Option<(usize, usize, Vec<i32>)> {
            let mut best_cell = None;
            let mut min_candidates = 10;
    
            for r in 0..9 {
                for c in 0..9 {
                    if grid[r][c] == 0 {
                        let mut candidates = Vec::new();
                        for num in 1..=9 {
                            if is_valid(grid, r, c, num) {
                                candidates.push(num);
                            }
                        }
    
                        // DEAD END DETECTED: This cell has 0 possible moves.
                        // Fail immediately without checking any other cells!
                        if candidates.is_empty() {
                            return Some((r, c, candidates));
                        }
    
                        if candidates.len() < min_candidates {
                            min_candidates = candidates.len();
                            best_cell = Some((r, c, candidates));
                        }
                    }
                }
            }
    
            best_cell
        }

fn helper(grid: &mut [[i32; 9]; 9], solution_found: &mut bool) {
        if *solution_found { return; }

        match find_best_cell(grid) {
            None => {
                // No empty cells left, puzzle is solved!
      		*solution_found = true;
            }
            Some((row, col, candidates)) => {
                // If candidates is empty (0 possibilities), this loop won't even execute
                // and the function will immediately backtrack!
                for num in candidates {
                    grid[row][col] = num;
                    helper(grid, solution_found);
                    if *solution_found { return; }
                    grid[row][col] = 0;
                }
            }
        }
    }

pub fn solve(gridApp : &mut GridApp){
    let grid = &mut gridApp.grid;
    // Check once upfront: is the user's input legally valid?
    if !check(grid) {
        return; // Board has starting duplicates, cannot be solved
    }
    
    let mut solutionFound = false;
    //Find first empty cell
    helper(grid, &mut solutionFound);
}