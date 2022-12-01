fn main() {
    aoc::run(run)
}

fn run(input: &str) -> usize {
    let mut maxs = [0; 3];
    let mut sum = 0;
    for line in input.split('\n') {
        if line.is_empty() {
            insert_max(&mut maxs, sum);
            sum = 0;
        } else {
            sum += line.parse::<usize>().unwrap();
        }
    }
    insert_max(&mut maxs, sum);

    maxs.iter().sum()
}

#[inline]
fn insert_max<const N: usize>(maxs: &mut [usize; N], mut v: usize) {
    for max in maxs {
        let tmp = (*max).min(v);
        *max = (*max).max(v);
        v = tmp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"),
            45000
        )
    }
}
