use std::cmp;

pub struct Diff {
    left: Vec<String>,
    right: Vec<String>,
}

impl Diff {
    pub fn new(left: Vec<String>, right: Vec<String>) -> Self {
        Self { left, right }
    }

    fn print_diff(grid: &[Vec<i32>], x: &[String], y: &[String], i: usize, j: usize) {
        if i > 0 && j > 0 && x[i - 1] == y[j - 1] {
            Diff::print_diff(grid, x, y, i - 1, j - 1);
            println!("  {}", x[i - 1]);
        } else if j > 0 && (i == 0 || grid[i][j - 1] >= grid[i - 1][j]) {
            Diff::print_diff(grid, x, y, i, j - 1);
            println!("> {}", y[j - 1]);
        } else if i > 0 && (j == 0 || grid[i][j - 1] < grid[i - 1][j]) {
            Diff::print_diff(grid, x, y, i - 1, j);
            println!("< {}", x[i - 1]);
        } else {
            println!();
        }
    }

    pub fn lcs(&self) -> Vec<Vec<i32>> {
        let m: usize = self.left.len();
        let n: usize = self.right.len();

        let mut grid = vec![vec![0; n + 1]; m + 1];

        for item in grid.iter_mut().take(m + 1) {
            //grid[i][0] = 0;
            item[0] = 0;
        }

        for j in 0..(n + 1) {
            grid[0][j] = 0;
        }

        for i in 0..m {
            for (j, right_item) in self.right.iter().enumerate().take(n) {
                if self.left[i] == *right_item {
                    grid[i + 1][j + 1] = grid[i][j] + 1;
                } else {
                    grid[i + 1][j + 1] = cmp::max(grid[i + 1][j], grid[i][j + 1]);
                }
            }
        }

        grid
    }

    pub fn print(&self) {
        let grid = Diff::lcs(self);
        Diff::print_diff(
            &grid,
            &self.left,
            &self.right,
            self.left.len(),
            self.right.len(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::Diff;

    #[test]
    fn larger_can_hold_smaller() {
        Diff::new([].to_vec(), [].to_vec());
        assert!(1 == 1);
    }
}