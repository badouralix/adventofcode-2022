const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const WINDOW_SIZE = 14;
const SEEN_SIZE = 'z' - 'a' + 1;

fn run(input: [:0]const u8) u64 {
    var seen = [_]bool{false} ** (SEEN_SIZE); // Initialize window
    var i: usize = WINDOW_SIZE; // start at minimum window size
    while (i < input.len) : (i += 1) {
        //stdout.print("Currently at index {}: {c}\n", .{i, input[i]}) catch unreachable;
        // Below is comptime-unrolled for
        comptime var b = 0;
        inline while (b < WINDOW_SIZE) : (b += 1) {
            const cur = input[i - b] - 'a';
            if (seen[cur]) {
                //stdout.print("{c} already seen. Resetting\n", .{input[i-b]}) catch unreachable;
                seen = [_]bool{false} ** (SEEN_SIZE);
                break;
            } else {
                seen[cur] = true;
                if (b == 13)
                    return i + 1;
            }
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

test "ez" {
    try std.testing.expect(run("mjqjpqmgbljsphdztnvjfqwrcgsmlb") == 19);
    try std.testing.expect(run("bvwbjplbgvbhsrlpgdmjqwftvncz") == 23);
    try std.testing.expect(run("nppdvjthqldpwncqszvftbrmjlhg") == 23);
    try std.testing.expect(run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg") == 29);
    try std.testing.expect(run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw") == 26);
}
