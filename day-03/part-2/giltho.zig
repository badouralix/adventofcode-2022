const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn item_priority(item: u8) u6 {
    if (item > 'Z') {
        return @intCast(u6, item - 'a' + 1);
    } else {
        return @intCast(u6, item - 'A' + 27);
    }
}

fn run(input: [:0]const u8) u64 {
    var maps = [_]u64{ undefined, undefined, undefined };
    var prio: u6 = undefined;
    var score: u64 = 0;
    var lines = std.mem.tokenize(u8, input, "\n");
    while (lines.next()) |line1| {
        var line2 = lines.next().?;
        var line3 = lines.next().?;
        maps[0] = 0;
        maps[1] = 0;
        maps[2] = 0;
        for (line1) |item| {
            prio = item_priority(item);
            maps[0] |= @as(u64, 1) << prio;
        }
        for (line2) |item| {
            prio = item_priority(item);
            maps[1] |= @as(u64, 1) << prio;
        }
        for (line3) |item| {
            prio = item_priority(item);
            maps[2] |= @as(u64, 1) << prio;
        }
        var common_item: u64 = maps[0] & maps[1] & maps[2];
        prio = 0;
        while (common_item > 1) {
            prio += 1;
            common_item >>= 1;
        }
        score += prio;
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

test "input example" {
    const input: [:0]const u8 = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
    try std.testing.expectEqual(run(input), 70);
}
