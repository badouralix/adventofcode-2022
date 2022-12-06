const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

// A window size of 4 is low enough that it makes sense not to store the window
// and only do direct comparisons, manually
fn run(input: [:0]const u8) u64 {
    var i: usize = 3;
    while (i < input.len) : (i += 1) {
        //stdout.print("{c}{c}{c}{c}\n", .{input[i-3], input[i-2], input[i-1], input[i]}) catch unreachable;
        if (input[i] != input[i - 1] and input[i] != input[i - 2] and input[i] != input[i - 3] and input[i - 1] != input[i - 2] and input[i - 1] != input[i - 3] and input[i - 2] != input[i - 3])
            return i + 1;
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

test "ez" {
    try std.testing.expect(run("mjqjpqmgbljsphdztnvjfqwrcgsmlb") == 7);
    try std.testing.expect(run("bvwbjplbgvbhsrlpgdmjqwftvncz") == 5);
    try std.testing.expect(run("nppdvjthqldpwncqszvftbrmjlhg") == 6);
    try std.testing.expect(run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg") == 10);
    try std.testing.expect(run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw") == 11);
}
