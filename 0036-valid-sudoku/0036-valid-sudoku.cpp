class Solution {
public:
    bool isValidSudoku(std::vector<std::vector<char>>& board) {
        // Track seen numbers for rows, columns, and 3x3 boxes
        // index 0 to 8 for grid position, index 0 to 8 for digit (1-9 mapped to 0-8)
        bool rows[9][9] = {false};
        bool cols[9][9] = {false};
        bool boxes[9][9] = {false};

        for (int r = 0; r < 9; ++r) {
            for (int c = 0; c < 9; ++c) {
                char current = board[r][c];

                // Skip empty cells
                if (current == '.') {
                    continue;
                }

                int num = current - '1'; // Map '1'-'9' to index 0-8
                int boxIndex = (r / 3) * 3 + (c / 3);

                // Check for duplicates
                if (rows[r][num] || cols[c][num] || boxes[boxIndex][num]) {
                    return false;
                }

                // Mark the number as seen
                rows[r][num] = true;
                cols[c][num] = true;
                boxes[boxIndex][num] = true;
            }
        }

        return true;
    }
};