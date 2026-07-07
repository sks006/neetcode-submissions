pub struct NumMatrix {
    prefix: Vec<i32>,
    stride: usize,
}

impl NumMatrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        // [INITIALIZATION BOUNDARY]
        // 1. Map dimensions and stride.
        let row= matrix.len();
        if rows == 0 || matrix[0].is_empty() {
            return Self { prefix: vec![0], stride: 1 };
        }
        let cols = matrix[0].len();
        let stride = cols + 1;
        // 2. Allocate the flattened prefix vector.
        let mut prefix= vec![0;(row+1)*stride];
        // [MUTATION BOUNDARY]
        // 3. Execute nested `for` loops.
          for r in 0..rows {
            for c in 0..cals{
                // 4. Calculate the four 1D offsets.
                let curr_out_idx = (r+1)* stride+(c+1);
                let top_out_idx  = r * stride + (c + 1);
                let left_out_idx = (r + 1) * stride + c;
                let diag_out_idx  = r * stride + c;
                   // 5. Apply inclusion-exclusion and write to memory.
        prefix[curr_out_idx]=matrix[r][c] 
                    + prefix[top_out_idx] 
                    + prefix[left_out_idx] 
                    - prefix[diag_out_idx];
            }
          }
        
     
        
        // [RETURN BOUNDARY]
         Self { prefix, stride }
    }
 // [READ-ONLY VERIFICATION BOUNDARY]
        // 1. Cast and shift inputs to usize.
        let r1 = row1 as usize;
        let c1 = col1 as usize;
        let r2 = (row2 + 1) as usize;
        let c2 = (col2 + 1) as usize;

        // 2. Compute the four 1D corner offsets using the stride.
        let bottom_right_idx = r2 * self.stride + c2;
        let top_right_idx    = r1 * self.stride + c2;
        let bottom_left_idx  = r2 * self.stride + c1;
        let top_left_idx     = r1 * self.stride + c1;

        // 3. Execute raw memory reads and return the integer.
        self.prefix[bottom_right_idx] 
            - self.prefix[top_right_idx] 
            - self.prefix[bottom_left_idx] 
            + self.prefix[top_left_idx]
}