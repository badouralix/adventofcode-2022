const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const window_size : usize = 14;


fn all_diff(data: [27]u8) bool {
    var idx : usize = 0;
    while (idx < 27) : (idx += 1) {
        if (data[idx] > 1) {
            return false;
        }
    }
    return true;
}

fn run(input: [:0]const u8) usize {
    var in_window : [27]u8 = .{0} ** 27;
    var idx : usize = 0;
    while (idx < window_size) : (idx += 1) {
        in_window[input[idx] - 'a'] += 1;
    }
    while (true) {
        if (!all_diff(in_window)) {
            in_window[input[idx - window_size] - 'a'] -= 1;
            in_window[input[idx] - 'a'] += 1;
            idx += 1;
            continue;
        } else {
            return idx;
        }
    }
    unreachable;
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

test "examples" {
    const input1: [:0]const u8 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const input2: [:0]const u8 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const input3: [:0]const u8 = "nppdvjthqldpwncqszvftbrmjlhg";
    try std.testing.expectEqual(@as(usize, 7), run(input1));
    try std.testing.expectEqual(@as(usize, 5), run(input2));
    try std.testing.expectEqual(@as(usize, 6), run(input3));
}
