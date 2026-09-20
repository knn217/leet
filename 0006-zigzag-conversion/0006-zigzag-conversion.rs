impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        let s_len = s.len();
        let num_rows_with_diag = num_rows - 2; // diag does not exist for 1st and last rows
        let mut col_gap = (num_rows + num_rows_with_diag) as usize;
        if col_gap == 0 { col_gap = 1; }
        let n_cols = (s_len as f64) / (col_gap as f64);
        let num_cols = n_cols.ceil() as usize;
        // println!("col_gap: {}, num_cols: {}", col_gap, num_cols);
        let mut string: String = String::with_capacity(s.len());
        for r in 0..num_rows {
            let start_ch_idx = r;
            for c in 0..num_cols {
                // Find the char in the col
                let idx_col = r + c * col_gap as usize;
                // println!("idx_col: {}", idx_col);
                if idx_col > s_len - 1 { break; }
                let ch_col = s.as_bytes()[idx_col] as char;
                string.push(ch_col);
                // Continue here if this is the start/end rows, which has no diag
                if r == 0 || r == (num_rows - 1) { continue; }
                // Find the char in the diag
                let diag_gap = (num_rows - 1 - r) * 2;
                // println!("diag_gap: {}, num_rows: {}, r: {}", diag_gap, num_rows, r);
                let idx_diag = idx_col + diag_gap;
                // println!("idx_diag: {}", idx_diag);
                if idx_diag > s_len - 1 { break; }
                let ch_diag = s.as_bytes()[idx_diag] as char;
                string.push(ch_diag);
            }
        }
        string
    }
}