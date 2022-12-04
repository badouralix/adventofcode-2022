const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn parse_range(str: []const u8) [2]u16 {
    var numbers = std.mem.split(u8, str, "-");
    var first = std.fmt.parseInt(u16, numbers.next().?, 10) catch unreachable;
    var second = std.fmt.parseInt(u16, numbers.next().?, 10) catch unreachable;
    return .{ first, second };
}

fn run(input: [:0]const u8) i64 {
    var lines = std.mem.tokenize(u8, input, "\n");
    var count: i64 = 0;
    while (lines.next()) |line| {
        var ranges = std.mem.split(u8, line, ",");
        var first = parse_range(ranges.next().?);
        var second = parse_range(ranges.next().?);
        if ((first[0] <= second[0] and first[1] >= second[0]) or
            (first[0] >= second[0] and first[0] <= second[1]))
        {
            count += 1;
        }
    }
    // your code here
    return count;
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

test "example input" {
    const input: [:0]const u8 =
        \\2-4,6-8
        \\2-3,4-5
        \\5-7,7-9
        \\2-8,3-7
        \\6-6,4-6
        \\2-6,4-8
    ;
    try std.testing.expectEqual(run(input), 4);
}
