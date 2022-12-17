pub fn is_in_row(board: &[[i8;9];9], row: &usize, number: &i8) -> bool{
    for col in 0..9 {
        if board[*row][col] == *number {
            return true;
        }
    }
    return false;
}

pub fn is_in_column(board: &[[i8;9];9], col: &usize, number: &i8) -> bool {
    for row in 0..9 {
        if board[row][*col] == *number {
            return true;
        }
    }
    return false;
}

pub fn is_in_box(board: &[[i8;9];9], row: &usize, col: &usize, number: &i8) -> bool {
    let l_col = *&col - (*&col % 3);
    let l_row = *&row - (*&row % 3);
    for row in l_row..l_row + 3 {
        for col in l_col..l_col + 3 {
            if board[row][col] == *number {return true;}
        }
    }
    return false;
}

pub fn is_valid_placement(board: &[[i8;9];9], row: &usize, col: &usize, number: &i8) -> bool {
    if !is_in_box(board, row, col, number) && !is_in_column(board, col, number) && !is_in_row(board, row, number) {return true;}
    else {return false;}
}

pub fn solve(board: &mut [[i8;9];9]) -> bool {
    let mut empties = 0;
    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] == 0 {
                for num in 1..=9 {
                    if is_valid_placement(&board, &i, &j, &num) {
                        board[*&i][*&j] = num;
                        if solve(&mut *board) == false {
                           board[*&i][*&j] = 0;
                           empties += 1;
                       }
                       else {return true;}
                    }
                }
                return false;
            }
        }
    }
    return empties == 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_board_test() {
        let mut board = [[7,5,2,0,5,0,6,0,0],
                        [0,0,0,0,0,3,0,0,0],
                        [1,0,0,0,0,9,5,0,0],
                        [8,0,0,0,0,0,0,9,0],
                        [0,4,3,0,0,0,7,5,0],
                        [0,9,0,0,0,0,0,0,8],
                        [0,0,9,7,0,0,0,0,5],
                        [0,0,0,2,0,0,0,0,0],
                        [0,0,7,0,4,0,2,0,3]];

        assert_eq!(solve(&mut board), false);
    }
}
