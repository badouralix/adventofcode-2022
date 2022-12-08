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
    let mut res = 2 * (width + length) - 4;
    for j in 1..length - 1 {
        for i in 1..width - 1 {
            if is_visible(&grid, i, j, width, length) {
                res += 1;
            }
        }
    }
    res
}

fn is_visible(grid: &[u8], i: usize, j: usize, width: usize, length: usize) -> bool {
    let v = grid[j * width + i];
    let mut left = true;
    for k in 0..i {
        if grid[j * width + k] >= v {
            left = false;
            break;
        }
    }
    if left {
        return true;
    }
    let mut right = true;
    for k in i + 1..width {
        if grid[j * width + k] >= v {
            right = false;
            break;
        }
    }
    if right {
        return true;
    }
    let mut up = true;
    for k in 0..j {
        if grid[k * width + i] >= v {
            up = false;
            break;
        }
    }
    if up {
        return true;
    }
    let mut down = true;
    for k in j + 1..length {
        if grid[k * width + i] >= v {
            down = false;
            break;
        }
    }
    if down {
        return true;
    }
    false
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
            21
        )
    }
}
