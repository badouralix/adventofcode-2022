const std = @import("std");
const builtin = @import("builtin");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const LEN = if (builtin.is_test) 5 else 99; // input is always a square

fn get_char(input: *const [:0]const u8, x: usize, y: usize) u8 {
    return input.*[x * (LEN + 1) + y];
}

fn visible(input: *const [:0]const u8, x: usize, y: usize) u64 {
    // Get base char
    const char: u8 = get_char(input, x, y);
    // Target char placeholder
    var tree: u8 = 0;
    
    // Build util values
    var visible_left: u64 = 0;
    var visible_right: u64 = 0;
    var visible_top: u64 = 0;
    var visible_bot: u64 = 0;
    
    // iterator
    var distance: usize = 0;

    // looking left
    while (y - distance > 0) {
        distance += 1; // Have to shift logic, otherwise y - distance underflows
        tree = get_char(input, x, y - distance);
        visible_left += 1;
        if (tree >= char)
            break;
    }
    // looking right
    distance = 1;
    while (y + distance <= LEN - 1) : (distance += 1) {
        tree = get_char(input, x, y + distance);
        visible_right += 1;
        if (tree >= char)
            break;
    }
    // looking up
    distance = 0;
    while (x - distance > 0) {
        distance += 1; // Have to shift logic, otherwise x - distance underflows
        tree = get_char(input, x - distance, y);
        visible_top += 1;
        if (tree >= char)
            break;
    }
    // looking down
    distance = 1;
    while (x + distance <= LEN - 1) : (distance += 1) {
        tree = get_char(input, x + distance, y);
        visible_bot += 1;
        if (tree >= char)
            break;
    }
    return visible_left * visible_right * visible_top * visible_bot;
}

//  o--> y
//  |
//  v
//  x

fn run(input: [:0]const u8) u64 {
    var best_score: u64 = 0;

    var x: usize = 1;
    var y: usize = 1;

    while (x < LEN - 1) : (x += 1) {
        y = 1;
        while (y < LEN - 1) : (y += 1) {
            best_score = @max(visible(&input, x, y), best_score);
        }
    }
    return best_score;
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings

    defer arena.deinit(); // clear memory
    a = arena.allocator();

    var arg_it = try std.process.argsWithAllocator(a);
    _ = arg_it.skip(); // skip over exe name
    const input: [:0]const u8 = arg_it.next().?;

    const start: i128 = std.time.nanoTimestamp(); // start time
    const answer = run(input); // compute answer
    const end: i128 = std.time.nanoTimestamp();
    const elapsed_nano = @intToFloat(f64, end - start);
    const elapsed_milli = elapsed_nano / 1_000_000.0;
    try stdout.print("_duration:{d}\n{}\n", .{ elapsed_milli, answer }); // emit actual lines parsed by AOC
}

test "ez_example_1" {
    const input: [:0]const u8 =
        \\30373
        \\25512
        \\65332
        \\33549
        \\35390
    ;
    stdout.print("\n\n", .{}) catch unreachable;
    try std.testing.expect(visible(&input, 1, 2) == 4);
    try std.testing.expect(visible(&input, 3, 2) == 8);
}

test "ez" {
    try std.testing.expect(run(
        \\30373
        \\25512
        \\65332
        \\33549
        \\35390
    ) == 8);
}
