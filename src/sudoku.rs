use crate::types::{Message, GridApp};

pub fn clear(grid : &mut [[i32; 9]; 9]){
for r in 0..9{
    for c in 0..9{
        grid[r][c] = 0;
    }
}
            
}

fn check(grid : &[[i32; 9]; 9]) -> bool {
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

fn first_empty(grid : &[[i32; 9]; 9]) -> Option<(usize, usize)>{
    for r in 0..9{
        for c in 0..9{
            if grid[r][c] == 0{
                return Some((r, c));
            }
        }
    }

    None
}

fn helper(grid : &mut [[i32; 9]; 9], solutionFound : &mut bool){
    if !check(grid) || *solutionFound {return;}
    
    match first_empty(grid){
        Some((row, col)) =>{
            let mut guess = 1;
            let prev = grid[row][col];
            while guess <= 9{
                grid[row][col] = guess;
                helper(grid, solutionFound);
                if *solutionFound{return;}
                guess+=1;
            }
            grid[row][col] = prev;
        },
        None =>{
            *solutionFound = true;
            
        }
    }
   
}

pub fn solve(gridApp : &mut GridApp){
    let grid = &mut gridApp.grid;
    let mut solutionFound = false;
    //Find first empty cell
    helper(grid, &mut solutionFound);
  

}