const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

inline fn add_to_top(top: *[3]i64, new_value: i64) void {
    if (new_value >= top[0]) {
        top[2] = top[1];
        top[1] = top[0];
        top[0] = new_value;
    } else if (new_value >= top[1]) {
        top[2] = top[1];
        top[1] = new_value;
    } else if (new_value >= top[2]) {
        top[2] = new_value;
    }
}

fn run(input: [:0]const u8) i64 {
    var top_three_calories = [3]i64{ 0, 0, 0 };
    var current_calories: i64 = 0;
    var iterator = std.mem.split(u8, input, "\n");
    while (iterator.next()) |token| {
        if (token.len != 0) {
            current_calories += std.fmt.parseInt(i64, token, 10) catch unreachable;
        } else {
            add_to_top(&top_three_calories, current_calories);
            current_calories = 0;
        }
    }
    add_to_top(&top_three_calories, current_calories);
    current_calories = 0;
    // your code here
    return top_three_calories[0] + top_three_calories[1] + top_three_calories[2];
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
    const elapsed_nano: f128 = @intToFloat(f128, std.time.nanoTimestamp() - start);
    const elapsed_milli: f64 = @floatCast(f64, @divFloor(elapsed_nano, 1_000_000));
    try stdout.print("_duration:{d}\n{}\n", .{ elapsed_milli, answer }); // emit actual lines parsed by AOC
}
