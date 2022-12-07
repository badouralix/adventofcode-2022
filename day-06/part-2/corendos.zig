const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const group_len = 14;

fn run(input: [:0]const u8) i64 {
    const zeroes: @Vector(group_len, u8) = [_]u8{0} ** group_len;
    const ones: @Vector(group_len, u8) = [_]u8{1} ** group_len;

    var i: usize = 0;
    while (i < input.len - group_len) : (i += 1) {
        const array = @ptrCast(*const [group_len]u8, input[i .. i + group_len]);
        const input_vector: @Vector(group_len, u8) = array.*;
        var output_vector: @Vector(group_len, u8) = [_]u8{0} ** group_len;
        comptime var j: usize = 0;
        inline while (j < group_len) : (j += 1) {
            const mask = @splat(group_len, input[i + j]);
            const select = @select(u8, input_vector == mask, ones, zeroes);
            output_vector += select;
        }
        const count = @reduce(.Add, output_vector);
        if (count == group_len) {
            return @intCast(i64, i + group_len);
        }
    }
    return -1;
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

test {
    const input1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const input2 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const input3 = "nppdvjthqldpwncqszvftbrmjlhg";
    const input4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const input5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    try std.testing.expectEqual(@as(i64, 19), run(input1));
    try std.testing.expectEqual(@as(i64, 23), run(input2));
    try std.testing.expectEqual(@as(i64, 23), run(input3));
    try std.testing.expectEqual(@as(i64, 29), run(input4));
    try std.testing.expectEqual(@as(i64, 26), run(input5));
}
