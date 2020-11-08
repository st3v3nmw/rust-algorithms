def can_be_placed(board, i, j, k):
    """ Returns True if k can be placed at position (i, j) in the sudoku board """
    # checks if k is in row i
    if k in board[i]:
        return False
    # checks if k is in column j
    if k in [board[x][j] for x in range(9)]:
       return False
    # get the indexes of the top-leftmost box if the 3 * 3 box
    box_i = i // 3 * 3
    box_j = j // 3 * 3
    # checks if k is in the 3x3 box that it belongs to
    box = []
    for row in board[box_i:box_i+3]:
        box.extend(row[box_j:box_j+3])
    if k in box: return False
    return True

def is_solved(board):
    """ Returns True if the Sudoku has been solved completely, else False """
    zero_count = 0
    for row in board:
        zero_count += row.count(0)
    return zero_count == 0

def solve_sudoku(board):
    """ Returns the solved sudoku """
    for i in range(9):
        for j in range(9):
            # checks if the cell at (i, j) is empty
            if board[i][j] == 0:
                # try each of values 1 to 9
                for k in range(1, 10):
                    # check if k can be placed at position (i, j) in the sudoku board
                    if can_be_placed(board, i, j, k):
                        # set (i, j) to k
                        board[i][j] = k
                        # recursive call
                        solve_sudoku(board)
                        # return board if it has been solved
                        if is_solved(board):
                            return board
                        # backtrack
                        board[i][j] = 0
            # return if the cell (i, j) is empty i.e. don't solve a cell (i, j+1) or (i, 0) if (i, j) or (i-1, 8) are empty, respectively
            if board[i][j] == 0: return

if __name__ == "__main__":
    board = [[0, 0, 7, 0, 1, 0, 2, 0, 0],
            [6, 0, 0, 0, 3, 0, 4, 7, 0],
            [2, 0, 0, 9, 0, 0, 1, 0, 3],
            [5, 0, 3, 0, 2, 0, 6, 0, 0],
            [0, 9, 6, 5, 0, 3, 7, 1, 0],
            [0, 0, 8, 0, 6, 0, 9, 0, 5],
            [8, 0, 4, 0, 0, 0, 0, 0, 7],
            [0, 6, 9, 0, 5, 0, 0, 0, 4],
            [0, 0, 2, 0, 4, 0, 3, 0, 0]]
    solve_sudoku(board)
    for row in board:
        print(row)