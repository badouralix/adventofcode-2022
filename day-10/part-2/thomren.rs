use std::env::args;
use std::time::Instant;

const CRT_WIDTH: usize = 40;
const CRT_HEIGHT: usize = 6;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("_parse");
    println!("{}", output);
}

fn run(input: &str) -> String {
    let mut crt = [false; CRT_HEIGHT * CRT_WIDTH];
    let mut reg = 1;
    let mut cycle = 0;
    draw_pixel(&mut crt, cycle, reg);
    for line in input.trim().lines() {
        cycle += 1;
        draw_pixel(&mut crt, cycle, reg);
        if line.starts_with("addx") {
            let (_, value) = line.split_once(' ').unwrap();
            let value = value.parse::<isize>().unwrap();
            reg += value;
            cycle += 1;
            draw_pixel(&mut crt, cycle, reg);
        }
    }
    print_crt(&crt)
}

fn draw_pixel(crt: &mut [bool], cycle: usize, reg: isize) {
    let x = cycle % CRT_WIDTH;
    if reg.abs_diff(x as isize) <= 1 && cycle < crt.len() {
        crt[cycle] = true;
    }
}

fn print_crt(crt: &[bool]) -> String {
    let mut bytes = [b'.'; (CRT_WIDTH + 1) * CRT_HEIGHT];
    for i in 0..CRT_HEIGHT {
        for j in 0..CRT_WIDTH {
            if crt[(i * CRT_WIDTH) + j] {
                bytes[(i * (1 + CRT_WIDTH)) + j] = b'#';
            }
        }
        bytes[i * (1 + CRT_WIDTH) + CRT_WIDTH] = b'\n';
    }

    std::str::from_utf8(&bytes[..bytes.len() - 1])
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
                .trim()),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        )
    }
}
