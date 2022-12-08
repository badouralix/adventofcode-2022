fn main() {
    aoc::run(run)
}

fn run(input: &str) -> usize {
    // Your code goes here
    let width = input.find('\n').unwrap();
    let mut grid = Vec::with_capacity(input.len());
    for c in input.as_bytes() {
        if *c != b'\n' {
            grid.push(*c);
        }
    }
    let length = grid.len() / width;
    let mut max_score = 0;
    for j in 1..length - 1 {
        for i in 1..width - 1 {
            max_score = max_score.max(score(&grid, i, j, width, length))
        }
    }
    max_score
}

fn score(grid: &[u8], i: usize, j: usize, width: usize, length: usize) -> usize {
    let v = grid[j * width + i];
    let mut left = 0;
    for k in (0..i).rev() {
        left += 1;
        if grid[j * width + k] >= v {
            break;
        }
    }
    let mut right = 0;
    for k in i + 1..width {
        right += 1;
        if grid[j * width + k] >= v {
            break;
        }
    }
    let mut up = 0;
    for k in (0..j).rev() {
        up += 1;
        if grid[k * width + i] >= v {
            break;
        }
    }
    let mut down = 0;
    for k in j + 1..length {
        down += 1;
        if grid[k * width + i] >= v {
            break;
        }
    }
    left * right * up * down
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("
30373
25512
65332
33549
35390"),
            8
        )
    }
}
