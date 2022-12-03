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
    var hashmap : u64 = 0;
    var score : u64 = 0;
    var lines = std.mem.tokenize(u8, input, "\n");
    var prio: u6 = undefined;
    while (lines.next()) |line| {
        hashmap = 0;
        var idx: usize = 0;
        var comp_length = line.len / 2;
        while (idx < comp_length) {
            prio = item_priority(line[idx]);
            hashmap |= (@as(u64, 1) << prio);
            idx += 1;
        }
        while(idx < line.len) {
            prio = item_priority(line[idx]);
            if (hashmap & (@as(u64, 1) << prio) != 0) {
                score += @as(u64, prio);
                break;
            }
            idx += 1;
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


test "input example" {
    const input: [:0] const u8 = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
    try std.testing.expectEqual(run(input), 157);
}