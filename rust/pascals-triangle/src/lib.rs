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
            let next = next_row(&row[i - 1]);
            row.push(next);
        }
        row
    }
}

pub fn next_row(prev: &[u32]) -> Vec<u32> {
    let mut row = vec![1];

    for i in 1..prev.len() {
        row.push(prev[i - 1] + prev[i]);
    }

    row.push(1);
    row
}
