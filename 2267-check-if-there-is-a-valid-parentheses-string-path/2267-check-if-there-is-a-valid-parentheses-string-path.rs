impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        let rows = grid.len();
        let cols = grid[0].len();
        
        // A valid path length is rows + cols - 1. 
        // If the path length is odd, it's impossible to balance parentheses.
        if (rows + cols - 1) % 2 != 0 {
            return false;
        }
        
        // Start and end positions must be '(' and ')' respectively
        if grid[0][0] != '(' || grid[rows - 1][cols - 1] != ')' {
            return false;
        }

        // Maximum possible open parentheses balance is (rows + cols) / 2
        let max_balance = (rows + cols) / 2;
        let mut visited = vec![vec![vec![false; max_balance + 1]; cols]; rows];

        Self::dfs(&grid, 0, 0, 0, &mut visited)
    }

    fn dfs(
        grid: &Vec<Vec<char>>, 
        r: usize, 
        c: usize, 
        mut balance: usize, 
        visited: &mut Vec<Vec<Vec<bool>>>
    ) -> bool {
        let rows = grid.len();
        let cols = grid[0].len();

        // Update balance
        if grid[r][c] == '(' {
            balance += 1;
        } else {
            if balance == 0 {
                return false; // Cannot close an unopened bracket
            }
            balance -= 1;
        }

        // If balance exceeds maximum remaining steps needed, it can never reach 0
        let max_balance = (rows + cols) / 2;
        if balance > max_balance {
            return false;
        }

        // Reached bottom-right corner
        if r == rows - 1 && c == cols - 1 {
            return balance == 0;
        }

        // Prune if already visited with the same balance
        if visited[r][c][balance] {
            return false;
        }
        visited[r][c][balance] = true;

        // Move Right
        if c + 1 < cols && Self::dfs(grid, r, c + 1, balance, visited) {
            return true;
        }

        // Move Down
        if r + 1 < rows && Self::dfs(grid, r + 1, c, balance, visited) {
            return true;
        }

        false
    }
}