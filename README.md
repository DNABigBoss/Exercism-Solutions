# How i solve a problem and i choose Pascals triangle
Solutions exercise from exercism (rust language)

# The Problem
Compute Pascal's triangle up to a given number of rows.

In Pascal's Triangle each number is computed by adding the numbers to the right and left of the current position in the previous row.

Result from first line to n-lines will be come a vector and the result line will be add to a larger vector. we must return this larger vector to solve this problem.

# My way to solved problem :
First, Make struct will be soon call by new function in implementation. This struct only fill with row_count (count of row) where have a usize type data.

Second, Make function rows to return result we want.

Then, if row_count isn't zero, i will add the process with add a function called next_row for add a line and entry that line to vector.

Last, return vector in vector.

# My Code
```rust
pub struct PascalsTriangle{
    row_count: usize,
}

impl PascalsTriangle {
    pub fn new(row_count: usize) -> Self {
        PascalsTriangle { 
            row_count 
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        if self.row_count == 0 {
            return Vec::new();
        }
        // Create first row
        let mut row = vec![vec![1]];
        // Create next row
        for i in 1..self.row_count {
            let next = next_row(&row[i-1]);
            row.push(next);
        }
        row
    }
}

pub fn next_row(prev: &[u32]) -> Vec<u32> {
    let mut row = vec![1];
    
    // from previous len a left add by right
    for i in 1..prev.len() {
        row.push(prev[i-1] + prev[i]);
    }

    row.push(1);
    row
}
```


link for exercism : https://exercism.io/
