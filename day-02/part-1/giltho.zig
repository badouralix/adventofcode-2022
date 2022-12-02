const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn diff_score(me: u8, them: u8) u8 {
    if (me == them) {
        return 3;
    } else if ((me + 1) % 3 == them) {
        return 0;
    } else {
        return 6;
    }
}

fn run(input: [:0]const u8) u64 {
    var score : u64 = 0;
    var cursor : usize = 0;
    while (cursor < input.len) {
        var them = input[cursor] - 'A';
        var me = input[cursor + 2] - 'X';
        cursor += 4;
        score += @intCast(u64, me + 1);
        score += diff_score(me, them);
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


test "advent example" {
    var input: [:0]const u8 = "A Y\nB X\nC Z";
    try std.testing.expectEqual(run(input), 15);
}