impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut col: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut square:Vec<HashSet<char>>=vec![HashSet::new();9];

        for r in 0..9{
            for c in 0..9{
                let val = board[r][c];
                if val=='.'{
                    continue
                };
                let square_idx = (r / 3) * 3 + (c / 3);

                if !rows[r].insert(val) || !col[c].insert(val) || !square[square_idx].insert(val) {
                return false; // Found a duplicate digit!
            }

            }
        }
         // 5. Added: If no duplicates are found after checking the whole board, it's valid
    true 
    }
}
