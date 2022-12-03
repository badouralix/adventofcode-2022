const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn run(input: [:0]const u8) u64 {
    var score: u64 = 0;
    var line_it = std.mem.tokenize(u8, input, "\n");
    var line_counter: u16 = 0; // usize max is too low (255)
    var seen1 = [_]bool{false} ** 60; // disregard invalid values between 'Z' and 'a', trust input
    var seen2 = [_]bool{false} ** 60;
    var seen3 = [_]bool{false} ** 60;
    while (line_it.next()) |line| : (line_counter += 1) {
        var i: usize = 0;
        const line_mod = line_counter % 3;
        var seen = switch (line_mod) {
            0 => &seen1,
            1 => &seen2,
            else => &seen3,
        };
        while (i < line.len) : (i += 1) {
            const cur = line[i] - 'A';
            seen.*[cur] = true;
            if (line_mod == 2) {
                if (seen1[cur] and seen2[cur] and seen3[cur]) {
                    score += if (cur < 32) cur + 27 else cur - 31;
                    seen1 = [_]bool{false} ** 60; // disregard invalid values between 'Z' and 'a', trust input
                    seen2 = [_]bool{false} ** 60;
                    seen3 = [_]bool{false} ** 60;
                    break;
                }
            }
        }
    }
    return score;
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

test "ez" {
    const input =
        \\vJrwpWtwJgWrhcsFMMfFFhFp
        \\jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        \\PmmdzqPrVvPwwTWBwg
        \\wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        \\ttgJtRGJQctTZtZT
        \\CrZsJsPPZsGzwwsLwLmpwMDw
    ;
    var buf = input.*;
    try stdout.print("\n", .{});
    const ans = run(&buf);
    try std.testing.expect(ans == 70);
}
