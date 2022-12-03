const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

inline fn toInteger(c: u8) usize {
    if (c >= 'a' and c <= 'z') {
        return c - 'a';
    } else if (c >= 'A' and c <= 'Z') {
        return c - 'A' + 26;
    }
    unreachable;
}

fn run(input: [:0]const u8) u64 {
    var total_score: u64 = 0;
    var it = std.mem.split(u8, input, "\n");
    while (it.next()) |line| {
        var seen = std.bit_set.IntegerBitSet(52).initEmpty();
        for (line[0 .. line.len / 2]) |c| {
            seen.set(toInteger(c));
        }

        total_score += for (line[line.len / 2 ..]) |c| {
            const index = toInteger(c);
            if (seen.isSet(index)) {
                break index + 1;
            }
        } else unreachable;
    }
    return total_score;
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
    const input =
        \\vJrwpWtwJgWrhcsFMMfFFhFp
        \\jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        \\PmmdzqPrVvPwwTWBwg
        \\wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        \\ttgJtRGJQctTZtZT
        \\CrZsJsPPZsGzwwsLwLmpwMDw
    ;

    const result = run(input);
    try std.testing.expectEqual(@as(u64, 157), result);
}
